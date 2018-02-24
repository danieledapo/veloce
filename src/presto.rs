extern crate failure;
extern crate hyper;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::convert;
use std::result;

use context::Context;

header! { (XPrestoCatalog, "X-Presto-Catalog") => [String] }
header! { (XPrestoSchema, "X-Presto-Schema") => [String] }
header! { (XPrestoSource, "X-Presto-Source") => [String] }
header! { (XPrestoUser, "X-Presto-User") => [String] }

pub type Result<T> = result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Reqwest error: {}", _0)]
    ReqwestError(#[cause] reqwest::Error),

    #[fail(display = "Presto error: {}", _0)]
    PrestoError(String),
}

impl convert::From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        Error::ReqwestError(e)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResults {
    pub id: String,

    #[serde(rename = "infoUri")]
    pub info_uri: String,

    #[serde(rename = "nextUri")]
    pub next_uri: Option<String>,

    pub columns: Option<Vec<Column>>,
    pub data: Option<Vec<Vec<Object>>>,

    error: Option<serde_json::Value>,
    // pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
}

// can expect everything, even though I don't know if lists/objects are actually
// ever returned
pub type Object = serde_json::Value;

#[derive(Debug)]
pub struct QueryIterator<'a> {
    pub client: &'a reqwest::Client,
    pub ctx: &'a Context,
    pub state: QueryIteratorState,
}

#[derive(Clone, Debug)]
pub enum QueryIteratorState {
    Query(String),
    Fetch(Option<String>),
}

impl<'a> QueryIterator<'a> {
    pub fn new(client: &'a reqwest::Client, ctx: &'a Context, query: String) -> QueryIterator<'a> {
        QueryIterator {
            client,
            ctx,
            state: QueryIteratorState::Query(query),
        }
    }
}

impl<'a> Iterator for QueryIterator<'a> {
    type Item = Result<QueryResults>;

    fn next(&mut self) -> Option<Self::Item> {
        let resp = match self.state {
            QueryIteratorState::Query(ref query) => {
                start_presto_query(self.client, self.ctx, query.clone())
            }
            QueryIteratorState::Fetch(None) => return None,
            QueryIteratorState::Fetch(Some(ref next_uri)) => {
                follow_presto_query(self.client, self.ctx, &next_uri.clone())
            }
        };

        match resp {
            Ok(res) => {
                self.state = QueryIteratorState::Fetch(res.next_uri.clone());
                Some(Ok(res))
            }
            Err(err) => Some(Err(err)),
        }
    }
}

pub fn start_presto_query(
    client: &reqwest::Client,
    ctx: &Context,
    query: String,
) -> Result<QueryResults> {
    let v1stat = ctx.server.clone() + "/v1/statement";

    let resp: QueryResults = client
        .post(&v1stat)
        .headers(presto_headers(&ctx))
        .body(query.into_bytes())
        .send()?
        .json()?;

    check_errors(resp)
}

pub fn follow_presto_query(
    client: &reqwest::Client,
    ctx: &Context,
    next_uri: &str,
) -> Result<QueryResults> {
    let resp = client
        .get(next_uri)
        .headers(presto_headers(&ctx))
        .send()?
        .json()?;

    check_errors(resp)
}

pub fn presto_headers(ctx: &Context) -> hyper::Headers {
    let mut headers = hyper::Headers::new();
    headers.set(XPrestoCatalog(ctx.catalog.clone()));
    headers.set(XPrestoSchema(ctx.schema.clone()));
    headers.set(XPrestoSource(ctx.user.clone()));
    headers.set(XPrestoUser(ctx.user.clone()));

    headers
}

fn check_errors(resp: QueryResults) -> Result<QueryResults> {
    if let Some(err) = resp.error {
        return Err(Error::PrestoError(err.to_string()));
    }

    Ok(resp)
}
