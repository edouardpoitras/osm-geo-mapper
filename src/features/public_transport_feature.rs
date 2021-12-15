use crate::{
    features::{GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry, PublicTransportType},
    operations::{line_string_operations::draw_line, address_from_properties, property_to_option_string},
};
use osm_geo_mapper_macros::{ extract_type_from_string, geotile_from_properties };
use paste::paste; // Required for the extract_type_from_string macro.
use geo_types as gt;
use log::warn;
use std::sync::Arc;

pub fn get_public_transport_geo_tile(props: &dyn GeoTileProperties, geometry: Geometry) -> GeoTile {
    let public_transport_type_str = props.fetch("public_transport").unwrap();
    let public_transport_type = extract_type_from_string!(public_transport_type_str<props> => PublicTransportType [Platform, Station, StopArea, StopPosition, Unclassified]);
    geotile_from_properties!(geometry<props> => PublicTransport<public_transport_type> [name, aerialway, area, bench, bin, building, bus, covered, departures_board, ferry, layer, level, local_ref, monorail, network, operator, passenger_information_display, shelter, subway, surface, tactile_paving, toilet, train, tram, trolleybus, uic_ref, uic_name, wheelchair]);
}

pub fn draw_public_transport_line_string(
    geo_tile: Arc<GeoTile>,
    data_structure: GeoTilesDataStructure,
    _public_transport_type: PublicTransportType,
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
