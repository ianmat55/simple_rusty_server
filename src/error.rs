use std::collections::HashMap;
use crate::http::StatusCode;
use crate::http::Response;

pub fn build_internal_server_error() -> Response {
    let res = Response {
        status_code: 500,
        status_line: StatusCode::InternalServerError,
        headers: HashMap::new(),
        body: String::from("Internal Server Error").into_bytes()
    };

    return res;
}

pub fn build_not_found_error() -> Response {
    let res = Response {
        status_code: 404,
        status_line: StatusCode::NotFound,
        headers: HashMap::new(),
        body: String::from("Not Found").into_bytes()
    };

    return res;
}