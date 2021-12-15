use crate::{
    features::{GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry, MilitaryType},
    operations::{line_string_operations::draw_line, address_from_properties, property_to_option_string},
};
use osm_geo_mapper_macros::{ extract_type_from_string, geotile_from_properties };
use paste::paste; // Required for the extract_type_from_string macro.
use geo_types as gt;
use log::warn;
use std::sync::Arc;

pub fn get_military_geo_tile(props: &dyn GeoTileProperties, geometry: Geometry) -> GeoTile {
    let military_type_str = props.fetch("military").unwrap();
    let military_type = extract_type_from_string!(military_type_str<props> => MilitaryType [Airfield, Bunker, Barracks, Checkpoint, DangerArea, NavalBase, NuclearExplosionSite, ObstacleCourse, Office, Range, TrainingArea, Trench, Unclassified]);
    geotile_from_properties!(geometry<props> => Military<military_type> [access, bunker_type, description, distance, end_date, gun_turret, iata, icao, location, military_service, name, office, opening_hours, operator, start_date, surface, trench]);
}

pub fn draw_military_line_string(
    geo_tile: Arc<GeoTile>,
    data_structure: GeoTilesDataStructure,
    _military_type: MilitaryType,
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
