use crate::{
    features::{GeoTile, GeoTileProperties, Geometry, PlaceType},
    operations::property_to_option_string, operations::address_from_properties,
};
use osm_geo_mapper_macros::{ extract_type_from_string, geotile_from_properties };
use paste::paste; // Required for the extract_type_from_string macro.
use log::warn;

pub fn get_place_geo_tile(props: &dyn GeoTileProperties, geometry: Geometry) -> GeoTile {
    let place_type_str = props.fetch("place").unwrap();
    let place_type = extract_type_from_string!(place_type_str<props> => PlaceType [Allotments, Archipelago, Borough, City, CityBlock, Continent, Country, County, District, Farm, Hamlet, Island, Islet, IsolatedDwelling, Locality, Municipality, Neighbourhood, Ocean, Plot, Province, Quarter, Region, Sea, Square, State, Suburb, Town, Unclassified, Village]);
    geotile_from_properties!(geometry<props> => Place<place_type> [name, admin_level, architect, capital, is_in, population, reference, start_date, state_code]);
}