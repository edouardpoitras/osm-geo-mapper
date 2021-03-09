use crate::{
    features::{GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry, PublicTransportType},
    operations::{line_string_operations::draw_line, address_from_properties, property_to_option_string},
};
use osm_geo_mapper_macros::extract_type_from_string;
use paste::paste; // Required for the extract_type_from_string macro.
use geo_types as gt;
use log::warn;
use std::sync::Arc;

pub fn get_public_transport_geo_tile(props: &GeoTileProperties, geometry: Geometry) -> GeoTile {
    let public_transport_type_str = props["public_transport"].as_str().unwrap();
    let public_transport_type = extract_type_from_string!(public_transport_type_str<props> => PublicTransportType [Platform, Station, StopArea, StopPosition, Unclassified]);
    let address = address_from_properties(props);
    let aerialway = property_to_option_string(props, "aerialway");
    let area = property_to_option_string(props, "area");
    let bench = property_to_option_string(props, "bench");
    let bin = property_to_option_string(props, "bin");
    let building = property_to_option_string(props, "building");
    let bus = property_to_option_string(props, "bus");
    let covered = property_to_option_string(props, "covered");
    let departures_board = property_to_option_string(props, "departures_board");
    let ferry = property_to_option_string(props, "ferry");
    let layer = property_to_option_string(props, "layer");
    let level = property_to_option_string(props, "level");
    let local_ref = property_to_option_string(props, "local_ref");
    let monorail = property_to_option_string(props, "monorail");
    let name = property_to_option_string(props, "name");
    let network = property_to_option_string(props, "network");
    let operator = property_to_option_string(props, "operator");
    let osm_id = props["id"].to_string();
    let passenger_information_display = property_to_option_string(props, "passenger_information_display");
    let shelter = property_to_option_string(props, "shelter");
    let subway = property_to_option_string(props, "subway");
    let surface = property_to_option_string(props, "surface");
    let tactile_paving = property_to_option_string(props, "tactile_paving");
    let toilet = property_to_option_string(props, "toilet");
    let train = property_to_option_string(props, "train");
    let tram = property_to_option_string(props, "tram");
    let trolleybus = property_to_option_string(props, "trolleybus");
    let uic_ref = property_to_option_string(props, "uic_ref");
    let uic_name = property_to_option_string(props, "uic_name");
    let wheelchair = property_to_option_string(props, "wheelchair");
    GeoTile::PublicTransport {
        address,
        aerialway,
        area,
        bench,
        bin,
        building,
        bus,
        covered,
        departures_board,
        ferry,
        geometry,
        layer,
        level,
        local_ref,
        monorail,
        name,
        network,
        operator,
        osm_id,
        passenger_information_display,
        public_transport_type,
        shelter,
        subway,
        surface,
        tactile_paving,
        toilet,
        train,
        tram,
        trolleybus,
        uic_ref,
        uic_name,
        wheelchair,
    }
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
