use crate::{
    features::{AmenityType, GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry},
    operations::{line_string_operations::draw_line, property_to_option_string},
};
use geo_types as gt;
use log::warn;
use std::rc::Rc;

pub fn get_amenity_geo_tile(props: &GeoTileProperties, geometry: Geometry) -> GeoTile {
    let amenity = props["amenity"].as_str().unwrap();
    let amenity_type = match amenity {
        "bar" => AmenityType::Bar,
        "amebbq" => AmenityType::AmeBBQ,
        "biergarten" => AmenityType::Biergarten,
        "cafe" => AmenityType::Cafe,
        "drinking_water" => AmenityType::DrinkingWater,
        "fast_food" => AmenityType::FastFood,
        "food_court" => AmenityType::FoodCourt,
        "ice_cream" => AmenityType::IceCream,
        "pub" => AmenityType::Pub,
        "restaurant" => AmenityType::Restaurant,
        "college" => AmenityType::College,
        "driving_school" => AmenityType::DrivingSchool,
        "kindergarten" => AmenityType::Kindergarten,
        "language_school" => AmenityType::LanguageSchool,
        "library" => AmenityType::Library,
        "toy_library" => AmenityType::ToyLibrary,
        "music_school" => AmenityType::MusicSchool,
        "school" => AmenityType::School,
        "university" => AmenityType::University,
        "bicycle_parking" => AmenityType::BicycleParking,
        "bicycle_repair_station" => AmenityType::BicycleRepairStation,
        "bicycle_rental" => AmenityType::BicycleRental,
        "boat_rental" => AmenityType::BoatRental,
        "boat_sharing" => AmenityType::BoatSharing,
        "bus_station" => AmenityType::BusStation,
        "car_rental" => AmenityType::CarRental,
        "car_sharing" => AmenityType::CarSharing,
        "car_wash" => AmenityType::CarWash,
        "vehicle_inspection" => AmenityType::VehicleInspection,
        "charging_station" => AmenityType::ChargingStation,
        "ferry_terminal" => AmenityType::FerryTerminal,
        "fuel" => AmenityType::Fuel,
        "grit_bin" => AmenityType::GritBin,
        "motorcycle_parking" => AmenityType::MotorcycleParking,
        "parking" => AmenityType::Parking,
        "parking_entrance" => AmenityType::ParkingEntrance,
        "parking_space" => AmenityType::ParkingSpace,
        "taxi" => AmenityType::Taxi,
        "atm" => AmenityType::ATM,
        "bank" => AmenityType::Bank,
        "bureau_de_change" => AmenityType::BureauDeChange,
        "baby_hatch" => AmenityType::BabyHatch,
        "clinic" => AmenityType::Clinic,
        "dentist" => AmenityType::Dentist,
        "doctors" => AmenityType::Doctors,
        "hospital" => AmenityType::Hospital,
        "nursing_home" => AmenityType::NursingHome,
        "pharmacy" => AmenityType::Pharmacy,
        "social_facility" => AmenityType::SocialFacility,
        "veterinary" => AmenityType::Veterinary,
        "arts_centre" => AmenityType::ArtsCentre,
        "brothel" => AmenityType::Brothel,
        "casino" => AmenityType::Casino,
        "cinema" => AmenityType::Cinema,
        "community_centre" => AmenityType::CommunityCentre,
        "fountain" => AmenityType::Fountain,
        "gambling" => AmenityType::Gambling,
        "nightclub" => AmenityType::Nightclub,
        "planetarium" => AmenityType::Planetarium,
        "public_bookcase" => AmenityType::PublicBookcase,
        "social_centre" => AmenityType::SocialCentre,
        "stripclub" => AmenityType::Stripclub,
        "studio" => AmenityType::Studio,
        "swingerclub" => AmenityType::Swingerclub,
        "theatre" => AmenityType::Theatre,
        "animal_boarding" => AmenityType::AnimalBoarding,
        "animal_shelter" => AmenityType::AnimalShelter,
        "baking_oven" => AmenityType::BakingOven,
        "bench" => AmenityType::Bench,
        "childcare" => AmenityType::Childcare,
        "clock" => AmenityType::Clock,
        "conference_centre" => AmenityType::ConferenceCentre,
        "courthouse" => AmenityType::Courthouse,
        "crematorium" => AmenityType::Crematorium,
        "dive_centre" => AmenityType::DiveCentre,
        "embassy" => AmenityType::Embassy,
        "fire_station" => AmenityType::FireStation,
        "firepit" => AmenityType::Firepit,
        "give_box" => AmenityType::GiveBox,
        "grave_yard" => AmenityType::GraveYard,
        "gym" => AmenityType::Gym,
        "hunting_stand" => AmenityType::HuntingStand,
        "internet_cafe" => AmenityType::InternetCafe,
        "kitchen" => AmenityType::Kitchen,
        "kneipp_water_cure" => AmenityType::KneippWaterCure,
        "marketplace" => AmenityType::Marketplace,
        "monastery" => AmenityType::Monastery,
        "photo_booth" => AmenityType::PhotoBooth,
        "place_of_worship" => AmenityType::PlaceOfWorship,
        "police" => AmenityType::Police,
        "post_box" => AmenityType::PostBox,
        "post_depot" => AmenityType::PostDepot,
        "post_office" => AmenityType::PostOffice,
        "prison" => AmenityType::Prison,
        "public_bath" => AmenityType::PublicBath,
        "public_building" => AmenityType::PublicBuilding,
        "ranger_station" => AmenityType::RangerStation,
        "recycling" => AmenityType::Recycling,
        "refugee_site" => AmenityType::RefugeeSite,
        "sanitary_dump_station" => AmenityType::SanitaryDumpStation,
        "sauna" => AmenityType::Sauna,
        "shelter" => AmenityType::Shelter,
        "shower" => AmenityType::Shower,
        "telephone" => AmenityType::Telephone,
        "toilets" => AmenityType::Toilets,
        "townhall" => AmenityType::Townhall,
        "vending_machine" => AmenityType::VendingMachine,
        "waste_basket" => AmenityType::WasteBasket,
        "waste_disposal" => AmenityType::WasteDisposal,
        "waste_transfer_station" => AmenityType::WasteTransferStation,
        "watering_place" => AmenityType::WateringPlace,
        "water_point" => AmenityType::WaterPoint,
        _ => {
            warn!("Unclassified amenity type {}", amenity);
            AmenityType::Unclassified
        }
    };
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
    geo_tile: Rc<GeoTile>,
    data_structure: &mut GeoTilesDataStructure,
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
        draw_line(&last_point, &point, 1, geo_tile.clone(), data_structure);
        last_point = point;
    }
}
