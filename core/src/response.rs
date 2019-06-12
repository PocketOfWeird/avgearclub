use http::{header, StatusCode};
use now_lambda::{error::NowError, Response};
use serde::Serialize;
use serde_json::error::Error as SerdeError;

pub fn json_error(e: SerdeError) -> Result<Response<String>, NowError> {
    let response = Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .header(header::CONTENT_TYPE, "text/json")
        .body(
            serde_json::json!({
                "error": "true",
                "message": e.to_string()
            })
            .to_string(),
        )
        .expect("failed to render response");
    return Ok(response);
}

pub fn json_ok<T: Serialize>(object: &T) -> Result<Response<String>, NowError> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/json")
        .body(serde_json::to_string(object).expect("Failed to serialize JSON"))
        .expect("Failed to render response");
    return Ok(response);
}
