// 此模块提供文件读写和验证等方法

use std::fs;

use crate::Player;

// 验证输入文件是否存在
pub fn verify_file(file_path: &str) -> Result<String, &'static str> {
    // 验证文件是否存在，是否可读，是否为CSV文件
    // 存在则返回文件路径，否则返回错误信息
    if std::path::Path::new(file_path).exists() {
        return Ok(file_path.to_string());
    }
    Err("File not found")
}

// 解析csv的方法
pub fn parse_csv(file_path: &str, players: &mut Vec<Player>) -> anyhow::Result<()> {
    // 判断players是否存在容量
    if players.capacity() == 0 {
        return Err(anyhow::anyhow!("players don't have capacity"));
    }
    let mut rdr = csv::Reader::from_path(file_path)?;
    for result in rdr.deserialize() {
        let record: Player = result?;
        players.push(record);
    }
    Ok(())
}

// 将csv 对象写入json文件
pub fn write_json(file_path: &str, players: &Vec<Player>) -> anyhow::Result<()> {
    // 判断players是否为空
    if players.is_empty() {
        return Err(anyhow::anyhow!("players is empty"));
    }
    // 将players序列化成json字符串
    let json_str = serde_json::to_string_pretty(players)?;
    // 将json字符串写入文件
    fs::write(file_path, json_str)?;
    Ok(())
}
