use std::thread::sleep;
use std::time::Duration;

use serde_json::Value;

use crate::CONFIG;

fn get_reddit_data() -> String {
    let mut data = "".to_string();
    while data.len() < 1024 {
        data = match ureq::get(
            format!(
                "https://www.reddit.com/r/Kitten/top/.json?time=week&sort=top&limit={}",
                CONFIG["Per_day"].as_u64().unwrap()
            )
            .as_str(),
        )
        .call()
        {
            Ok(x) => x.into_string().unwrap(),
            Err(_) => "".to_string(),
        };
        sleep(Duration::from_secs(10));
    }
    data
}

pub fn get_reddit_value() -> Vec<Value> {
    let data: Value = serde_json::from_str(get_reddit_data().as_str()).unwrap();
    data["data"]["children"].as_array().unwrap().to_vec()
}
