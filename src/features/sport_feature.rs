use crate::{
    features::{GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry, SportType},
    operations::{line_string_operations::draw_line, address_from_properties, property_to_option_string},
};
use osm_geo_mapper_macros::{ extract_type_from_string, geotile_from_properties };
use paste::paste; // Required for the extract_type_from_string macro.
use geo_types as gt;
use log::warn;
use std::sync::Arc;

pub fn get_sport_geo_tile(props: &GeoTileProperties, geometry: Geometry) -> GeoTile {
    let sport_type_str = props["sport"].as_str().unwrap();
    let sport_type = extract_type_from_string!(sport_type_str<props> => SportType [AmericanFootball, Aikido, Archery, Athletics, AustralianFootball, Badminton, Bandy, Baseball, Basketball, Beachvolleyball, Biathlon, Billiards, Bmx, Bobsleigh, Boules, Bowls, Boxing, Bullfighting, CanadianFootball, Canoe, Chess, CliffDiving, Climbing, ClimbingAdventure, Cockfighting, Cricket, Crossfit, Croquet, Curling, Cycling, Darts, DogAgility, DogRacing, Equestrian, Fencing, FieldHockey, Fitness, Floorball, FreeFlying, Futsal, GaelicGames, Golf, Gymnastics, Handball, Hapkido, Horseshoes, HorseRacing, IceHockey, IceSkating, IceStock, Jiu, Judo, Karate, Karting, Kickboxing, Kitesurfing, Korfball, Krachtbal, Lacrosse, MartialArts, MiniatureGolf, ModelAerodrome, Motocross, Motor, Multi, Netball, NinePin, ObstacleCourse, Orienteering, PaddleTennis, Padel, Parachuting, Parkour, Pelota, Pesapallo, Pickleball, Pilates, PoleDance, Racquet, RcCar, RollerSkating, Rowing, RugbyLeague, RugbyUnion, Running, Sailing, ScubaDiving, Shooting, Shot, Skateboard, SkiJumping, Skiing, Snooker, Soccer, Speedway, Squash, Sumo, Surfing, Swimming, TableTennis, TableSoccer, Taekwondo, Tennis, TenPin, Toboggan, Ultimate, Unclassified, Volleyball, Wakeboarding, WaterPolo, WaterSki, Weightlifting, Wrestling, Yoga]);
    geotile_from_properties!(geometry<props> => Sport<sport_type> [access, alt_name, archery, area, athletics, baseball, billiards, boules, capacity, climbing, club, cricket_nets, darts, depth, ele, height, hoops, lanes, length, lit, name, note, opening_hours, operator, shooting, source, surface, takeoff, tidal, wave, website, width]);
}

pub fn draw_sport_line_string(
    geo_tile: Arc<GeoTile>,
    data_structure: GeoTilesDataStructure,
    _sport_type: SportType,
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
