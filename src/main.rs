#![allow(warnings, dead_code)]

use clap::Parser;
use redex::{cli::CLIArgs, interpreter};

fn main() {
    let args = CLIArgs::parse();

    interpreter::interpret(
        r#"
M := ab
Lx.xyM"#,
        &args,
    );
}
