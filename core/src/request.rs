use now_lambda::Body;
use serde::de::DeserializeOwned;
use serde_json::error::Error as SerdeError;

pub fn from_body<T: DeserializeOwned>(body: Body) -> Result<T, SerdeError> {
    let data = String::from(body);
    match serde_json::from_str(&data) {
        Ok(object) => Ok(object),
        Err(e) => Err(e),
    }
}
