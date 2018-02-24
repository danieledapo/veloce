#[macro_use]
extern crate clap;

#[macro_use]
extern crate structopt;

use std::env;
use structopt::StructOpt;

use clap::Shell;

include!("src/context.rs");

fn main() {
    // OUT_DIR is set by Cargo and it's where any additional build artifacts
    // are written.
    let outdir = env::var_os("OUT_DIR").unwrap();

    // Use clap to build completion files.
    let mut app = Context::clap();
    app.gen_completions("veloce", Shell::Bash, &outdir);
    app.gen_completions("veloce", Shell::Fish, &outdir);
    app.gen_completions("veloce", Shell::PowerShell, &outdir);
    app.gen_completions("veloce", Shell::Zsh, &outdir);
}
