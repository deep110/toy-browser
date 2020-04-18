use reqwest;
use std::fs;

pub struct HttpClient {
    client: reqwest::blocking::Client,
    error_string: String,
    file_prefix: &'static str,
}

impl HttpClient {
    pub fn get(self, url: &str) -> String {
        // add local file read
        if url.contains(self.file_prefix) {
            let file_path = url.replace(self.file_prefix, "");
            return match fs::read_to_string(file_path) {
                Err(_) => self.error_string,
                Ok(data) => data,
            };
        }

        match self.client.get(url).send() {
            Err(_) => self.error_string,
            Ok(resp) => match resp.text() {
                Err(_) => self.error_string,
                Ok(text) => text,
            },
        }
    }
}

pub fn create_http_client() -> HttpClient {
    HttpClient {
        client: reqwest::blocking::Client::new(),
        error_string: String::from("<html><body>Error</body></html>"),
        file_prefix: "file://",
    }
}
