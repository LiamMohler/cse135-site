use std::env;

fn main() {
    println!("Cache-Control: no-cache");
    println!("Content-Type: text/html\n");

    println!("<!DOCTYPE html>");
    println!("<html>");
    println!("<head>");
    println!("<title>Rust Environment</title>");
    println!("</head>");
    println!("<body>");
    println!("<h1>Rust Environment Variables</h1>");
    println!("<table border='1'>");
    println!("<tr><th>Variable</th><th>Value</th></tr>");

    for (key, value) in env::vars() {
        println!("<tr><td>{}</td><td>{}</td></tr>", key, value);
    }

    println!("</table>");
    println!("</body>");
    println!("</html>");
}
