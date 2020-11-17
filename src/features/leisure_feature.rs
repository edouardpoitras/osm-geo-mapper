use crate::{
    features::{GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry, LeisureType},
    operations::{line_string_operations::draw_line, property_to_option_string},
};
use geo_types as gt;
use log::warn;
use std::sync::Arc;

pub fn get_leisure_geo_tile(props: &GeoTileProperties, geometry: Geometry) -> GeoTile {
    let leisure = props["leisure"].as_str().unwrap();
    let leisure_type = match leisure {
        "adult_gaming_centre" => LeisureType::AdultGamingCentre,
        "amusement_arcade" => LeisureType::AmusementArcade,
        "beach_resort" => LeisureType::BeachResort,
        "bandstand" => LeisureType::Bandstand,
        "bird_hide" => LeisureType::BirdHide,
        "common" => LeisureType::Common,
        "dance" => LeisureType::Dance,
        "disc_golf_course" => LeisureType::DiscGolfCourse,
        "dog_park" => LeisureType::DogPark,
        "escape_game" => LeisureType::EscapeGame,
        "firepit" => LeisureType::Firepit,
        "fishing" => LeisureType::Fishing,
        "fitness_centre" => LeisureType::FitnessCentre,
        "fitness_station" => LeisureType::FitnessStation,
        "garden" => LeisureType::Garden,
        "hackerspace" => LeisureType::Hackerspace,
        "horse_riding" => LeisureType::HorseRiding,
        "ice_rink" => LeisureType::IceRink,
        "marina" => LeisureType::Marina,
        "miniature_golf" => LeisureType::MiniatureGolf,
        "nature_reserve" => LeisureType::NatureReserve,
        "park" => LeisureType::Park,
        "picnic_table" => LeisureType::PicnicTable,
        "pitch" => LeisureType::Pitch,
        "playground" => LeisureType::Playground,
        "slipway" => LeisureType::Slipway,
        "sports_centre" => LeisureType::SportsCentre,
        "stadium" => LeisureType::Stadium,
        "summer_camp" => LeisureType::SummerCamp,
        "swimming_area" => LeisureType::SwimmingArea,
        "swimming_pool" => LeisureType::SwimmingPool,
        "track" => LeisureType::Track,
        "water_park" => LeisureType::WaterPark,
        _ => {
            warn!("Unclassified leisure type {}: {:?}", leisure, props);
            LeisureType::Unclassified
        }
    };
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
