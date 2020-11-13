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
        "apartments" => BuildingType::Apartments,
        "bakehouse" => BuildingType::Bakehouse,
        "barn" => BuildingType::Barn,
        "bridge" => BuildingType::Bridge,
        "bunker" => BuildingType::Bunker,
        "bungalow" => BuildingType::Bungalow,
        "cabin" => BuildingType::Cabin,
        "carport" => BuildingType::Carport,
        "cathedral" => BuildingType::Cathedral,
        "chapel" => BuildingType::Chapel,
        "church" => BuildingType::Church,
        "civic" => BuildingType::Civic,
        "commercial" => BuildingType::Commercial,
        "conservatory" => BuildingType::Conservatory,
        "construction" => BuildingType::Construction,
        "cowshed" => BuildingType::Cowshed,
        "detached" => BuildingType::Detached,
        "digester" => BuildingType::Digester,
        "dormitory" => BuildingType::Dormitory,
        "farm" => BuildingType::Farm,
        "farm_auxiliary" => BuildingType::FarmAuxiliary,
        "fire_station" => BuildingType::FireStation,
        "garage" => BuildingType::Garage,
        "garages" => BuildingType::Garages,
        "gatehouse" => BuildingType::Gatehouse,
        "ger" => BuildingType::Ger,
        "government" => BuildingType::Government,
        "grandstand" => BuildingType::Grandstand,
        "greenhouse" => BuildingType::Greenhouse,
        "hangar" => BuildingType::Hangar,
        "hospital" => BuildingType::Hospital,
        "hotel" => BuildingType::Hotel,
        "house" => BuildingType::House,
        "houseboat" => BuildingType::Houseboat,
        "hut" => BuildingType::Hut,
        "industrial" => BuildingType::Industrial,
        "kindergarten" => BuildingType::Kindergarten,
        "kiosk" => BuildingType::Kiosk,
        "mosque" => BuildingType::Mosque,
        "office" => BuildingType::Office,
        "parking" => BuildingType::Parking,
        "pavilion" => BuildingType::Pavilion,
        "portable_classroom" => BuildingType::PortableClassroom,
        "public" => BuildingType::Public,
        "religious" => BuildingType::Religious,
        "residential" => BuildingType::Residential,
        "retail" => BuildingType::Retail,
        "riding_hall" => BuildingType::RidingHall,
        "roof" => BuildingType::Roof,
        "ruins" => BuildingType::Ruins,
        "school" => BuildingType::School,
        "semidetached_house" => BuildingType::SemidetachedHouse,
        "service" => BuildingType::Service,
        "shed" => BuildingType::Shed,
        "shrine" => BuildingType::Shrine,
        "sports_hall" => BuildingType::SportsHall,
        "slurry_tank" => BuildingType::SlurryTank,
        "stable" => BuildingType::Stable,
        "stadium" => BuildingType::Stadium,
        "static_caravan" => BuildingType::StaticCaravan,
        "sty" => BuildingType::Sty,
        "supermarket" => BuildingType::Supermarket,
        "synagogue" => BuildingType::Synagogue,
        "temple" => BuildingType::Temple,
        "terrace" => BuildingType::Terrace,
        "toilets" => BuildingType::Toilets,
        "train_station" => BuildingType::TrainStation,
        "transformer_tower" => BuildingType::TransformerTower,
        "transportation" => BuildingType::Transportation,
        "tree_house" => BuildingType::TreeHouse,
        "university" => BuildingType::University,
        "warehouse" => BuildingType::Warehouse,
        "water_tower" => BuildingType::WaterTower,
        "yes" => BuildingType::Building,
        _ => {
            warn!("Unclassified building type {}: {:?}", building, props);
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
