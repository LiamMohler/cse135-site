use std::env;
use std::io::{self, Read};
use std::collections::HashMap;
use std::time::SystemTime;

fn main() {
    println!("Cache-Control: no-cache");
    println!("Content-Type: text/html\n");

    let method = env::var("REQUEST_METHOD").unwrap_or_else(|_| "GET".to_string());
    let ip = env::var("REMOTE_ADDR").unwrap_or_else(|_| "Unknown".to_string());
    let user_agent = env::var("HTTP_USER_AGENT").unwrap_or_else(|_| "Unknown".to_string());
    let hostname = env::var("HOSTNAME").unwrap_or_else(|_| "Unknown".to_string());
    let now = SystemTime::now();

    // Get data
    let mut data: HashMap<String, String> = HashMap::new();
    
    if method == "GET" {
        if let Ok(query) = env::var("QUERY_STRING") {
            for pair in query.split('&') {
                let parts: Vec<&str> = pair.split('=').collect();
                if parts.len() == 2 {
                    data.insert(parts[0].to_string(), parts[1].to_string());
                }
            }
        }
    } else {
        if let Ok(content_length) = env::var("CONTENT_LENGTH") {
            if let Ok(len) = content_length.parse::<usize>() {
                let mut buffer = vec![0; len];
                let _ = io::stdin().read_exact(&mut buffer);
                let post_data = String::from_utf8_lossy(&buffer);
                
                for pair in post_data.split('&') {
                    let parts: Vec<&str> = pair.split('=').collect();
                    if parts.len() == 2 {
                        data.insert(parts[0].to_string(), parts[1].to_string());
                    }
                }
            }
        }
    }

    // Remove form control fields
    data.remove("language");
    data.remove("method");

    println!("<!DOCTYPE html>");
    println!("<html>");
    println!("<head><title>Rust Echo</title></head>");
    println!("<body>");
    println!("<h1>Rust Echo</h1>");
    println!("<p><strong>Method:</strong> {}</p>", method);
    println!("<p><strong>Hostname:</strong> {}</p>", hostname);
    println!("<p><strong>Date/Time:</strong> {:?}</p>", now);
    println!("<p><strong>User Agent:</strong> {}</p>", user_agent);
    println!("<p><strong>IP:</strong> {}</p>", ip);

    println!("<h2>Data Received</h2>");
    if !data.is_empty() {
        println!("<table border='1'>");
        println!("<tr><th>Key</th><th>Value</th></tr>");
        for (key, value) in &data {
            println!("<tr><td>{}</td><td>{}</td></tr>", key, value);
        }
        println!("</table>");
    } else {
        println!("<p>No data</p>");
    }

    println!("</body></html>");
}
