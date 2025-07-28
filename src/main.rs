use std::fs;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;
use core::format::Format;
use termcolor::{ColorChoice, StandardStream};

use crate::parser::Parser;

mod core;
mod parser;

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let input_file_path = "example.rscript";

    let source = fs::read_to_string(input_file_path)?;
    info!("Successfully read input file: {}", input_file_path);

    println!("---SOURCE---");
    print!("{}", source);
    println!("---ENDING---");

    let parse_start = std::time::Instant::now();
    let program = Parser::new(&source).parse()?;
    let parse_duration = parse_start.elapsed();

    let print_start = std::time::Instant::now();
    println!("---PARSED---");
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    program.format(&mut stdout, 4, 0)?;
    println!("---ENDING---");
    let print_duration = print_start.elapsed();

    info!("Time taken for parsing: {:?}", parse_duration);
    info!("Time taken for printing: {:?}", print_duration);

    Ok(())
}
