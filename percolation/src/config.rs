use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub lattice_size: u32,
    pub number_of_trails: u32,
    pub min_probability: f32,
    pub max_probability: f32,
    pub probability_step: f32,
}

pub fn read_config(config_path: &String) -> Result<Config> {
    let config_str = fs::read_to_string(config_path).expect("Unable to read the config file.");
    serde_json::from_str(&config_str)
}
