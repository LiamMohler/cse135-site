use std::env;
use std::fs;
use std::io::{self, Read};
use std::collections::HashMap;

fn main() {
    println!("Cache-Control: no-cache");

    let session_dir = "/tmp/rust_sessions";
    let _ = fs::create_dir_all(session_dir);

    // Get or create session
    let cookie_string = env::var("HTTP_COOKIE").unwrap_or_default();
    let mut session_id = String::new();
    
    for cookie in cookie_string.split(';') {
        let parts: Vec<&str> = cookie.trim().split('=').collect();
        if parts.len() == 2 && parts[0] == "session_id" {
            session_id = parts[1].to_string();
        }
    }
    
    if session_id.is_empty() {
        session_id = format!("{}", std::process::id());
    }

    let session_file = format!("{}/sess_{}", session_dir, session_id);

    // Handle POST
    if env::var("REQUEST_METHOD").unwrap_or_default() == "POST" {
        if let Ok(content_length) = env::var("CONTENT_LENGTH") {
            if let Ok(len) = content_length.parse::<usize>() {
                let mut buffer = vec![0; len];
                let _ = io::stdin().read_exact(&mut buffer);
                let post_data = String::from_utf8_lossy(&buffer);
                
                let mut data: HashMap<String, String> = HashMap::new();
                for pair in post_data.split('&') {
                    let parts: Vec<&str> = pair.split('=').collect();
                    if parts.len() == 2 {
                        data.insert(parts[0].to_string(), parts[1].to_string());
                    }
                }

                let session_data = format!(
                    "username={}\nfavorite_color={}\nnotes={}",
                    data.get("username").unwrap_or(&String::new()),
                    data.get("favorite_color").unwrap_or(&String::new()),
                    data.get("notes").unwrap_or(&String::new())
                );
                
                let _ = fs::write(&session_file, session_data);
                
                println!("Set-Cookie: session_id={}; Path=/", session_id);
                println!("Location: state-view-rust.cgi\n");
                return;
            }
        }
    }

    // Display form
    println!("Set-Cookie: session_id={}; Path=/", session_id);
    println!("Content-Type: text/html\n");

    println!("<!DOCTYPE html>");
    println!("<html>");
    println!("<head><title>Rust State Input</title></head>");
    println!("<body>");
    println!("<h1>Rust State - Input</h1>");
    println!("<form method='POST'>");
    println!("<label>Username:</label><br>");
    println!("<input type='text' name='username'><br><br>");
    println!("<label>Favorite Color:</label><br>");
    println!("<input type='text' name='favorite_color'><br><br>");
    println!("<label>Notes:</label><br>");
    println!("<textarea name='notes' rows='4'></textarea><br><br>");
    println!("<button type='submit'>Save</button>");
    println!("</form>");
    println!("<p><a href='state-view-rust.cgi'>View Data</a></p>");
    println!("</body></html>");
}
