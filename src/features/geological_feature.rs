use crate::{
    features::{GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry, GeologicalType},
    operations::{line_string_operations::draw_line, address_from_properties, property_to_option_string},
};
use osm_geo_mapper_macros::{ extract_type_from_string, geotile_from_properties };
use paste::paste; // Required for the extract_type_from_string macro.
use geo_types as gt;
use log::warn;
use std::sync::Arc;

pub fn get_geological_geo_tile(props: &GeoTileProperties, geometry: Geometry) -> GeoTile {
    let geological_type_str = props["geological"].as_str().unwrap();
    let geological_type = extract_type_from_string!(geological_type_str<props> => GeologicalType [Moraine, Outcrop, PalaeontologicalSite, Unclassified]);
    geotile_from_properties!(geometry<props> => Geological<geological_type> [name, surface]);
}

pub fn draw_geological_line_string(
    geo_tile: Arc<GeoTile>,
    data_structure: GeoTilesDataStructure,
    _geological_type: GeologicalType,
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
