use clap::{Parser, Subcommand};

use crate::verify_file;
// 重新设计rust cli工具
// 实现功能 cargo run -- csv -i input.csv -o output.json

// 这个mod 主要功能是为了实现clap的参数解析
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Csv {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Csv(Opts),
}

// 公开struct但不可见成员
#[derive(Debug, Parser)]
pub struct Opts {
    #[arg(short, long, value_parser = verify_file)]
    input: String,
    #[arg(short, long, default_value = "output.json")]
    output: String,
    #[arg(short, long, default_value_t = ',')]
    delimiter: char,
    #[arg(long, default_value_t = false)]
    header: bool,
}

// 提供获取参数的方法
impl Opts {
    // pub fn to_string(&self) -> String {
    //     format!(
    //         "input: {}, output: {}, delimiter: {}, header: {}",
    //         self.input, self.output, self.delimiter, self.header
    //     )
    // }
    pub fn input(&self) -> String {
        self.input.clone()
    }

    pub fn output(&self) -> String {
        self.output.clone()
    }
    pub fn delimiter(&self) -> char {
        self.delimiter
    }
    pub fn header(&self) -> bool {
        self.header
    }
}
