#![deny(warnings)]

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
use std::env;
use std::path;
use std::process::{ChildStdin, Command, ExitStatus, Stdio};

use rustyline::Editor;
use structopt::StructOpt;

mod context;
mod presto;

use context::Context;

const VELOCE_BANNER: &'static str = r##"
           _
__   _____| | ___   ___ ___
\ \ / / _ \ |/ _ \ / __/ _ \
 \ V /  __/ | (_) | (_|  __/
  \_/ \___|_|\___/ \___\___|
"##;

fn main() {
    let ctx = Context::from_args();
    let cli = reqwest::Client::new();

    let base_dir = env::home_dir().unwrap_or_else(path::PathBuf::new);
    let history_file_path = base_dir.join("veloce.history");

    let mut editor = Editor::<()>::new();

    if editor.load_history(&history_file_path).is_err() {
        println!("cannot load history")
    }

    println!("{}", VELOCE_BANNER.trim_left_matches('\n'));

    let mut query = String::new();
    let mut prompt = ">> ";

    while let Ok(line) = editor.readline(&prompt) {
        query += " ";
        query += &line;

        if !query.ends_with(';') {
            prompt = "...> ";
            continue;
        }

        if query.is_empty() {
            continue;
        }

        run_query(&cli, &ctx, sanitize_query(&query).to_string());

        editor.add_history_entry(&query);
        query.clear();

        prompt = ">> ";
    }

    editor
        .save_history(&history_file_path)
        .expect("cannot write history file");
}

fn sanitize_query(query: &str) -> &str {
    // remove trailing ; to make the query work
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

        for row in qres.data.unwrap_or_else(Vec::new) {
            table.add_row(row.iter().map(|c| c.to_string()).collect());
        }
    }

    if table.is_empty() {
        println!("(0 rows)\n");
        return;
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
    let (cmd, args) = {
        let mut parts = ctx.pager.split(' ');
        let cmd = parts.next().unwrap_or("less");
        let args: Vec<&str> = parts.collect();

        (cmd, args)
    };

    let mut pager = Command::new(&cmd)
        .args(&args)
        .stdin(Stdio::piped())
        .spawn()?;

    {
        let stdin = pager.stdin.as_mut().expect("cannot happen");
        f(stdin);
    }

    pager.wait()
}
