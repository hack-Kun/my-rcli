use clap::Parser;
use rcli::{parse_csv, write_json, Commands, Csv};

fn main() -> anyhow::Result<()> {
    let cli = Csv::parse();
    match cli.cmd {
        Commands::Csv(opts) => {
            let input_path = opts.input();
            // 从文件读取csv并解析成结构体
            let mut players = Vec::with_capacity(128);
            parse_csv(&input_path, &mut players)?;
            // println!("{:?}", players);
            // 将csv 的对象vec 写入json
            let output_path = opts.output();
            write_json(&output_path, &players)?;
        }
    }
    Ok(())
}
