use std::env;
use std::time::SystemTime;

fn main() {
    println!("Cache-Control: no-cache");
    println!("Content-Type: text/html\n");

    let ip = env::var("REMOTE_ADDR").unwrap_or_else(|_| "Unknown".to_string());
    let now = SystemTime::now();
    
    println!("<!DOCTYPE html>");
    println!("<html>");
    println!("<head>");
    println!("<title>Hello Rust</title>");
    println!("</head>");
    println!("<body>");
    println!("<h1>Hello, Rust!</h1>");
    println!("<p>Greeting from Team Liam</p>");
    println!("<p>This page was generated with the Rust programming language</p>");
    println!("<p>Generated at: {:?}</p>", now);
    println!("<p>Your IP Address: {}</p>", ip);
    println!("</body>");
    println!("</html>");
}
