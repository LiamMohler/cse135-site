use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Get session ID from cookie
    let cookie_header = env::var("HTTP_COOKIE").unwrap_or_default();
    let mut session_id = String::new();
    
    for cookie in cookie_header.split(';') {
        let parts: Vec<&str> = cookie.trim().split('=').collect();
        if parts.len() == 2 && parts[0] == "session_id" {
            session_id = parts[1].to_string();
        }
    }
    
    // Delete session file
    if !session_id.is_empty() {
        let session_file = format!("/tmp/rust_sessions/session_{}.txt", session_id);
        if Path::new(&session_file).exists() {
            fs::remove_file(&session_file).ok();
        }
    }
    
    // Print headers (expire cookie)
    println!("Content-Type: text/html");
    if !session_id.is_empty() {
        println!("Set-Cookie: session_id={}; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT", session_id);
    }
    println!();
    
    // Print HTML
    println!("<!DOCTYPE html>");
    println!("<html>");
    println!("<head>");
    println!("    <title>Session Destroyed</title>");
    println!("</head>");
    println!("<body>");
    println!("    <h1>Session Destroyed</h1>");
    println!("    <p>Your session has been cleared.</p>");
    println!("    <a href=\"rust-cgiform.html\">Back to the Rust CGI Form</a><br>");
    println!("    <a href=\"rust-sessions-1.cgi\">Back to Page 1</a><br>");
    println!("    <a href=\"rust-sessions-2.cgi\">Back to Page 2</a>");
    println!("</body>");
    println!("</html>");
}
