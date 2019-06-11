use http::{header, StatusCode};
use now_lambda::{error::NowError, lambda, IntoResponse, Request, Response};
use std::error::Error;
//use serde_json;
//use types::Equipment;

fn handler(request: Request) -> Result<impl IntoResponse, NowError> {
    // let (parts, body) = request.into_parts();
    // let body = serde_json::to_vec(&body).expect("Failed to serialize to JSON");
    let uri = request.uri();
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/json")
        .body(format!("{{ requestURI: {} }}", uri))
        .expect("failed to render response");

    return Ok(response);
}

fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
