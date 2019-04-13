use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "action")]
pub enum Request {
    Get,
    Set {
        x_coord: u32,
        y_coord: u32,
        colour: String
    }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Response {
    Matrix { state: Vec<CellStateResponse> },
    PaintResult { ok: bool },
    Error { message: String },
}

#[derive(Serialize, Deserialize)]
pub struct CellStateResponse {x: u32, y: u32, colour: String}

impl CellStateResponse {
    pub fn new(x: u32, y: u32, colour: String) -> Self {
        CellStateResponse {
            x, y, colour
        }
    }
}