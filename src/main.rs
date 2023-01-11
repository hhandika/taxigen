use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::BufWriter;

const MAMMAL_DIVERSITY_URL: &str = "https://www.mammaldiversity.org/mdd.json";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    request_color_names().await?;

    Ok(())
}

async fn request_color_names() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(MAMMAL_DIVERSITY_URL).await?;

    match resp.status() {
        reqwest::StatusCode::OK => {
            let color_names: Vec<Taxonomy> = resp.json().await?;
            write_json_to_file(&color_names)?;
        }
        _ => println!("Error"),
    }

    Ok(())
}

fn write_json_to_file(json: &[Taxonomy]) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("json")?;
    let file = File::create("json/mammals.json")?;
    let buff = BufWriter::new(file);

    serde_json::to_writer_pretty(buff, json)?;

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Taxonomy {
    id: String,
    order: String,
    family: String,
    genus: String,
    specific_epithet: String,
    authority_species_author: String,
    authority_species_year: String,
    country_distribution: String,
    continent_distribution: String,
}
