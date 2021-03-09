use crate::{
    features::{GeoTile, GeoTileProperties, Geometry, BuildingType},
    operations::{address_from_properties, property_to_option_string},
};
use osm_geo_mapper_macros::extract_type_from_string;
use paste::paste; // Required for the extract_type_from_string macro.
use log::warn;

pub fn get_building_geo_tile(props: &GeoTileProperties, geometry: Geometry, building_type_str: &str) -> GeoTile {
    let building_type = extract_type_from_string!(building_type_str<props> => BuildingType [Apartments, Bakehouse, Barn, Bridge, Building, Bunker, Bungalow, Cabin, Carport, Cathedral, Chapel, Church, Civic, Commercial, Conservatory, Construction, Cowshed, Detached, Digester, Dormitory, Farm, FarmAuxiliary, FireStation, Garage, Garages, Gatehouse, Ger, Government, Grandstand, Greenhouse, Hangar, Hospital, Hotel, House, Houseboat, Hut, Industrial, Kindergarten, Kiosk, Mosque, Office, Parking, Pavilion, PortableClassroom, Public, Religious, Residential, Retail, RidingHall, Roof, Ruins, School, SemidetachedHouse, Service, Shed, Shrine, SportsHall, SlurryTank, Stable, Stadium, StaticCaravan, Sty, Supermarket, Synagogue, Temple, Terrace, Toilets, TrainStation, TransformerTower, Transportation, TreeHouse, Unclassified, University, Warehouse, WaterTower]);
    let access = property_to_option_string(props, "access");
    let address = address_from_properties(props);
    let amenity = property_to_option_string(props, "amenity");
    let capacity = property_to_option_string(props, "capacity");
    let covered = property_to_option_string(props, "covered");
    let entrance = property_to_option_string(props, "entrance");
    let height = property_to_option_string(props, "height");
    let levels = property_to_option_string(props, "building:levels");
    let name = property_to_option_string(props, "name");
    let office = property_to_option_string(props, "office");
    let operator = property_to_option_string(props, "operator");
    let osm_id = props["id"].to_string();
    let power = property_to_option_string(props, "power");
    let public_transport = property_to_option_string(props, "public_transport");
    let shop = property_to_option_string(props, "shop");
    let sport = property_to_option_string(props, "sport");
    GeoTile::Building {
        access,
        address,
        amenity,
        building_type,
        capacity,
        covered,
        entrance,
        geometry,
        height,
        levels,
        name,
        office,
        operator,
        osm_id,
        power,
        public_transport,
        shop,
        sport,
    }
}
