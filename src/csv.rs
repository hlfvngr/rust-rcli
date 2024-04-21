use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    pub name: String,
    pub position: String,
    #[serde(rename = "DOB")]
    pub dob: String,
    #[serde(rename = "Nationality")]
    pub nationality: String,
    #[serde(rename = "Kit Number")]
    pub kit: u8,
}

pub fn convert_csv(input: &str, output: &str) -> Result<()> {
    let mut reader = csv::Reader::from_path(input)?;

    let mut players: Vec<Player> = Vec::new();

    for record in reader.deserialize() {
        let player: Player = record?;
        players.push(player);
    }

    let str = serde_json::to_string_pretty(&players)?;
    let mut output_file = File::create(output)?;
    output_file.write_all(str.as_bytes())?;
    Ok(())
}
