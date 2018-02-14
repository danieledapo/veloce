#[macro_use]
extern crate hyper;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate structopt;

extern crate prettytable;
extern crate reqwest;
extern crate rustyline;
extern crate serde_json;

use rustyline::Editor;
use structopt::StructOpt;

mod context;
mod presto;

use context::Context;

fn main() {
    let ctx = Context::from_args();
    let cli = reqwest::Client::new();

    let mut editor = Editor::<()>::new();

    while let Ok(query) = editor.readline(">> ") {
        let query = sanitize_query(&query).to_string();
        run_query(&cli, &ctx, query.to_string());

        editor.add_history_entry(&query);
    }
}

fn sanitize_query(query: &str) -> &str {
    // remove trailing ; like the official cli
    query.trim_right_matches(|c| c == ';' || char::is_whitespace(c))
}

fn run_query(cli: &reqwest::Client, ctx: &Context, query: String) {
    let qit = presto::QueryIterator::new(&cli, &ctx, query);
    let mut titles_set = false;

    let mut table = prettytable::Table::new();

    for res in qit {
        let res = res.expect("presto api error");

        if !titles_set {
            if let Some(cols) = res.columns {
                table.set_titles(cols.iter().map(|c| &c.name).collect());
                titles_set = true;
            }
        }

        if let Some(rows) = res.data {
            for row in rows {
                table.add_row(
                    row.iter()
                        .map(|c| serde_json::to_string(c).unwrap())
                        .collect(),
                );
            }
        }
    }

    table.printstd();
}
