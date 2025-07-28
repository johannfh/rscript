#![allow(unused)]

use std::{env::args, fs};
extern crate pretty_env_logger;
#[macro_use]
extern crate log;
use core::format::Format;
use termcolor::{ColorChoice, StandardStream};

use crate::{parser::Parser, runtime::Runtime};

mod core;
mod parser;
mod runtime;

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let input_file_path = args()
        .skip(1)
        .next()
        .ok_or(anyhow::anyhow!("No input file provided"))?;

    let source = fs::read_to_string(&input_file_path)?;
    info!("Successfully read input file: {}", input_file_path);

    println!("---SOURCE---");
    print!("{}", source);
    println!("---ENDING---");

    let mut runtime = Runtime::new();
    info!("Created a new runtime instance");

    runtime.execute(&source);

    dbg!(runtime);

    Ok(())
}
