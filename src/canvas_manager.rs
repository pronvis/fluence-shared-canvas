use crate::error_type::AppResult;
use crate::request_response::Response;

use std::collections::hash_map::HashMap;
use std::iter;
use serde_json::Value;
use log::info;

use crate::request_response::CellStateResponse;

// ================== Colour ==================
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Colour {
    r: u8,
    g: u8,
    b: u8
}

impl Colour {
    pub fn new() -> Self {
        Colour {
            r: 0,
            g: 0,
            b: 0
        }
    }
}

// ================== Cell ==================

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct  Cell {
    x: u32,
    y: u32
}

impl Cell {
    pub fn new(x: u32, y: u32) -> Self {
        Cell{x, y}
    }
}

// ================== CanvasManager ==================

pub struct CanvasManager {
    canvas: HashMap<Cell, String>
}

impl CanvasManager {
    pub fn new() -> Self {
        let x_size = 1000;
        let y_size = 1000;
        let white_colour = String::from("#ffafff");

        let mut canvas_map: HashMap<Cell, String> = HashMap::new();
        for x in 0..x_size {
            for y in 0..y_size {
                let cell = Cell::new(x, y);
                canvas_map.insert(cell, white_colour.clone());
            }
        }

        CanvasManager {
            canvas: canvas_map
        }
    }

    pub fn to_response(&self) -> AppResult<Value> {
        let result_state: Vec<CellStateResponse> = self.canvas.iter().map(|kv| CellStateResponse::new(kv.0.x, kv.0.y, kv.1.clone())).collect();
        let result = Response::Matrix {
            state:result_state
        };

        let json_response: AppResult<Value> = serde_json::to_value(result).map_err(Into::into);

        match &json_response {
            Ok(res) => info!("JSON ok : {}", res.to_string()),
            Err(err) => info!("JSON error: {}", err.to_string())
        }

        json_response
    }
}