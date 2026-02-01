use std::env;

fn main() {
    let query_string = env::var("QUERY_STRING").unwrap_or_default();
    
    println!("Cache-Control: no-cache");
    println!("Content-Type: text/html\n");
    
    println!("<!DOCTYPE html>");
    println!("<html><head><title>GET Request Echo</title>");
    println!("</head><body><h1 align='center'>GET Request Echo</h1>");
    println!("<hr>");
    println!("<p><b>Query String:</b> {}</p>", query_string);
    
    // Parse query string
    if !query_string.is_empty() {
        for param in query_string.split('&') {
            let parts: Vec<&str> = param.split('=').collect();
            if parts.len() == 2 {
                let key = parts[0];
                let value = parts[1].replace("+", " ");
                println!("{} = {}<br/>", key, value);
            }
        }
    }
    
    println!("</body></html>");
}
