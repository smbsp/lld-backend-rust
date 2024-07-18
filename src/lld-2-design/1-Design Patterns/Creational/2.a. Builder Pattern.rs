// When building an HTTP client, such as the popular reqwest library in Rust or similar libraries in other languages, 
// there are many configurations and options that can be set: base URL, headers, timeouts, proxy settings, authentication, 
// and more. Using the Builder Pattern makes the creation of such a complex object more manageable and readable.

use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a header map and add custom headers
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("X-Custom-Header", HeaderValue::from_static("custom-value"));

    // Using the builder pattern to configure the HTTP client
    let client = Client::builder()
        .timeout(Duration::from_secs(10))  // Set the request timeout
        .default_headers(headers)  // Set the default headers
        .proxy(reqwest::Proxy::https("https://secureproxy.com")?)  // Set the proxy
        .build()?;  // Build the client

    // Use the client to make a request
    let response = client.get("https://api.example.com/data")
        .send()?;

    println!("Status: {}", response.status());
    Ok(())
}
