use std::path::Path;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about,long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Csv(CsvCfg),
    Base64(Base64Cfg),
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

#[derive(Parser, Debug)]
pub struct Base64Cfg {
    #[command(flatten)]
    pub codec: Codec,

    #[arg(short, long)]
    pub input: String,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct Codec {
    /// 加密
    #[arg(short, long)]
    pub encrypt: bool,

    /// 解密
    #[arg(short, long)]
    pub decrypt: bool,
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if Path::new(&filename).exists() {
        Ok(filename.into())
    } else {
        Err("file not exist".into())
    }
}
