#![allow(warnings, dead_code)]

use clap::Parser;
use redex::{cli::CLIArgs, interpreter, stdlib::numerals::{NumeralKind, generate_numeral}};

fn main() {
    let args = CLIArgs::parse();

    // dbg!(generate_numeral(43, NumeralKind::Church).to_string());

    interpreter::interpret(
        r#"1PLUS2"#,
        &args,
    );
}
