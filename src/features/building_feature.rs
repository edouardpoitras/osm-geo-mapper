use crate::{
    features::{BuildingType, GeoTile, GeoTileProperties, Geometry},
    operations::{address_from_properties, property_to_option_string},
};
use log::warn;

pub fn get_building_geo_tile(props: &GeoTileProperties, geometry: Geometry, part: bool) -> GeoTile {
    let building = if !part {
        props["building"].as_str().unwrap()
    } else {
        props["building:part"].as_str().unwrap()
    };
    let building_type = match building {
        "apartments" => BuildingType::Residential,
        "barn" => BuildingType::Barn,
        "commercial" => BuildingType::Commercial,
        "detached" => BuildingType::Detached,
        "dormitory" => BuildingType::Residential,
        "farm" => BuildingType::Farm,
        "garage" => BuildingType::Garage,
        "greenhouse" => BuildingType::Greenhouse,
        "house" => BuildingType::House,
        "hospital" => BuildingType::Hospital,
        "industrial" => BuildingType::Industrial,
        "portable_classroom" => BuildingType::PortableClassroom,
        "residential" => BuildingType::Residential,
        "roof" => BuildingType::Roof,
        "school" => BuildingType::School,
        "service" => BuildingType::Service,
        "shed" => BuildingType::Shed,
        "terrace" => BuildingType::Terrace,
        _ => {
            warn!("Unclassified building type {}", building);
            BuildingType::Unclassified
        }
    };
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
