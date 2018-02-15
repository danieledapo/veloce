#![deny(warnings)]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
// as of now clippy doesn't allow unwrap_or(vec![]) because of a bug
#![cfg_attr(feature = "clippy", allow(or_fun_call))]

#[macro_use]
extern crate failure;

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

use std::error::Error;
use std::process::{ChildStdin, Command, ExitStatus, Stdio};

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

        if query.is_empty() {
            continue;
        }

        run_query(&cli, &ctx, query.to_string());

        editor.add_history_entry(&query);
    }
}

fn sanitize_query(query: &str) -> &str {
    // remove trailing ; like the official cli
    query.trim_right_matches(';').trim()
}

fn run_query(cli: &reqwest::Client, ctx: &Context, query: String) {
    let qit = presto::QueryIterator::new(cli, ctx, query);
    let res: presto::Result<Vec<presto::QueryResults>> = qit.collect();

    match res {
        Ok(data) => {
            display_data(ctx, data);
        }
        Err(e) => println!("presto api error: {:?}", e),
    }
}

fn display_data(ctx: &Context, data: Vec<presto::QueryResults>) {
    let mut table = prettytable::Table::new();

    for qres in data {
        if let Some(cols) = qres.columns {
            table.set_titles(cols.iter().map(|c| &c.name).collect());
        }

        for row in qres.data.unwrap_or(vec![]) {
            table.add_row(row.iter().map(|c| c.to_string()).collect());
        }
    }

    let res = with_pager(ctx, |p| {
        let res = table.print(p);
        match res {
            Err(ref e) if e.kind() != std::io::ErrorKind::BrokenPipe => {
                println!("pager error: {}", e.description())
            }
            _ => (),
        }
    });

    if let Err(e) = res {
        println!("pager error: {}", e.description());
    }
}

fn with_pager<F>(ctx: &Context, f: F) -> Result<ExitStatus, std::io::Error>
where
    F: FnOnce(&mut ChildStdin) -> (),
{
    let mut pager = Command::new(&ctx.pager).stdin(Stdio::piped()).spawn()?;

    {
        let stdin = pager.stdin.as_mut().expect("cannot happen");
        f(stdin);
    }

    pager.wait()
}
