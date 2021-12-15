use crate::{
    features::{AmenityType, GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry},
    operations::{line_string_operations::draw_line, address_from_properties, property_to_option_string},
};
use osm_geo_mapper_macros::{ extract_type_from_string, geotile_from_properties };
use paste::paste; // Required for the extract_type_from_string macro.
use geo_types as gt;
use log::warn;
use std::sync::Arc;

pub fn get_amenity_geo_tile(props: &dyn GeoTileProperties, geometry: Geometry) -> GeoTile {
    let amenity_type_str = props.fetch("amenity").unwrap();
    let amenity_type = extract_type_from_string!(amenity_type_str<props> => AmenityType [Bar, AmeBBQ, Biergarten, Cafe, DrinkingWater, FastFood, FoodCourt, IceCream, Pub, Restaurant, College, DrivingSchool, Kindergarten, LanguageSchool, Library, ToyLibrary, MusicSchool, School, University, BicycleParking, BicycleRepairStation, BicycleRental, BoatRental, BoatSharing, BusStation, CarRental, CarSharing, CarWash, VehicleInspection, ChargingStation, FerryTerminal, Fuel, GritBin, MotorcycleParking, Parking, ParkingEntrance, ParkingSpace, Taxi, ATM, Bank, BureauDeChange, BabyHatch, Clinic, Dentist, Doctors, Hospital, NursingHome, Pharmacy, SocialFacility, Veterinary, ArtsCentre, Brothel, Casino, Cinema, CommunityCentre, Fountain, Gambling, Nightclub, Planetarium, PublicBookcase, SocialCentre, Stripclub, Studio, Swingerclub, Theatre, AnimalBoarding, AnimalShelter, BakingOven, Bench, Childcare, Clock, ConferenceCentre, Courthouse, Crematorium, DiveCentre, Embassy, FireStation, Firepit, GiveBox, GraveYard, Gym, HuntingStand, InternetCafe, Kitchen, KneippWaterCure, Marketplace, Monastery, PhotoBooth, PlaceOfWorship, Police, PostBox, PostDepot, PostOffice, Prison, PublicBath, PublicBuilding, RangerStation, Recycling, RefugeeSite, SanitaryDumpStation, Sauna, Shelter, Shower, Telephone, Toilets, Townhall, Unclassified, VendingMachine, WasteBasket, WasteDisposal, WasteTransferStation, WateringPlace, WaterPoint]);
    geotile_from_properties!(geometry<props> => Amenity<amenity_type> [name, access, amperage, backrest, beds, bottle, brand, brewery, building, capacity, cargo, colour, contact, covered, cuisine, date, delivery, denomination, description, diet, direction, drink, drinking_water, drive_through, fee, fuel, indoor, lit, material, network, opening_hours, operator, payment, phone, religion, seats, self_service, smoking, socket, voltage, website, wheelchair]);
}

pub fn draw_amenity_line_string(
    geo_tile: Arc<GeoTile>,
    data_structure: GeoTilesDataStructure,
    _amenity_type: AmenityType,
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
