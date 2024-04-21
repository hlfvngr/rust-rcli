use clap::Parser;
use rcli::csv::*;
use rcli::rcli::*;

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        Command::Csv(csv_cfg) => convert_csv(&csv_cfg.input, &csv_cfg.output)?,
    }
    Ok(())
}
