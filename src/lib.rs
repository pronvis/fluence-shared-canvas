mod error_type;
mod request_response;
mod canvas_manager;

use crate::error_type::AppResult;
use crate::canvas_manager::CanvasManager;
use crate::request_response::{Request, Response};

use fluence::sdk::*;
use serde_json::Value;
use std::cell::RefCell;
use log::info;

fn init() {
    logger::WasmLogger::init_with_level(log::Level::Info).unwrap();
}

#[invocation_handler(init_fn = init)]
fn main(req: String) -> String {
    match do_request(req) {
        Ok(res) => res.to_string(),
        Err(err) => {
            let response = Response::Error {
                message: err.to_string(),
            };
            serde_json::to_string(&response).unwrap()
        }
    }
}

thread_local! {
    static CANVAS_MANAGER: RefCell<CanvasManager> = RefCell::new(CanvasManager::new());
}

fn do_request(req: String) -> AppResult<Value> {
    let request: Request = serde_json::from_str(req.as_str())?;

    match request {
        Request::Get => CANVAS_MANAGER.with(|cm| cm.borrow_mut().to_response()),
        Request::Set{x_coord, y_coord, colour} => serde_json::from_str("lala").map_err(Into::into)
    }
}
