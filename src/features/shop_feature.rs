use crate::{
    features::{GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry, ShopType},
    operations::{line_string_operations::draw_line, address_from_properties, property_to_option_string},
};
use osm_geo_mapper_macros::{ extract_type_from_string, geotile_from_properties };
use paste::paste; // Required for the extract_type_from_string macro.
use geo_types as gt;
use log::warn;
use std::sync::Arc;

pub fn get_shop_geo_tile(props: &dyn GeoTileProperties, geometry: Geometry) -> GeoTile {
    let shop_type_str = props.fetch("shop").unwrap();
    let shop_type = extract_type_from_string!(shop_type_str<props> => ShopType [Agrarian, Alcohol, Anime, Antiques, Appliance, Art, Atv, BabyGoods, Bag, Bakery, BathroomFurnishing, Beauty, Bed, Beverages, Bicycle, Boat, Bookmaker, Books, Boutique, BrewingSupplies, Butcher, Camera, Candles, Cannabis, Car, Caravan, CarParts, Carpet, CarRepair, Charity, Cheese, Chemist, Chocolate, Clothes, Coffee, Collector, Computer, Confectionery, Convenience, Copyshop, Cosmetics, Craft, Curtain, Dairy, Deli, DepartmentStore, Doityourself, Doors, Drugstore, DryCleaning, ECigarette, Electrical, Electronics, Energy, Erotic, Fabric, Farm, Fashion, FashionAccessories, Fireplace, Fishing, Flooring, Florist, Frame, FrozenFood, Fuel, FuneralDirectors, Furniture, Games, GardenCentre, GardenFurniture, Gas, General, Gift, Glaziery, Golf, Greengrocer, Groundskeeping, Hairdresser, HairdresserSupply, Hardware, HealthFood, HearingAids, Herbalist, Hifi, HouseholdLinen, Houseware, Hunting, IceCream, InteriorDecoration, Jetski, Jewelry, Kiosk, Kitchen, Lamps, Laundry, Leather, Lighting, Locksmith, Lottery, Mall, Massage, MedicalSupply, MilitarySurplus, MobilePhone, Model, MoneyLender, Motorcycle, Music, MusicalInstrument, Newsagent, NutritionSupplements, Optician, Organic, Outdoor, Outpost, Paint, Party, Pasta, Pastry, Pawnbroker, Perfumery, PestControl, Pet, PetGrooming, Photo, Pyrotechnics, Radiotechnics, Religion, ScubaDiving, Seafood, SecondHand, Security, Sewing, Shoes, Ski, Snowmobile, Spices, Sports, Stationery, StorageRental, Supermarket, SwimmingPool, Tailor, Tattoo, Tea, Ticket, Tiles, Tobacco, Toys, Trade, Trailer, TravelAgency, Trophy, Tyres, Unclassified, User, Vacant, VacuumCleaner, VarietyStore, Video, VideoGames, Watches, Water, Weapons, Wholesale, WindowBlind, Windows, Wine, Wool]);
    geotile_from_properties!(geometry<props> => Shop<shop_type> [agrarian, alcohol, authorization, bakehouse, beauty, books, branch, brand, brewery, bulk_purchase, butcher, cash_withdrawal, clothes, coffee, collector, cuisine, delivery, denomination, description, diet, distillery, drink, dry_cleaning, email, fair_trade, female, fuel, furniture, ice_cream, industrial, laundry_service, lgbtq, licensed, lottery, male, massage, medical_supply, membership, min_age, music, music_genre, musical_instrument, name, opening_hours, operator, organic, origin, oven, ownership, parts, payment, pet, phone, produce, product, religion, rental, repair, reservation, sales, salt, second_hand, self_service, service, shoes, stamps, tobacco, trade, unisex, vending, video_games, website, wheelchair, wholesale, winery]);
}

pub fn draw_shop_line_string(
    geo_tile: Arc<GeoTile>,
    data_structure: GeoTilesDataStructure,
    _shop_type: ShopType,
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
