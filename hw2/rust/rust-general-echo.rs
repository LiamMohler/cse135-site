use std::env;
use std::io::{self, Read};

fn main() {
    let protocol = env::var("SERVER_PROTOCOL").unwrap_or_default();
    let method = env::var("REQUEST_METHOD").unwrap_or_default();
    let query_string = env::var("QUERY_STRING").unwrap_or_default();
    
    // Read message body from stdin
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
    println!("<html><head><title>General Request Echo</title>");
    println!("</head><body><h1 align='center'>General Request Echo</h1>");
    println!("<hr>");
    println!("<p><b>HTTP Protocol:</b> {}</p>", protocol);
    println!("<p><b>HTTP Method:</b> {}</p>", method);
    println!("<p><b>Query String:</b> {}</p>", query_string);
    println!("<p><b>Message Body:</b> {}</p>", form_data);
    println!("</body></html>");
}
