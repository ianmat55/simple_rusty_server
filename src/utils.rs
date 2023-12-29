use std::collections::HashMap;
use rand::Rng;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum FileReadError {
    FileNotFound(String),
    IOError(std::io::Error),
    // You can add more variants for other error types if needed
}

#[derive(Serialize, Deserialize, Debug)]
struct ClientGuess {
    data: u8
}

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
        let mut response = format!(
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

#[derive(Serialize, Deserialize, Debug)]
struct Guess {
    message: String,
    rand: u8,
}

fn read_file(file_path: &str) -> Result<Vec<u8>, FileReadError> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => {
            return if error.kind() == io::ErrorKind::NotFound {
                Err(FileReadError::FileNotFound(file_path.to_owned()))
            } else {
                Err(FileReadError::IOError(error))
            };
        }
    };

    let mut file_contents = Vec::new();
    file.read_to_end(&mut file_contents).map_err(FileReadError::IOError)?;
    Ok(file_contents)
}

fn trim_null_bytes(data_str: &str) -> String {
    data_str.trim_matches(char::from(0)).trim().to_string()
}

fn build_internal_server_error() -> Response {
    let res = Response {
        status_code: 500,
        status_line: StatusCode::InternalServerError,
        headers: HashMap::new(),
        body: String::from("Internal Server Error").into_bytes()
    };

    return res;
}

fn build_not_found_error() -> Response {
    let res = Response {
        status_code: 404,
        status_line: StatusCode::NotFound,
        headers: HashMap::new(),
        body: String::from("Not Found").into_bytes()
    };

    return res;
}

fn check_guess(guess: u8) -> Vec<u8> {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Secret number is {}", secret_number);
    let res = if guess == secret_number {
        "equal"
    } else {
        "not equal"
    }.to_string(); // Convert to String
    
    let guess = Guess {
        message: res, 
        rand: secret_number,
    };
    
    let serialized = serde_json::to_string(&guess).unwrap(); 

    return serialized.into_bytes();
}

fn parse_request(buffer: &[u8; 1024]) -> Result<Request, &str> {
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

fn handle_get(req: Request) -> Response {
    let status_code: u16 = 200;
    let status_line: StatusCode = StatusCode::Ok;
    let mut headers: HashMap<String, String> = HashMap::new();
    let body: Vec<u8>;

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
    let mut headers: HashMap<String, String> = HashMap::new();
    let body: Vec<u8>;

    let res = match req.path.trim() {
        "/" => {
            if let Some(data) = &req.data {
                let cleaned_json = trim_null_bytes(data);
     
                body = match serde_json::from_str::<ClientGuess>(&cleaned_json) {
                    Ok(parsed_data) => {
                        println!("Parsed data: {}", parsed_data.data);
                        // match check_guess(parsed_data.data) {
                        //     Ok(data) => data,
                        //     Err(error) => {
                        //         return build_internal_server_error();
                        //     },
                        // }
                        check_guess(parsed_data.data)
                    },
                    Err(error) => {
                        eprintln!("Failed to parse JSON: {}", error);
                        return build_internal_server_error();
                    },
                };

                let response = Response {
                    status_code,
                    status_line,
                    headers,
                    body,
                };

                return response;
            } else {
                return build_internal_server_error();
            }
        },
        _ => return build_not_found_error(),
    };
}

pub fn handle_response(buffer: &[u8; 1024]) -> Response {
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

