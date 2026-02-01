use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Get query string
    let query_string = env::var("QUERY_STRING").unwrap_or_default();
    
    // Parse username from query
    let mut username_param = String::new();
    for param in query_string.split('&') {
        let parts: Vec<&str> = param.split('=').collect();
        if parts.len() == 2 && parts[0] == "username" {
            username_param = parts[1].replace("+", " ");
        }
    }
    
    // Get or create session ID from cookie
    let cookie_header = env::var("HTTP_COOKIE").unwrap_or_default();
    let mut session_id = String::new();
    
    for cookie in cookie_header.split(';') {
        let parts: Vec<&str> = cookie.trim().split('=').collect();
        if parts.len() == 2 && parts[0] == "session_id" {
            session_id = parts[1].to_string();
        }
    }
    
    // Create new session ID if needed
    if session_id.is_empty() {
        session_id = format!("{:x}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos());
    }
    
    // Session file path
    let session_dir = "/tmp/rust_sessions";
    let session_file = format!("{}/session_{}.txt", session_dir, session_id);
    
    // Create session directory
    fs::create_dir_all(session_dir).ok();
    
    // Read existing session data or create new
    let mut username = String::new();
    if Path::new(&session_file).exists() {
        username = fs::read_to_string(&session_file).unwrap_or_default();
    }
    
    // Update username if provided
    if !username_param.is_empty() {
        username = username_param.clone();
        fs::write(&session_file, &username).ok();
    }
    
    // Print headers
    println!("Content-Type: text/html");
    println!("Set-Cookie: session_id={}; Path=/", session_id);
    println!();
    
    // Print HTML
    println!("<!DOCTYPE html>");
    println!("<html>");
    println!("<head>");
    println!("    <title>Rust Sessions</title>");
    println!("</head>");
    println!("<body>");
    println!("    <h1>Rust Sessions Page 1</h1>");
    
    if !username.is_empty() {
        println!("    <p><b>Name:</b> {}</p>", username);
    } else {
        println!("    <p><b>Name:</b> You do not have a name set</p>");
    }
    
    println!("    <br><br>");
    println!("    <a href=\"rust-sessions-2.cgi\">Session Page 2</a><br>");
    println!("    <a href=\"rust-cgiform.html\">Rust CGI Form</a><br>");
    println!("    <form style=\"margin-top:30px\" action=\"rust-destroy-session.cgi\" method=\"get\">");
    println!("        <button type=\"submit\">Destroy Session</button>");
    println!("    </form>");
    println!("</body>");
    println!("</html>");
}
