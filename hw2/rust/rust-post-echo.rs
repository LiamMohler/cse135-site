use std::env;
use std::io::{self, Read};

fn main() {
    // Read POST data from stdin
    let content_length: usize = env::var("CONTENT_LENGTH")
        .unwrap_or_default()
        .parse()
        .unwrap_or(0);
    
    let mut form_data = String::new();
    if content_length > 0 {
        io::stdin().take(content_length as u64).read_to_string(&mut form_data).ok();
    }
    
    println!("Cache-Control: no-cache");
    println!("Content-Type: text/html\n");
    
    println!("<!DOCTYPE html>");
    println!("<html><head><title>POST Request Echo</title>");
    println!("</head><body><h1 align='center'>POST Request Echo</h1>");
    println!("<hr>");
    println!("<p><b>Message Body:</b></p>");
    println!("<ul>");
    
    // Parse form data
    if !form_data.is_empty() {
        for param in form_data.split('&') {
            let parts: Vec<&str> = param.split('=').collect();
            if parts.len() == 2 {
                let key = parts[0];
                let value = parts[1].replace("+", " ");
                println!("<li>{} = {}</li>", key, value);
            }
        }
    }
    
    println!("</ul>");
    println!("</body></html>");
}
