use crate::{
    features::{AmenityType, GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry},
    operations::{line_string_operations::draw_line, address_from_properties, property_to_option_string},
};
use osm_geo_mapper_macros::extract_type_from_string;
use paste::paste; // Required for the extract_type_from_string macro.
use geo_types as gt;
use log::warn;
use std::sync::Arc;

pub fn get_amenity_geo_tile(props: &GeoTileProperties, geometry: Geometry) -> GeoTile {
    let amenity_type_str = props["amenity"].as_str().unwrap();
    let amenity_type = extract_type_from_string!(amenity_type_str<props> => AmenityType [Bar, AmeBBQ, Biergarten, Cafe, DrinkingWater, FastFood, FoodCourt, IceCream, Pub, Restaurant, College, DrivingSchool, Kindergarten, LanguageSchool, Library, ToyLibrary, MusicSchool, School, University, BicycleParking, BicycleRepairStation, BicycleRental, BoatRental, BoatSharing, BusStation, CarRental, CarSharing, CarWash, VehicleInspection, ChargingStation, FerryTerminal, Fuel, GritBin, MotorcycleParking, Parking, ParkingEntrance, ParkingSpace, Taxi, ATM, Bank, BureauDeChange, BabyHatch, Clinic, Dentist, Doctors, Hospital, NursingHome, Pharmacy, SocialFacility, Veterinary, ArtsCentre, Brothel, Casino, Cinema, CommunityCentre, Fountain, Gambling, Nightclub, Planetarium, PublicBookcase, SocialCentre, Stripclub, Studio, Swingerclub, Theatre, AnimalBoarding, AnimalShelter, BakingOven, Bench, Childcare, Clock, ConferenceCentre, Courthouse, Crematorium, DiveCentre, Embassy, FireStation, Firepit, GiveBox, GraveYard, Gym, HuntingStand, InternetCafe, Kitchen, KneippWaterCure, Marketplace, Monastery, PhotoBooth, PlaceOfWorship, Police, PostBox, PostDepot, PostOffice, Prison, PublicBath, PublicBuilding, RangerStation, Recycling, RefugeeSite, SanitaryDumpStation, Sauna, Shelter, Shower, Telephone, Toilets, Townhall, Unclassified, VendingMachine, WasteBasket, WasteDisposal, WasteTransferStation, WateringPlace, WaterPoint]);
    let address = address_from_properties(props);
    let access = property_to_option_string(props, "access");
    let amperage = property_to_option_string(props, "amperage");
    let backrest = property_to_option_string(props, "backrest");
    let beds = property_to_option_string(props, "beds");
    let bottle = property_to_option_string(props, "bottle");
    let brand = property_to_option_string(props, "brand");
    let brewery = property_to_option_string(props, "brewery");
    let building = property_to_option_string(props, "building");
    let capacity = property_to_option_string(props, "capacity");
    let cargo = property_to_option_string(props, "cargo");
    let colour = property_to_option_string(props, "colour");
    let contact = property_to_option_string(props, "contact");
    let covered = property_to_option_string(props, "covered");
    let cuisine = property_to_option_string(props, "cuisine");
    let date = property_to_option_string(props, "date");
    let delivery = property_to_option_string(props, "delivery");
    let denomination = property_to_option_string(props, "denomination");
    let description = property_to_option_string(props, "description");
    let diet = property_to_option_string(props, "diet");
    let direction = property_to_option_string(props, "direction");
    let drink = property_to_option_string(props, "drink");
    let drinking_water = property_to_option_string(props, "drinking_water");
    let drive_through = property_to_option_string(props, "drive_through");
    let emergency = property_to_option_string(props, "emergency");
    let fee = property_to_option_string(props, "fee");
    let fuel = property_to_option_string(props, "fuel");
    let indoor = property_to_option_string(props, "indoor");
    let lit = property_to_option_string(props, "lit");
    let material = property_to_option_string(props, "material");
    let name = property_to_option_string(props, "name");
    let network = property_to_option_string(props, "network");
    let opening_hours = property_to_option_string(props, "opening_hours");
    let operator = property_to_option_string(props, "operator");
    let osm_id = props["id"].to_string();
    let payment = property_to_option_string(props, "payment");
    let phone = property_to_option_string(props, "phone");
    let religion = property_to_option_string(props, "religion");
    let seats = property_to_option_string(props, "seats");
    let self_service = property_to_option_string(props, "self_service");
    let smoking = property_to_option_string(props, "smoking");
    let socket = property_to_option_string(props, "socket");
    let voltage = property_to_option_string(props, "voltage");
    let website = property_to_option_string(props, "website");
    let wheelchair = property_to_option_string(props, "wheelchair");
    GeoTile::Amenity {
        address,
        access,
        amenity_type,
        amperage,
        backrest,
        beds,
        bottle,
        brand,
        brewery,
        building,
        capacity,
        cargo,
        colour,
        contact,
        covered,
        cuisine,
        date,
        delivery,
        denomination,
        description,
        diet,
        direction,
        drink,
        drinking_water,
        drive_through,
        emergency,
        fee,
        fuel,
        indoor,
        geometry,
        lit,
        material,
        name,
        network,
        opening_hours,
        operator,
        osm_id,
        payment,
        phone,
        religion,
        seats,
        self_service,
        smoking,
        socket,
        voltage,
        website,
        wheelchair,
    }
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
