use clap::Parser;
use rcli::{read_csv, write_json, Commands, Rcli};

fn main() -> anyhow::Result<()> {
    let csv = Rcli::parse().command;

    match csv {
        Commands::Csv(opts) => {
            let data = read_csv(opts.input)?;
            let file_path = opts.output;
            write_json(data, file_path)?;
        }
    };

    Ok(())
}
