use crate::{
    features::{GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry, TelecomType},
    operations::{line_string_operations::draw_line, address_from_properties, property_to_option_string},
};
use osm_geo_mapper_macros::{ extract_type_from_string, geotile_from_properties };
use paste::paste; // Required for the extract_type_from_string macro.
use geo_types as gt;
use log::warn;
use std::sync::Arc;

pub fn get_telecom_geo_tile(props: &dyn GeoTileProperties, geometry: Geometry) -> GeoTile {
    let telecom_type_str = props.fetch("telecom").unwrap();
    let telecom_type = extract_type_from_string!(telecom_type_str<props> => TelecomType [ConnectionPoint, DataCenter, DistributionPoint, Exchange, ServiceDevice, Unclassified]);
    geotile_from_properties!(geometry<props> => Telecom<telecom_type> [capacity, connection_point, location, manufacturer, name, operator, owner, street_cabinet, support]);
}

pub fn draw_telecom_line_string(
    geo_tile: Arc<GeoTile>,
    data_structure: GeoTilesDataStructure,
    _telecom_type: TelecomType,
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
