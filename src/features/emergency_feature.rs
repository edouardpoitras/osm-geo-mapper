use crate::{
    features::{EmergencyType, GeoTile, GeoTileProperties, Geometry},
    operations::{address_from_properties, property_to_option_string},
};
use osm_geo_mapper_macros::{ extract_type_from_string, geotile_from_properties };
use paste::paste; // Required for the extract_type_from_string macro.
use log::warn;

pub fn get_emergency_geo_tile(props: &GeoTileProperties, geometry: Geometry) -> GeoTile {
    let emergency_type_str = props["emergency"].as_str().unwrap();
    let emergency_type = extract_type_from_string!(emergency_type_str<props> => EmergencyType [AmbulanceStation, AssemblyPoint, Defibrillator, DrinkingWater, DryRiserInlet, EmergencyWardEntrance, FireAlarmBox, FireExtinguisher, FireHose, FireHydrant, LandingSite, Lifeguard, LifeguardBase, LifeguardPlatform, LifeguardTower, LifeRing, Phone, SuctionPoint, Siren, Unclassified, WaterTank]);
    geotile_from_properties!(geometry<props> => Emergency<emergency_type> [access, colour, couplings, defibrillator, description, direction, entrance, height, indoor, manufacturer, model, name, opening_hours, operator, phone, support, water_source]);
}