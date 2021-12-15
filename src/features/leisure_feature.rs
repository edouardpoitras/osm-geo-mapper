use crate::{
    features::{GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry, LeisureType},
    operations::{line_string_operations::draw_line, address_from_properties, property_to_option_string},
};
use osm_geo_mapper_macros::{ extract_type_from_string, geotile_from_properties };
use paste::paste; // Required for the extract_type_from_string macro.
use geo_types as gt;
use log::warn;
use std::sync::Arc;

pub fn get_leisure_geo_tile(props: &dyn GeoTileProperties, geometry: Geometry) -> GeoTile {
    let leisure_type_str = props.fetch("leisure").unwrap();
    let leisure_type = extract_type_from_string!(leisure_type_str<props> => LeisureType [AdultGamingCentre, AmusementArcade, BeachResort, Bandstand, BirdHide, Common, Dance, DiscGolfCourse, DogPark, EscapeGame, Firepit, Fishing, FitnessCentre, FitnessStation, Garden, Hackerspace, HorseRiding, IceRink, Marina, MiniatureGolf, NatureReserve, Park, PicnicTable, Pitch, Playground, Slipway, SportsCentre, Stadium, SummerCamp, SwimmingArea, SwimmingPool, Track, Unclassified, WaterPark]);
    geotile_from_properties!(geometry<props> => Leisure<leisure_type> [name, access, barrier, building, covered, fee, lit, seasonal, shelter, sport, surface]);
}

pub fn draw_leisure_line_string(
    geo_tile: Arc<GeoTile>,
    data_structure: GeoTilesDataStructure,
    _leisure_type: LeisureType,
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
