use crate::{
    features::{GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry, LeisureType},
    operations::{line_string_operations::draw_line, address_from_properties, property_to_option_string},
};
use osm_geo_mapper_macros::extract_type_from_string;
use paste::paste; // Required for the extract_type_from_string macro.
use geo_types as gt;
use log::warn;
use std::sync::Arc;

pub fn get_leisure_geo_tile(props: &GeoTileProperties, geometry: Geometry) -> GeoTile {
    let leisure_type_str = props["leisure"].as_str().unwrap();
    let leisure_type = extract_type_from_string!(leisure_type_str<props> => LeisureType [AdultGamingCentre, AmusementArcade, BeachResort, Bandstand, BirdHide, Common, Dance, DiscGolfCourse, DogPark, EscapeGame, Firepit, Fishing, FitnessCentre, FitnessStation, Garden, Hackerspace, HorseRiding, IceRink, Marina, MiniatureGolf, NatureReserve, Park, PicnicTable, Pitch, Playground, Slipway, SportsCentre, Stadium, SummerCamp, SwimmingArea, SwimmingPool, Track, Unclassified, WaterPark]);
    let address = address_from_properties(props);
    let access = property_to_option_string(props, "access");
    let barrier = property_to_option_string(props, "barrier");
    let building = property_to_option_string(props, "building");
    let covered = property_to_option_string(props, "covered");
    let fee = property_to_option_string(props, "fee");
    let lit = property_to_option_string(props, "lit");
    let name = property_to_option_string(props, "name");
    let osm_id = props["id"].to_string();
    let seasonal = property_to_option_string(props, "seasonal");
    let shelter = property_to_option_string(props, "shelter");
    let sport = property_to_option_string(props, "sport");
    let surface = property_to_option_string(props, "surface");
    GeoTile::Leisure {
        address,
        access,
        barrier,
        building,
        covered,
        fee,
        geometry,
        leisure_type,
        lit,
        name,
        osm_id,
        seasonal,
        shelter,
        sport,
        surface,
    }
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
