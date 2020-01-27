use redis::{Commands};
use std::env;
use std::process;

fn fetch_key(key: &String) -> redis::RedisResult<String> {
    let redis_url: &str = "redis://localhost:6379";
    let client = redis::Client::open(redis_url)?;
    let mut con = client.get_connection()?;
    con.get(key)
}

fn main() {
    let key = env::var("key").expect("key cannot be empty");
    match fetch_key(&key) {
        Ok(value) => println!("{}", value),
        Err(err) => {
            println!("{}", err.to_string());
            process::exit(1);
        }
    };
}
