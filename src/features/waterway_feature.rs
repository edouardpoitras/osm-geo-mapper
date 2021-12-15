use crate::{
    features::{GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry, WaterwayType},
    operations::{line_string_operations::draw_line, address_from_properties, property_to_option_string},
};
use osm_geo_mapper_macros::{ extract_type_from_string, geotile_from_properties };
use paste::paste; // Required for the extract_type_from_string macro.
use geo_types as gt;
use log::warn;
use std::sync::Arc;

pub fn get_waterway_geo_tile(props: &dyn GeoTileProperties, geometry: Geometry) -> GeoTile {
    let waterway_type_str = props.fetch("waterway").unwrap();
    let waterway_type = extract_type_from_string!(waterway_type_str<props> => WaterwayType [Boatyard, Canal, Dam, Ditch, Dock, Drain, Fairway, Fuel, LockGate, Pressurised, River, Riverbank, Stream, TidalChannel, TurningPoint, Unclassified, Waterfall, WaterPoint, Weir]);
    geotile_from_properties!(geometry<props> => Waterway<waterway_type> [access, boat, canoe, cemt, depth, diameter, dock, draft, fuel, height, industrial, intermittent, layer, location, lock, maxheight, maxlength, maxspeed, maxwidth, motorboat, name, operator, salt, ship, tidal, tunnel, usage, width]);
}

pub fn draw_waterway_line_string(
    geo_tile: Arc<GeoTile>,
    data_structure: GeoTilesDataStructure,
    _waterway_type: WaterwayType,
    line_string: gt::LineString<f64>,
) {
    let points = line_string.into_points();
    let mut first_iteration = true;
    let mut last_point = points[0];
    for point in points {
        if first_iteration {
            first_iteration = false;
            continue;
        }
        draw_line(&last_point, &point, 1, geo_tile.clone(), data_structure.clone());
        last_point = point;
    }
}
