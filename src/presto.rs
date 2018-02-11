extern crate reqwest;

extern crate serde;
extern crate serde_json;

use context::Context;

header! { (XPrestoCatalog, "X-Presto-Catalog") => [String] }
header! { (XPrestoSchema, "X-Presto-Schema") => [String] }
header! { (XPrestoSource, "X-Presto-Source") => [String] }
header! { (XPrestoUser, "X-Presto-User") => [String] }

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResults {
    pub id: String,

    #[serde(rename = "infoUri")]
    pub info_uri: String,

    #[serde(rename = "nextUri")]
    pub next_uri: Option<String>,

    pub columns: Option<Vec<Column>>,
    pub data: Option<Vec<Vec<Object>>>,

    pub error: Option<serde_json::Value>,
    // pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
}

// can expect everything, even though I don't know if lists/objects are actually
// ever returned
pub type Object = serde_json::Value;

pub fn start_presto_query(
    client: &reqwest::Client,
    ctx: &Context,
    query: String,
) -> reqwest::Result<QueryResults> {
    let v1stat = ctx.server.clone() + "/v1/statement";

    println!("{}", query);
    client
        .post(&v1stat)
        .headers(ctx.presto_headers())
        .body(query.into_bytes())
        .send()?
        .json()

    // TODO: handle errors
}

pub fn follow_presto_query(
    client: &reqwest::Client,
    ctx: &Context,
    next_uri: &str,
) -> reqwest::Result<QueryResults> {
    client
        .get(next_uri)
        .headers(ctx.presto_headers())
        .send()?
        .json()
}
