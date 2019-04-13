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

#[derive(Debug)]
pub struct CanvasManager {
    canvas: HashMap<Cell, String>
}

impl CanvasManager {
    pub fn new() -> Self {
        let x_size = 10;
        let y_size = 10;
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

        json_response
    }

    pub fn set(&mut self, x_coord: u32, y_coord: u32, colour: String) -> AppResult<Value> {
        self.canvas.insert(Cell::new(x_coord, y_coord), colour);

        let result = Response::PaintResult {
            ok: true
        };
        serde_json::to_value(result).map_err(Into::into)
    }
}