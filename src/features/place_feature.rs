use crate::{
    features::{GeoTile, GeoTileProperties, Geometry, PlaceType},
    operations::property_to_option_string,
};
use log::warn;

pub fn get_place_geo_tile(props: &GeoTileProperties, geometry: Geometry) -> GeoTile {
    let place = props["place"].as_str().unwrap();
    let place_type = match place {
        "allotments" => PlaceType::Allotments,
        "archipelago" => PlaceType::Archipelago,
        "borough" => PlaceType::Borough,
        "city" => PlaceType::City,
        "city_block" => PlaceType::CityBlock,
        "continent" => PlaceType::Continent,
        "country" => PlaceType::Country,
        "county" => PlaceType::County,
        "district" => PlaceType::District,
        "farm" => PlaceType::Farm,
        "hamlet" => PlaceType::Hamlet,
        "island" => PlaceType::Island,
        "islet" => PlaceType::Islet,
        "isolated_dwelling" => PlaceType::IsolatedDwelling,
        "locality" => PlaceType::Locality,
        "municipality" => PlaceType::Municipality,
        "neighbourhood" => PlaceType::Neighbourhood,
        "ocean" => PlaceType::Ocean,
        "plot" => PlaceType::Plot,
        "province" => PlaceType::Province,
        "quarter" => PlaceType::Quarter,
        "region" => PlaceType::Region,
        "sea" => PlaceType::Sea,
        "square" => PlaceType::Square,
        "state" => PlaceType::State,
        "suburb" => PlaceType::Suburb,
        "town" => PlaceType::Town,
        "village" => PlaceType::Village,
        _ => {
            warn!("Unclassified place type {}: {:?}", place, props);
            PlaceType::Unclassified
        }
    };
    let admin_level = property_to_option_string(props, "admin_level");
    let architect = property_to_option_string(props, "architect");
    let capital = property_to_option_string(props, "capital");
    let is_in = property_to_option_string(props, "is_in");
    let name = property_to_option_string(props, "name");
    let osm_id = props["id"].to_string();
    let population = property_to_option_string(props, "population");
    let reference = property_to_option_string(props, "ref");
    let start_date = property_to_option_string(props, "start_date");
    let state_code = property_to_option_string(props, "state_code");
    GeoTile::Place {
        admin_level,
        architect,
        capital,
        geometry,
        is_in,
        name,
        osm_id,
        place_type,
        population,
        reference,
        start_date,
        state_code,
    }
}