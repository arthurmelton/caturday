use lazy_static::lazy_static;
use serde_json::Value;
use std::fs::{File, self};

use crate::set_cont::set_cont;

lazy_static! {
    pub static ref CONFIG: Value = {
        if File::open("Config.json").is_ok() {
            serde_json::from_str(&fs::read_to_string("Config.json").expect("Unable to read file"))
                .unwrap()
        } else {
            let _ = set_cont(
                "Config.json".to_string(),
                "{\n\t\"Token\": \"(discord bot token)\",
\t\"Application_id\": 123456789,\n
\t\"Channel_id\": 123456789\n}"
                    .to_string(),
            );
            println!("You need to edit the Config.json");
            std::process::exit(1);
        }
    };
}