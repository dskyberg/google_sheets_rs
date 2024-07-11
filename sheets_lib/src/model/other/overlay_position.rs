use serde::{Deserialize, Serialize};

use super::GridCoordinate;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct OverlayPosition {
    pub anchor_cell: GridCoordinate,
    pub offset_x_pixels: u32,
    pub offset_y_pixels: u32,
    pub width_pixels: u32,
    pub height_pixels: u32,
}
