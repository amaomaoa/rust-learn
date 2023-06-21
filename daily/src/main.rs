use serde_derive::{Deserialize, Serialize};
use std::{env::args, fs::write};

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    city: String,
    server: String,
    time: String,
    info: String,
}

fn get_all_info(seq: &str) -> String {
    "info".to_string()
}

fn main() {
    let input_city_name = args().nth(1).unwrap();
    let input_server_name = args().nth(2).unwrap();
    let output = args().nth(3).unwrap();
    let now = chrono::Utc::now().to_string();
    let out_josn = Config {
        city: input_city_name,
        server: input_server_name,
        time: now,
        info: get_all_info("11"),
    };
    write(output, serde_json::to_string_pretty(&out_josn).unwrap()).unwrap();
}
