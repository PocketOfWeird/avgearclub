use core::request::from_body;
use core::response::{json_error, json_ok};
use core::types::{Equipment, EquipmentInput};
use now_lambda::{error::NowError, lambda, IntoResponse, Request};
use std::error::Error;


fn handler(request: Request) -> Result<impl IntoResponse, NowError> {
    let input: EquipmentInput = match from_body(request.into_body()) {
        Ok(input) => input,
        Err(e) => return json_error(e),
    };
    let equipment: Equipment = match input.id.is_none() {
        true => Equipment::new(&input),
        false => Equipment::update(&input)
    };

    return json_ok(&equipment);
}

fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
