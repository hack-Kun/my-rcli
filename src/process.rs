use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(name = "rcli", version, author ,about, long_about = None)]
pub struct Rcli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = file_not_exits)]
    pub input: String,
    #[arg(short, long, default_value = "output.json")]
    pub output: String,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn file_not_exits(file_path: &str) -> Result<String, &'static str> {
    if std::path::Path::new(file_path).exists() {
        return Ok(file_path.to_string());
    }
    Err("File not found")
}
