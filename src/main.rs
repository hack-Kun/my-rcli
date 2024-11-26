use clap::Parser;
use rcli::{parse_csv_and_out_json, Commands, Csv};

fn main() -> anyhow::Result<()> {
    let cli = Csv::parse();
    match cli.cmd {
        Commands::Csv(opts) => {
            let input_path = opts.input();
            let output_path = opts.output();
            parse_csv_and_out_json(&input_path, &output_path)?;
        }
    }
    Ok(())
}
