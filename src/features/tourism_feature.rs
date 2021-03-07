use crate::{
    features::{GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry, TourismType},
    operations::{line_string_operations::draw_line, address_from_properties, property_to_option_string},
};
use geo_types as gt;
use log::warn;
use std::sync::Arc;

pub fn get_tourism_geo_tile(props: &GeoTileProperties, geometry: Geometry) -> GeoTile {
    let tourism_str = props["tourism"].as_str().unwrap();
    let tourism_type = match tourism_str {
        "alpine_hut" => TourismType::AlpineHut,
        "apartment" => TourismType::Apartment,
        "aquarium" => TourismType::Aquarium,
        "artwork" => TourismType::Artwork,
        "attraction" => TourismType::Attraction,
        "camp_pitch" => TourismType::CampPitch,
        "camp_site" => TourismType::CampSite,
        "caravan_site" => TourismType::CaravanSite,
        "chalet" => TourismType::Chalet,
        "gallery" => TourismType::Gallery,
        "guest_house" => TourismType::GuestHouse,
        "hostel" => TourismType::Hostel,
        "hotel" => TourismType::Hotel,
        "information" => TourismType::Information,
        "motel" => TourismType::Motel,
        "museum" => TourismType::Museum,
        "picnic_site" => TourismType::PicnicSite,
        "theme_park" => TourismType::ThemePark,
        "viewpoint" => TourismType::Viewpoint,
        "wilderness_hut" => TourismType::WildernessHut,
        "yes" => TourismType::Tourism,
        "zoo" => TourismType::Zoo,
        _ => {
            warn!("Unclassified tourism type {}: {:?}", tourism_str, props);
            TourismType::Unclassified
        }
    };
    let access = property_to_option_string(props, "access");
    let address = address_from_properties(props);
    let aerialway = property_to_option_string(props, "aerialway");
    let artist_name = property_to_option_string(props, "artist_name");
    let artwork_subject = property_to_option_string(props, "artwork_subject");
    let artwork_type = property_to_option_string(props, "artwork_type");
    let attraction = property_to_option_string(props, "attraction");
    let backcountry = property_to_option_string(props, "backcountry");
    let balcony = property_to_option_string(props, "balcony");
    let bar = property_to_option_string(props, "bar");
    let beds = property_to_option_string(props, "beds");
    let bbq = property_to_option_string(props, "bbq");
    let brand = property_to_option_string(props, "brand");
    let cabins = property_to_option_string(props, "cabins");
    let camp_site = property_to_option_string(props, "camp_site");
    let capacity = property_to_option_string(props, "capacity");
    let caravans = property_to_option_string(props, "caravans");
    let contact = property_to_option_string(props, "contact");
    let covered = property_to_option_string(props, "covered");
    let description = property_to_option_string(props, "description");
    let dog = property_to_option_string(props, "dog");
    let drinking_water = property_to_option_string(props, "drinking_water");
    let ele = property_to_option_string(props, "ele");
    let electricity = property_to_option_string(props, "electricity");
    let email = property_to_option_string(props, "email");
    let exhibit = property_to_option_string(props, "exhibit");
    let fee = property_to_option_string(props, "fee");
    let fireplace = property_to_option_string(props, "fireplace");
    let group_only = property_to_option_string(props, "group_only");
    let heritage = property_to_option_string(props, "heritage");
    let hot_water = property_to_option_string(props, "hot_water");
    let information = property_to_option_string(props, "information");
    let internet_access = property_to_option_string(props, "internet_access");
    let kitchen = property_to_option_string(props, "kitchen");
    let lit = property_to_option_string(props, "lit");
    let material = property_to_option_string(props, "material");
    let mattress = property_to_option_string(props, "mattress");
    let motor_vehicle = property_to_option_string(props, "motor_vehicle");
    let museum = property_to_option_string(props, "museum");
    let museum_type = property_to_option_string(props, "museum_type");
    let name = property_to_option_string(props, "name");
    let nudism = property_to_option_string(props, "nudism");
    let number_of_apartments = property_to_option_string(props, "number_of_apartments");
    let openfire = property_to_option_string(props, "openfire");
    let opening_hours = property_to_option_string(props, "opening_hours");
    let operator = property_to_option_string(props, "operator");
    let osm_id = props["id"].to_string();
    let parking = property_to_option_string(props, "parking");
    let payment = property_to_option_string(props, "payment");
    let permanent_camping = property_to_option_string(props, "permanent_camping");
    let picnic_table = property_to_option_string(props, "picnic_table");
    let phone = property_to_option_string(props, "phone");
    let power_supply = property_to_option_string(props, "power_supply");
    let reservation = property_to_option_string(props, "reservation");
    let rooms = property_to_option_string(props, "rooms");
    let sanitary_dump_station = property_to_option_string(props, "sanitary_dump_station");
    let scout = property_to_option_string(props, "scout");
    let shower = property_to_option_string(props, "shower");
    let smoking = property_to_option_string(props, "smoking");
    let stars = property_to_option_string(props, "stars");
    let start_date = property_to_option_string(props, "start_date");
    let static_caravans = property_to_option_string(props, "static_caravans");
    let subject = property_to_option_string(props, "subject");
    let surface = property_to_option_string(props, "surface");
    let swimming_pool = property_to_option_string(props, "swimming_pool");
    let tents = property_to_option_string(props, "tents");
    let toilets = property_to_option_string(props, "toilets");
    let washing_machine = property_to_option_string(props, "washing_machine");
    let waste_disposal = property_to_option_string(props, "waste_disposal");
    let website = property_to_option_string(props, "website");
    let wheelchair = property_to_option_string(props, "wheelchair");
    let wikipedia = property_to_option_string(props, "wikipedia");
    let winter_room = property_to_option_string(props, "winter_room");
    let zoo = property_to_option_string(props, "zoo");
    GeoTile::Tourism {
        aerialway,
        access,
        address,
        artist_name,
        artwork_subject,
        artwork_type,
        attraction,
        backcountry,
        balcony,
        bar,
        beds,
        bbq,
        brand,
        cabins,
        camp_site,
        capacity,
        caravans,
        contact,
        covered,
        description,
        dog,
        drinking_water,
        ele,
        electricity,
        email,
        exhibit,
        fee,
        fireplace,
        geometry,
        group_only,
        heritage,
        hot_water,
        information,
        internet_access,
        kitchen,
        lit,
        material,
        mattress,
        motor_vehicle,
        museum,
        museum_type,
        name,
        nudism,
        number_of_apartments,
        openfire,
        opening_hours,
        operator,
        osm_id,
        parking,
        payment,
        permanent_camping,
        picnic_table,
        phone,
        power_supply,
        reservation,
        rooms,
        sanitary_dump_station,
        scout,
        shower,
        smoking,
        stars,
        start_date,
        static_caravans,
        subject,
        surface,
        swimming_pool,
        tents,
        toilets,
        tourism_type,
        washing_machine,
        waste_disposal,
        website,
        wheelchair,
        wikipedia,
        winter_room,
        zoo,
    }
}

pub fn draw_tourism_line_string(
    geo_tile: Arc<GeoTile>,
    data_structure: GeoTilesDataStructure,
    _tourism_type: TourismType,
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
