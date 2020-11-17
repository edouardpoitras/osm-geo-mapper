use geo_types as gt;

use super::viewport::Viewport;
use crate::operations;

pub fn coord_to_lat_long_string(coord: &gt::Coordinate<i32>) -> String {
    format!(
        "Lat: {}, Long: {}",
        operations::from_tile_scale(coord.y),
        operations::from_tile_scale(coord.x),
    )
}

pub fn geo_tile_text_lines(viewport: &Viewport) -> Vec<String> {
    let mut vec = Vec::new();
    {
        let locked_data_structure = viewport.data_structure.read().unwrap();
        if let Some(tile) = locked_data_structure.get(&viewport.coordinates) {
            vec.push(format!("{}", tile));
        } else {
            vec.push("No details available\n".to_string());
        }
    }
    vec.push(coord_to_lat_long_string(&viewport.coordinates));
    vec
}
