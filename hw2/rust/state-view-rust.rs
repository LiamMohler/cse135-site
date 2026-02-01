use std::env;
use std::fs;

fn main() {
    println!("Cache-Control: no-cache");
    println!("Content-Type: text/html\n");

    let session_dir = "/tmp/rust_sessions";

    // Get session
    let cookie_string = env::var("HTTP_COOKIE").unwrap_or_default();
    let mut session_id = String::new();
    
    for cookie in cookie_string.split(';') {
        let parts: Vec<&str> = cookie.trim().split('=').collect();
        if parts.len() == 2 && parts[0] == "session_id" {
            session_id = parts[1].to_string();
        }
    }

    let mut username = String::new();
    let mut favorite_color = String::new();
    let mut notes = String::new();
    let mut has_data = false;

    if !session_id.is_empty() {
        let session_file = format!("{}/sess_{}", session_dir, session_id);
        if let Ok(content) = fs::read_to_string(&session_file) {
            has_data = true;
            for line in content.lines() {
                let parts: Vec<&str> = line.split('=').collect();
                if parts.len() == 2 {
                    match parts[0] {
                        "username" => username = parts[1].to_string(),
                        "favorite_color" => favorite_color = parts[1].to_string(),
                        "notes" => notes = parts[1].to_string(),
                        _ => {}
                    }
                }
            }
        }
    }

    println!("<!DOCTYPE html>");
    println!("<html>");
    println!("<head><title>Rust State View</title></head>");
    println!("<body>");
    println!("<h1>Rust State - View</h1>");

    if has_data && !username.is_empty() {
        println!("<p><strong>Username:</strong> {}</p>", username);
        println!("<p><strong>Favorite Color:</strong> {}</p>", favorite_color);
        println!("<p><strong>Notes:</strong> {}</p>", notes);
        println!("<form method='POST' action='state-clear-rust.cgi'>");
        println!("<button type='submit'>Clear</button>");
        println!("</form>");
    } else {
        println!("<p>No data</p>");
    }

    println!("<p><a href='state-rust.cgi'>Back</a></p>");
    println!("</body></html>");
}
