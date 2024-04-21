use base64::{engine::general_purpose::STANDARD, Engine as _};
use clap::Parser;
use rcli::csv::*;
use rcli::rcli::*;

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        Command::Csv(csv_cfg) => convert_csv(&csv_cfg.input, &csv_cfg.output)?,
        Command::Base64(base64_cfg) => {
            if base64_cfg.codec.decrypt {
                let a = STANDARD.decode(base64_cfg.input)?;
                println!("{}", String::from_utf8(a)?);
            } else if base64_cfg.codec.encrypt {
                let a = STANDARD.encode(base64_cfg.input);
                println!("{}", a)
            }
        }
    }
    Ok(())
}
