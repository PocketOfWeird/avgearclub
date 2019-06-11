use http::{header, StatusCode};
use now_lambda::{error::NowError, lambda, IntoResponse, Request, Response};
use serde_json;
use std::error::Error;
use types::Equipment;

fn handler(request: Request) -> Result<impl IntoResponse, NowError> {
    // let (parts, body) = request.into_parts();
    // let body = serde_json::to_vec(&body).expect("Failed to serialize to JSON");
    let equipment: Equipment = Equipment::from(request.into_body());
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/json")
        .body(serde_json::to_string(&equipment).expect("Failed to serialize to JSON"))
        .expect("failed to render response");

    return Ok(response);
}

fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
