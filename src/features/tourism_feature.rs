use crate::{
    features::{GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry, TourismType},
    operations::{line_string_operations::draw_line, address_from_properties, property_to_option_string},
};
use osm_geo_mapper_macros::{ extract_type_from_string, geotile_from_properties };
use paste::paste; // Required for the extract_type_from_string macro.
use geo_types as gt;
use log::warn;
use std::sync::Arc;

pub fn get_tourism_geo_tile(props: &dyn GeoTileProperties, geometry: Geometry) -> GeoTile {
    let tourism_type_str = props.fetch("tourism").unwrap();
    let tourism_type = extract_type_from_string!(tourism_type_str<props> => TourismType [AlpineHut, Apartment, Aquarium, Artwork, Attraction, CampPitch, CampSite, CaravanSite, Chalet, Gallery, GuestHouse, Hostel, Hotel, Information, Motel, Museum, PicnicSite, ThemePark, Tourism, Unclassified, Viewpoint, WildernessHut, Zoo]);
    geotile_from_properties!(geometry<props> => Tourism<tourism_type> [name, access, artist_name, artwork_subject, artwork_type, attraction, backcountry, balcony, bar, beds, bbq, brand, cabins, camp_site, capacity, caravans, contact, covered, description, dog, drinking_water, ele, electricity, email, exhibit, fee, fireplace, group_only, heritage, hot_water, information, internet_access, kitchen, lit, material, mattress, motor_vehicle, museum, museum_type, nudism, number_of_apartments, openfire, opening_hours, operator, parking, payment, permanent_camping, picnic_table, phone, power_supply, reservation, rooms, sanitary_dump_station, scout, shower, smoking, stars, start_date, static_caravans, subject, surface, swimming_pool, tents, toilets, washing_machine, waste_disposal, website, wheelchair, wikipedia, winter_room, zoo]);
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
