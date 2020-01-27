use redis::{Commands};
use std::env;
use std::process;

fn fetch_key(key: &String, url: &str) -> redis::RedisResult<String> {
    let client = redis::Client::open(url)?;
    let mut con = client.get_connection()?;
    con.get(key)
}

fn main() {
    let key = env::var("key").expect("key cannot be empty");
    let redis_url = env::var("URL").unwrap_or(String::from("redis://localhost:6379"));
    match fetch_key(&key, redis_url.as_str()) {
        Ok(value) => println!("{}", value),
        Err(err) => {
            println!("{}", err.to_string());
            process::exit(1);
        }
    };
}
