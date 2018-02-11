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

    std::io::stdout().write(b"> ").expect("cannot write output");
    std::io::stdout().flush().expect("cannot flush");

    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("cannot read input");

    // remove trailing ; like the official cli
    line = line.trim_right_matches(|c| c == ';' || char::is_whitespace(c))
        .to_string();

    let resp = presto::start_presto_query(&cli, &ctx, line);
    println!("{:?}", resp);

    match resp {
        Ok(mut presto_res) => while let Some(next_uri) = presto_res.next_uri {
            let resp = presto::follow_presto_query(&cli, &ctx, &next_uri);
            println!("{:?}", resp);

            match resp {
                Ok(res) => {
                    presto_res = res;

                    if let (Some(cols), Some(data)) = (presto_res.columns, presto_res.data) {
                        for col in cols {
                            print!("{} ", col.name);
                        }
                        println!();

                        for row in data {
                            for cell in row {
                                println!("{}", cell.to_string());
                            }
                        }
                    }
                }
                _ => {
                    break;
                }
            }
        },
        e => println!("{:?}", e),
    }
}
