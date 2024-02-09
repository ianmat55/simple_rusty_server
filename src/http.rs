use std::collections::HashMap;

use crate::utils::read_file;
use crate::utils::FileReadError;
use crate::utils::trim_null_bytes;
use crate::guess::check_guess;
use crate::guess::ClientGuess;
use crate::error::build_internal_server_error;
use crate::error::build_not_found_error;

#[derive(PartialEq)]
pub enum HttpType {
    GET,
    POST
}

pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
    InternalServerError = 500,
}

impl StatusCode {
    fn phrase(&self) -> &str {
        match self {
            StatusCode::Ok => "OK",
            StatusCode::NotFound => "Not Found",
            StatusCode::BadRequest => "Bad Request",
            StatusCode::InternalServerError => "InternalServerError",
        }
    }
}

pub struct Request {
    pub method: HttpType,
    pub path: String,
    pub data: Option<String>,
}
pub struct Response {
    pub status_code: u16,
    pub status_line: StatusCode,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn format(&self) -> Vec<u8> {
        let mut response: String = format!(
            "HTTP/1.1 {} {}\r\n",
            self.status_code,
            self.status_line.phrase()
        );

        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }

        response.push_str("\r\n");
        let mut response_bytes = response.into_bytes();

        response_bytes.extend_from_slice(&self.body);
        
        response_bytes
    }
}

fn handle_get(req: Request) -> Response {
    let status_code: u16 = 200;
    let status_line: StatusCode = StatusCode::Ok;
    let headers: HashMap<String, String> = HashMap::new();
    // let body: Vec<u8>;

    let file_path = match req.path.trim() {
        "/" => "client/index.html",
        "/main.js" => "client/main.js",
        "/favicon.ico" => "client/icon.png",
        "/styles.css" => "client/styles.css",
        _ => "client/404.html",
    };
    
    let res: Response;
    
    match read_file(file_path) {
        Ok(contents) => {
            res = Response {
                status_code,
                status_line,
                headers,
                body: contents,
            };
        },
        Err(error) => {
            res = match error {
                FileReadError::FileNotFound(_) => build_not_found_error(),
                FileReadError::IOError(_) => build_internal_server_error(),
            };
        },
    };

    return res;
}

fn handle_post(req: Request) -> Response {
    let status_code: u16 = 200;
    let status_line: StatusCode = StatusCode::Ok;
    let headers: HashMap<String, String> = HashMap::new();

    match req.path.trim() {
        "/" => {
            if let Some(data) = &req.data {
                let cleaned_json = trim_null_bytes(data);
     
                let body = match serde_json::from_str::<ClientGuess>(&cleaned_json) {
                    Ok(parsed_data) => {
                        println!("Parsed data: {}", parsed_data.data);
                        check_guess(parsed_data.data)
                    },
                    Err(error) => {
                        eprintln!("Failed to parse JSON: {}", error);
                        return build_internal_server_error();
                    },
                };

                return Response {
                    status_code,
                    status_line,
                    headers,
                    body,
                };
            } else {
                return build_internal_server_error();
            }
        },
        _ => return build_not_found_error(),
    };
}

fn parse_request(buffer: &[u8]) -> Result<Request, &str> {
    let request_str = std::str::from_utf8(buffer).unwrap();

    // println!("request:\n {}", request_str);
    
    // get our first line which is the important one
    let mut http = request_str.lines();
    let http_line = http.next().unwrap();

    let http_parts: Vec<&str> = http_line.split_whitespace().collect();
        
    // first half is HTTP type, second is path, third is alawys HTTP/1.1
    if http_parts.len() < 3 {
        return Err("ERROR");
    }

    let method: HttpType = match http_parts[0] {
        "GET" => HttpType::GET,
        "POST" => HttpType::POST,
        _ => return Err("Invalid http method"),
    };

    let path = String::from(http_parts[1]);

    // Handling POST
    let mut data = None;
    if method == HttpType::POST {
        let body = request_str.splitn(2, "\r\n\r\n").nth(1).unwrap_or("");
        data = Some(String::from(body));
    }

    let req: Request = Request {
        method,
        path,
        data, 
    };

    return Ok(req);
}

pub fn handle_response(buffer: &[u8]) -> Response {
    let req = match parse_request(buffer) {
        Ok(req) => req,
        Err(error) => {
            eprintln!("Failed to parse request: {}", error);
            // Set status for failed request
            return build_internal_server_error(); 
        }
    };

    let res = match req.method {
        HttpType::GET => {
            handle_get(req)
        },
        HttpType::POST => {
            handle_post(req)
        },
    };

    return res;
}