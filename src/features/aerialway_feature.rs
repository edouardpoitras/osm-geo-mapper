use crate::{
    features::{AerialwayType, GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry},
    operations::{line_string_operations::draw_line, address_from_properties, property_to_option_string},
};
use osm_geo_mapper_macros::{ extract_type_from_string, geotile_from_properties };
use paste::paste; // Required for the extract_type_from_string macro.
use geo_types as gt;
use log::warn;
use std::sync::Arc;

pub fn get_aerialway_geo_tile(props: &GeoTileProperties, geometry: Geometry) -> GeoTile {
    let aerialway_type_str = props["aerialway"].as_str().unwrap();
    let aerialway_type = extract_type_from_string!(aerialway_type_str<props> => AerialwayType [CableCar, ChairLift, DragLift, Gondola, Goods, JBar, MagicCarpet, MixedLift, Platter, Pylon, RopeTow, TBar, Station, Unclassified, ZipLine]);
    geotile_from_properties!(geometry<props> => Aerialway<aerialway_type> [access, duration, ele, fee, foot, incline, maxspeed, maxweight, name, oneway, opening_hours, operator, toll, usage, website]);
}

pub fn draw_aerialway_line_string(
    geo_tile: Arc<GeoTile>,
    data_structure: GeoTilesDataStructure,
    _aerialway_type: AerialwayType,
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
