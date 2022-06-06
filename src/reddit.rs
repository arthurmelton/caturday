use std::thread::sleep;
use std::time::Duration;

fn get_reddit_data() -> String {
    let mut data = "".to_string();
    while data.len() < 1024 {
        data = match ureq::get("https://www.reddit.com/r/Kitten/top/.json?time=week&sort=top&limit=24")
            .call() {
            Ok(x) => x.into_string().unwrap(),
            Err(_) => "".to_string()
        };
        sleep(Duration::from_secs(10));
    }
    data
}