// For the typical use case of an HTTP client builder, thread safety is not necessary. However, if your specific scenario 
// requires it, wrapping the builder in synchronization primitives like Mutex and Arc can ensure thread safety. The final 
// product (HTTP client) should be designed to be thread-safe and immutable, allowing safe concurrent usage across threads.

use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::sync::{Arc, Mutex};
use std::time::Duration;

// Thread-safe builder using Mutex
struct ClientBuilder {
    timeout: Option<Duration>,
    headers: Option<HeaderMap>,
    proxy: Option<String>,
}

impl ClientBuilder {
    fn new() -> Self {
        Self {
            timeout: None,
            headers: None,
            proxy: None,
        }
    }

    fn timeout(&mut self, duration: Duration) -> &mut Self {
        self.timeout = Some(duration);
        self
    }

    fn default_headers(&mut self, headers: HeaderMap) -> &mut Self {
        self.headers = Some(headers);
        self
    }

    fn proxy(&mut self, proxy: String) -> &mut Self {
        self.proxy = Some(proxy);
        self
    }

    fn build(&self) -> Result<Client, Box<dyn std::error::Error>> {
        let mut client_builder = Client::builder();

        if let Some(timeout) = self.timeout {
            client_builder = client_builder.timeout(timeout);
        }

        if let Some(ref headers) = self.headers {
            client_builder = client_builder.default_headers(headers.clone());
        }

        if let Some(ref proxy) = self.proxy {
            client_builder = client_builder.proxy(reqwest::Proxy::https(proxy)?);
        }

        Ok(client_builder.build()?)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let builder = Arc::new(Mutex::new(ClientBuilder::new()));

    // Simulating multiple threads modifying the builder
    let builder_clone = Arc::clone(&builder);
    std::thread::spawn(move || {
        let mut builder = builder_clone.lock().unwrap();
        builder.timeout(Duration::from_secs(10));
    });

    let builder_clone = Arc::clone(&builder);
    std::thread::spawn(move || {
        let mut builder = builder_clone.lock().unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert("X-Custom-Header", HeaderValue::from_static("custom-value"));
        builder.default_headers(headers);
    });

    let builder_clone = Arc::clone(&builder);
    std::thread::spawn(move || {
        let mut builder = builder_clone.lock().unwrap();
        builder.proxy("https://secureproxy.com".to_string());
    });

    // Ensure all threads have completed
    std::thread::sleep(Duration::from_secs(1));

    // Build the client
    let client = builder.lock().unwrap().build()?;

    // Use the client to make a request
    let response = client.get("https://api.example.com/data").send()?;

    println!("Status: {}", response.status());
    Ok(())
}
