use std::collections::HashMap;
use std::env::current_dir;
use std::fs;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::net::TcpStream;

use chrono::Local;

use self::response::{HttpResponseStatus, ResponseHttp};

pub mod header;
mod response;

pub struct HttpPacket {
    file_requested: Option<String>,
    headers: header::Headers,
    method: Option<String>,
    path: Option<String>,
}

pub trait HttpServerTrait {
    fn stream_content(&mut self, stream: TcpStream);
}

impl HttpServerTrait for HttpPacket {
    fn stream_content(&mut self, mut stream: TcpStream) {
        let reader = BufReader::new(&mut stream);
        let requested_packet: Vec<_> = reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        self.parse_http_packet(requested_packet);
        self.write_response(stream);
    }
}

impl HttpPacket {
    pub fn new_server() -> Self {
        let mut http = HttpPacket {
            file_requested: None,
            headers: HashMap::new(),
            method: None,
            path: None,
        };
        http.default_headers();
        http
    }
    fn parse_http_packet(&mut self, packets: Vec<String>) {
        let first_packet: Vec<&str> = packets[0].split(" ").collect();
        self.method = Some(String::from(first_packet[0]));
        self.path = Some(String::from(first_packet[1]));
        for v in &packets[1..] {
            let header_content: Vec<&str> = v.split(": ").collect();
            let key = String::from(header_content[0]);
            let value = String::from(header_content[1]);
            self.add_new_header(key, value);
        }
    }

    fn write_response(&mut self, mut stream: TcpStream) {
        stream.write_all(self.writer_response().as_bytes()).unwrap();
    }

    fn add_new_header(&mut self, key: String, value: String) {
        header::parse(&mut self.headers, key, value);
    }

    fn default_headers(&mut self) {
        let local_date = Local::now().format("%a, %d %b %Y %H:%M:%S %Z");
        self.add_new_header(String::from("Server"), String::from("WebServerRust/1.0"));
        self.add_new_header(String::from("Date"), String::from(local_date.to_string()));
        self.add_new_header(
            String::from("Content-Type"),
            String::from("text/html; charset=utf-8"),
        );
    }

    fn writer_response(&mut self) -> String {
        self.parse_content().response_format()
    }

    fn parse_content(&mut self) -> ResponseHttp {
        self.extract_file_from_path();
        let cwd = format!("{}{}", current_dir().unwrap().display(), "/web");
        let file_requested = if self.path.as_ref().unwrap() == "/" {
            self.file_requested.as_ref().unwrap()
        } else {
            self.path.as_ref().unwrap()
        };
        let mut response_http = ResponseHttp::new_response_http(&mut self.headers, None, None);
        match fs::read_to_string(format!("{}/{}", cwd, file_requested).as_str()) {
            Ok(contents) => {
                response_http.content = contents;
                response_http.code = HttpResponseStatus::status_ok();
                response_http
            }
            Err(err) => match err.kind() {
                ErrorKind::NotFound => {
                    let content_default_not_found: String = format!(
                        r#"
                    <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>Not Found</title>
                <link href="css/style.css" rel="stylesheet">
            </head>
            <body>
              <h1>404 Not Found File</h1>
            </body>
        </html>
                    "#
                    )
                    .to_string();
                    response_http.content = content_default_not_found;
                    response_http.code = HttpResponseStatus::status_not_found();
                    response_http
                }
                _ => {
                    let content_default_internal_error: String = format!(
                        r#"
                    <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>Internal Error</title>
                <link href="css/style.css" rel="stylesheet">
            </head>
            <body>
              <h1>Internal Server Error</h1>
            </body>
        </html>

                    "#
                    )
                    .to_string();
                    response_http.content = content_default_internal_error;
                    response_http.code = HttpResponseStatus::status_internal_server_error();
                    response_http
                }
            },
        }
    }

    fn extract_file_from_path(&mut self) {
        if self.method.clone().unwrap() == "GET" {
            if self.path.clone().unwrap() == "/" {
                self.add_new_header(
                    String::from("Content-Type"),
                    String::from("text/html; charset=utf-8"),
                );
                self.file_requested = Some(String::from("index.html"));
                return;
            }
            if self
                .headers
                .get(&header::HeaderKeys::ContentType)
                .expect("Content type is empty")
                .contains("text/html")
            {
                let splits: Vec<&str> = self.path.as_ref().unwrap().split("/").collect();
                let mut file_name = String::from(splits[splits.len() - 1]);
                let split_file_namme: Vec<&str> = file_name.split(".").collect();
                if split_file_namme.len() == 1 {
                    file_name.push_str(".html");
                    self.path.as_mut().unwrap().push_str(".html");
                }
                self.file_requested = Some(file_name);
            }
        }
    }
}
