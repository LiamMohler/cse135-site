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
    
    // Read session data
    let mut username = String::new();
    if !session_id.is_empty() {
        let session_file = format!("/tmp/rust_sessions/session_{}.txt", session_id);
        if Path::new(&session_file).exists() {
            username = fs::read_to_string(&session_file).unwrap_or_default();
        }
    }
    
    // Print headers
    println!("Content-Type: text/html");
    println!();
    
    // Print HTML
    println!("<!DOCTYPE html>");
    println!("<html>");
    println!("<head>");
    println!("    <title>Rust Sessions Page 2</title>");
    println!("</head>");
    println!("<body>");
    println!("    <h1>Rust Sessions Page 2</h1>");
    
    if !username.is_empty() {
        println!("    <p><b>Name:</b> {}</p>", username);
    } else {
        println!("    <p><b>Name:</b> You do not have a name set</p>");
    }
    
    println!("    <br><br>");
    println!("    <a href=\"rust-sessions-1.cgi\">Session Page 1</a><br>");
    println!("    <a href=\"rust-cgiform.html\">Rust CGI Form</a><br>");
    println!("    <form style=\"margin-top:30px\" action=\"rust-destroy-session.cgi\" method=\"get\">");
    println!("        <button type=\"submit\">Destroy Session</button>");
    println!("    </form>");
    println!("</body>");
    println!("</html>");
}
