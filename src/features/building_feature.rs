use crate::{
    features::{GeoTile, GeoTileProperties, Geometry, BuildingType},
    operations::{address_from_properties, property_to_option_string},
};
use osm_geo_mapper_macros::{ extract_type_from_string, geotile_from_properties };
use paste::paste; // Required for the extract_type_from_string macro.
use log::warn;

pub fn get_building_geo_tile(props: &dyn GeoTileProperties, geometry: Geometry, building_type_str: &str) -> GeoTile {
    let building_type = extract_type_from_string!(building_type_str<props> => BuildingType [Apartments, Bakehouse, Barn, Bridge, Building, Bunker, Bungalow, Cabin, Carport, Cathedral, Chapel, Church, Civic, Commercial, Conservatory, Construction, Cowshed, Detached, Digester, Dormitory, Farm, FarmAuxiliary, FireStation, Garage, Garages, Gatehouse, Ger, Government, Grandstand, Greenhouse, Hangar, Hospital, Hotel, House, Houseboat, Hut, Industrial, Kindergarten, Kiosk, Mosque, Office, Parking, Pavilion, PortableClassroom, Public, Religious, Residential, Retail, RidingHall, Roof, Ruins, School, SemidetachedHouse, Service, Shed, Shrine, SportsHall, SlurryTank, Stable, Stadium, StaticCaravan, Sty, Supermarket, Synagogue, Temple, Terrace, Toilets, TrainStation, TransformerTower, Transportation, TreeHouse, Unclassified, University, Warehouse, WaterTower]);
    geotile_from_properties!(geometry<props> => Building<building_type> [name, access, amenity, capacity, covered, entrance, height, levels, office, operator, power, public_transport, shop, sport]);
}
