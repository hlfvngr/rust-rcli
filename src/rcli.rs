use std::path::Path;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about,long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Csv(CsvCfg),
}

#[derive(Parser, Debug)]
pub struct CsvCfg {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,

    #[arg(short, long, default_value = "output.json")]
    pub output: String,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if Path::new(&filename).exists() {
        Ok(filename.into())
    } else {
        Err("file not exist".into())
    }
}
