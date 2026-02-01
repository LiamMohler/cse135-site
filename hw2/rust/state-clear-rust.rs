use std::env;
use std::fs;

fn main() {
    println!("Cache-Control: no-cache");

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

    if !session_id.is_empty() {
        let session_file = format!("{}/sess_{}", session_dir, session_id);
        let _ = fs::remove_file(&session_file);
    }

    println!("Set-Cookie: session_id=; Path=/; Max-Age=0");
    println!("Location: state-rust.cgi\n");
}
