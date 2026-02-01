use std::env;
use std::time::SystemTime;

fn main() {
    println!("Cache-Control: no-cache");
    println!("Content-Type: application/json\n");

    let ip = env::var("REMOTE_ADDR").unwrap_or_else(|_| "Unknown".to_string());
    let now = SystemTime::now();
    
    println!("{{");
    println!("  \"title\": \"Hello, Rust!\",");
    println!("  \"heading\": \"Hello, Rust!\",");
    println!("  \"message\": \"This page was generated with the Rust programming language\",");
    println!("  \"team\": \"Liam\",");
    println!("  \"time\": \"{:?}\",", now);
    println!("  \"ip\": \"{}\"", ip);
    println!("}}");
}
