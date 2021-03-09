use crate::{
    features::{GeoTile, GeoTileProperties, Geometry, PlaceType},
    operations::property_to_option_string, operations::address_from_properties,
};
use osm_geo_mapper_macros::extract_type_from_string;
use paste::paste; // Required for the extract_type_from_string macro.
use log::warn;

pub fn get_place_geo_tile(props: &GeoTileProperties, geometry: Geometry) -> GeoTile {
    let place_type_str = props["place"].as_str().unwrap();
    let place_type = extract_type_from_string!(place_type_str<props> => PlaceType [Allotments, Archipelago, Borough, City, CityBlock, Continent, Country, County, District, Farm, Hamlet, Island, Islet, IsolatedDwelling, Locality, Municipality, Neighbourhood, Ocean, Plot, Province, Quarter, Region, Sea, Square, State, Suburb, Town, Unclassified, Village]);
    let address = address_from_properties(props);
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
        address,
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