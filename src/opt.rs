use serde::{Deserialize, Serialize};
// 通过csv工具读取csv文件并转换为结构体模式
pub fn read_csv(file_path: String) -> anyhow::Result<Vec<Player>> {
    let mut opt = Vec::with_capacity(128);
    let mut rdr = csv::Reader::from_path(file_path)?;
    for result in rdr.deserialize() {
        let record: Player = result?;
        opt.push(record);
    }
    Ok(opt)
}

// 将结构体转为json写入文件
pub fn write_json(data: Vec<Player>, file_path: String) -> anyhow::Result<()> {
    let json_str = serde_json::to_string_pretty(&data)?;
    std::fs::write(file_path, json_str)?;
    Ok(())
}

// 定义csv文件对应的结构体
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}
