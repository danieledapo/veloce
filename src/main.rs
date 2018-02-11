#[macro_use]
extern crate hyper;

#[macro_use]
extern crate serde_derive;

extern crate reqwest;

#[macro_use]
extern crate structopt;

use std::io::Write;
use structopt::StructOpt;

mod context;
mod presto;

use context::Context;

fn main() {
    let ctx = Context::from_args();
    let cli = reqwest::Client::new();

    std::io::stdout()
        .write_all(b"> ")
        .expect("cannot write output");
    std::io::stdout().flush().expect("cannot flush");

    let mut query = String::new();
    std::io::stdin()
        .read_line(&mut query)
        .expect("cannot read input");

    // remove trailing ; like the official cli
    query = query
        .trim_right_matches(|c| c == ';' || char::is_whitespace(c))
        .to_string();

    let qit = presto::QueryIterator::new(&cli, &ctx, query);
    let mut print_cols = true;

    for res in qit {
        let res = res.expect("presto api error");
        if print_cols {
            if let Some(cols) = res.columns {
                for col in cols {
                    print!("{}", col.name);
                }
                println!();
                print_cols = false;
            }
        }

        if let Some(rows) = res.data {
            assert_eq!(print_cols, false);
            for row in rows {
                println!("{:?}", row);
            }
        }
    }
}
