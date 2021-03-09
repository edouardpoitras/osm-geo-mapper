use crate::{
    features::{AerowayType, GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry},
    operations::{line_string_operations::draw_line, address_from_properties, property_to_option_string},
};
use osm_geo_mapper_macros::extract_type_from_string;
use paste::paste; // Required for the extract_type_from_string macro.
use geo_types as gt;
use log::warn;
use std::sync::Arc;

pub fn get_aeroway_geo_tile(props: &GeoTileProperties, geometry: Geometry) -> GeoTile {
    let aeroway_type_str = props["aeroway"].as_str().unwrap();
    let aeroway_type = extract_type_from_string!(aeroway_type_str<props> => AerowayType [Aerodrome, Apron, Gate, Hangar, Helipad, Heliport, Navigationaid, Runway, Spaceport, Taxiway, Terminal, Windsock, Unclassified]);
    let address = address_from_properties(props);
    let description = property_to_option_string(props, "description");
    let iata = property_to_option_string(props, "iata");
    let icao = property_to_option_string(props, "icao");
    let name = property_to_option_string(props, "name");
    let operator = property_to_option_string(props, "operator");
    let osm_id = props["id"].to_string();
    let surface = property_to_option_string(props, "surface");
    GeoTile::Aeroway {
        address,
        aeroway_type,
        description,
        geometry,
        iata,
        icao,
        name,
        operator,
        osm_id,
        surface,
    }
}

pub fn draw_aeroway_line_string(
    geo_tile: Arc<GeoTile>,
    data_structure: GeoTilesDataStructure,
    _aeroway_type: AerowayType,
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
