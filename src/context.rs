extern crate hyper;

use presto;

pub const DEFAULT_PAGER: &'static str = "less --no-init -n --chop-long-lines --quit-if-one-screen";

#[derive(Debug, StructOpt)]
#[structopt(name = "basic")]
pub struct Context {
    /// The Presto server to connect to
    #[structopt(short = "s", long = "server", parse(from_str = "parse_server_url"))]
    pub server: String,

    /// The Presto catalog to use
    #[structopt(short = "c", long = "catalog")]
    pub catalog: String,

    /// The Presto schema to use
    #[structopt(long = "schema")]
    pub schema: String,

    /// The Presto username to use
    #[structopt(short = "u", long = "user", default_value = "veloce")]
    pub user: String,

    /// The Pager to use to show results
    #[structopt(short = "p", long = "pager", env = "PAGER", raw(default_value = "DEFAULT_PAGER"))]
    pub pager: String, // TODO: take from env
}

impl Context {
    pub fn presto_headers(&self) -> hyper::Headers {
        let mut headers = hyper::Headers::new();
        headers.set(presto::XPrestoCatalog(self.catalog.clone()));
        headers.set(presto::XPrestoSchema(self.schema.clone()));
        headers.set(presto::XPrestoSource(self.user.clone()));
        headers.set(presto::XPrestoUser(self.user.clone()));

        headers
    }
}

pub fn parse_server_url(src: &str) -> String {
    let src = {
        if !src.starts_with("http") {
            "http://".to_string() + src
        } else {
            src.to_string()
        }
    };

    src.trim_right_matches('/').to_string()
}
