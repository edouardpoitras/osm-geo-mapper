use tui::text::Text;
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

pub fn geo_tile_text_lines(viewport: &Viewport) -> Text {
    let mut text = Text::from("");
    {
        let locked_data_structure = viewport.data_structure.read().unwrap();
        if let Some(geo_tiles) = locked_data_structure.get(&viewport.coordinates) {
            for geo_tile in geo_tiles.into_iter() {
                text.extend(Text::from(format!("{}\n", geo_tile)));
            }
        } else {
            text.extend(Text::from("No details available\n".to_string()));
        }
    }
    text.extend(Text::from(coord_to_lat_long_string(&viewport.coordinates)));
    text
}
