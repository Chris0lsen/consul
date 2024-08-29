use serde_json::Result;
use std::fs::File;
use std::io::{BufReader, Read};
use toml;

use crate::data_structs::{Config, TaskListData};

pub fn load() -> Result<TaskListData> {
    // Read the config file
    let config = read_config("config.toml").map_err(serde_json::Error::io)?;
    println!("Config: {:?}", config);

    // Open the data file specified in the config
    let file = File::open(config.data_path).map_err(serde_json::Error::io)?;
    let reader = BufReader::new(file);

    // Deserialize the JSON data into a Rust struct
    let data: TaskListData = serde_json::from_reader(reader)?;

    // Now you can use `data` in your application
    println!("{:?}", data);

    Ok(data)
}

fn read_config(config_path: &str) -> std::result::Result<Config, std::io::Error> {
    let mut file = File::open(config_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}
