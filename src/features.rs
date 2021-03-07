#![allow(clippy::write_with_newline)]

use paste::paste;
use geo_types as gt;
use serde_json::{Map, Value as JsonValue};
use std::collections::HashMap;
use std::{fmt, sync::{Arc, RwLock}};

pub mod aeroway_feature;
pub mod amenity_feature;
pub mod barrier_feature;
pub mod boundary_feature;
pub mod building_feature;
pub mod highway_feature;
pub mod landuse_feature;
pub mod leisure_feature;
pub mod man_made_feature;
pub mod natural_feature;
pub mod place_feature;
pub mod power_feature;
pub mod public_transport_feature;
pub mod route_feature;
pub mod tourism_feature;

pub const TILE_SCALE: f64 = 100_000.0;
pub type GeoTilesDataStructure = Arc<RwLock<HashMap<gt::Coordinate<i32>, Arc<GeoTile>>>>;
pub type GeoTileProperties = Map<String, JsonValue>;

#[derive(Debug, Clone)]
pub enum Geometry {
    LineString(geo_types::LineString<f64>),
    Point(geo_types::Point<f64>),
    Polygon(geo_types::Polygon<f64>),
}

#[derive(Debug, Clone)]
pub struct Address {
    pub house_number: Option<String>,
    pub unit: Option<String>,
    pub street: Option<String>,
    pub postal_code: Option<String>,
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut addr = String::new();
        if let Some(number) = &self.house_number {
            addr += format!("{} ", number).as_ref();
        }
        if let Some(unit) = &self.unit {
            addr += format!("{} ", unit).as_ref();
        }
        if let Some(street) = &self.street {
            addr += format!("{} ", street).as_ref();
        }
        if let Some(postal_code) = &self.postal_code {
            addr += format!("{} ", postal_code).as_ref();
        }
        write!(f, "{}", addr.trim_end())?;
        Ok(())
    }
}

/// Helper macro to create the many enums used in this library.
#[macro_export]
macro_rules! create_enum {
    ($($enum:ident [$($variant:ident),*$(,)*]),*$(,)*) => {
        $(
            #[derive(Debug, Clone, Copy)]
            pub enum $enum {
                $(
                    $variant,
                )*
            }
        )*
    }
}

/// You can find all features at https://wiki.openstreetmap.org/wiki/Map_Features
macro_rules! implement_geotile {
    ($($variant:ident [$($attr:ident),*]),*$(,)*) => {
        paste! {
           // First generate the giant enum of GeoTiles.
           #[derive(Debug, Clone)]
           pub enum GeoTile {
               $(
                   $variant {
                       [<$variant:snake _type>]: [<$variant:camel Type>],
                       geometry: Geometry,
                       osm_id: String,
                       address: Option<Address>,
                       $(
                           $attr: Option<String>,
                       )*
                   },
               )*
           }
           // Now generate the display implementation.
           impl fmt::Display for GeoTile {
               fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                   match self {
                       $(
                           GeoTile::$variant {
                               $(
                                   $attr,
                               )*
                               [<$variant:snake _type>],
                               ..
                           } => {
                               let variant_str = stringify!($variant);
                               write!(f, "Feature: {}\n", variant_str)?;
                               write!(f, "Type: {:?}\n", [<$variant:snake _type>])?;
                               print_geotile_attributes!(f => $($attr),*);
                               Ok(())
                           },
                       )*
                   }
               }
            }
        }
    }
}

///
/// Helper macro to pretty-print all the Option<String> attributes of a particular GeoTile variant.
/// print_geotile_attributes!(f => field1, field2, ...)
/// Where f is a &mut fmt::Formatter (available when implementing fmt::Display).
/// 
#[macro_export]
macro_rules! print_geotile_attributes {
    ($f:expr => $($attr: ident),*$(,)*) => {
        {
            $(
                // Extract every Option<String> attribute and print it's value.
                if let Some($attr) = $attr {
                    let mut attr_str = stringify!($attr).to_string();
                    // Replace underscores with spaces and capitalize.
                    attr_str = attr_str.replace("_", " ");
                    let mut c = attr_str.chars();
                    attr_str = match c.next() {
                        None => String::new(),
                        Some(x) => x.to_uppercase().collect::<String>() + c.as_str(),
                    };
                    // Add our print statement.
                    write!($f, "{}: {}\n", attr_str, $attr)?;
                }
            )*
        }
    };
}

create_enum!(
    AerialwayType [CableCar, Gondola], //...
    AerowayType [Aerodrome, Apron, Gate, Hangar, Helipad, Heliport, Navigationaid, Runway, Spaceport, Taxiway, Terminal, Windsock, Unclassified],
    AmenityType [Bar, AmeBBQ, Biergarten, Cafe, DrinkingWater, FastFood, FoodCourt, IceCream, Pub, Restaurant, College, DrivingSchool, Kindergarten, LanguageSchool, Library, ToyLibrary, MusicSchool, School, University, BicycleParking, BicycleRepairStation, BicycleRental, BoatRental, BoatSharing, BusStation, CarRental, CarSharing, CarWash, VehicleInspection, ChargingStation, FerryTerminal, Fuel, GritBin, MotorcycleParking, Parking, ParkingEntrance, ParkingSpace, Taxi, ATM, Bank, BureauDeChange, BabyHatch, Clinic, Dentist, Doctors, Hospital, NursingHome, Pharmacy, SocialFacility, Veterinary, ArtsCentre, Brothel, Casino, Cinema, CommunityCentre, Fountain, Gambling, Nightclub, Planetarium, PublicBookcase, SocialCentre, Stripclub, Studio, Swingerclub, Theatre, AnimalBoarding, AnimalShelter, BakingOven, Bench, Childcare, Clock, ConferenceCentre, Courthouse, Crematorium, DiveCentre, Embassy, FireStation, Firepit, GiveBox, GraveYard, Gym, HuntingStand, InternetCafe, Kitchen, KneippWaterCure, Marketplace, Monastery, PhotoBooth, PlaceOfWorship, Police, PostBox, PostDepot, PostOffice, Prison, PublicBath, PublicBuilding, RangerStation, Recycling, RefugeeSite, SanitaryDumpStation, Sauna, Shelter, Shower, Telephone, Toilets, Townhall, Unclassified, VendingMachine, WasteBasket, WasteDisposal, WasteTransferStation, WateringPlace, WaterPoint],
    BarrierType [Block, Bollard, BorderControl, BumpGate, BusTrap, CableBarrier, CattleGrid, Chain, CityWall, CycleBarrier, Debris, Ditch, Entrance, Fence, FullHeightTurnstile, Gate, GuardRail, HampshireGate, Handrail, Hedge, HeightRestrictor, HorseStile, JerseyBarrier, Kerb, KissingGate, LiftGate, Log, MotorcycleBarrier, RetainingWall, Rope, SallyPort, Spikes, Stile, SumpBuster, SwingGate, TollBooth, Turnstile, Unclassified, Wall],
    BoundaryType [AboriginalLands, Administrative, Maritime, Marker, NationalPark, Political, PostalCode, ProtectedArea, UserDefined, Unclassified],
    BuildingType [Apartments, Bakehouse, Barn, Bridge, Building, Bunker, Bungalow, Cabin, Carport, Cathedral, Chapel, Church, Civic, Commercial, Conservatory, Construction, Cowshed, Detached, Digester, Dormitory, Farm, FarmAuxiliary, FireStation, Garage, Garages, Gatehouse, Ger, Government, Grandstand, Greenhouse, Hangar, Hospital, Hotel, House, Houseboat, Hut, Industrial, Kindergarten, Kiosk, Mosque, Office, Parking, Pavilion, PortableClassroom, Public, Religious, Residential, Retail, RidingHall, Roof, Ruins, School, SemidetachedHouse, Service, Shed, Shrine, SportsHall, SlurryTank, Stable, Stadium, StaticCaravan, Sty, Supermarket, Synagogue, Temple, Terrace, Toilets, TrainStation, TransformerTower, Transportation, TreeHouse, Unclassified, University, Warehouse, WaterTower],
    CraftType [Unclassified],
    EmergencyType [Unclassified],
    GeologicalType [Unclassified],
    HighwayType [Bridleway, BusGuideway, BusStop, Construction, Corridor, Crossing, Cycleway, Escape, Footway, LivingStreet, Motorway, MotorwayLink, Path, Pedestrian, Primary, PrimaryLink, Proposed, Raceway, Road, Residential, Secondary, SecondaryLink, Service, Steps, Stop, StreetLamp, Tertiary, TertiaryLink, Track, TrafficSignals, Trunk, TrunkLink, TurningCircle, Unclassified],
    HistoricType [Unclassified],
    LanduseType [Allotments, Basin, Brownfield, Cemetery, Commercial, Conservation, Construction, Depot, Farmland, Farmyard, Flowerbed, Forest, Garages, Grass, Greenfield, GreenhouseHorticulture, Industrial, Landfill, Meadow, Military, Orchard, PeatCutting, PlantNursery, Port, Quarry, Railway, RecreationGround, Religious, Reservoir, Residential, Retail, SaltPond, Unclassified, VillageGreen, Vineyard],
    LeisureType [AdultGamingCentre, AmusementArcade, BeachResort, Bandstand, BirdHide, Common, Dance, DiscGolfCourse, DogPark, EscapeGame, Firepit, Fishing, FitnessCentre, FitnessStation, Garden, Hackerspace, HorseRiding, IceRink, Marina, MiniatureGolf, NatureReserve, Park, PicnicTable, Pitch, Playground, Slipway, SportsCentre, Stadium, SummerCamp, SwimmingArea, SwimmingPool, Track, Unclassified, WaterPark],
    ManMadeType [Adit, Beacon, Breakwater, Bridge, BunkerSilo, CarpetHanger, Chimney, CommunicationsTower, Crane, Cross, Cutline, Clearcut, Dovecote, Dyke, Embankment, Flagpole, Gasometer, GoodsConveyor, Groyne, Kiln, Lighthouse, Mast, Mineshaft, MonitoringStation, Obelisk, Observatory, OffshorePlatform, PetroleumWell, Pier, Pipeline, PumpingStation, ReservoirCovered, Silo, SnowFence, SnowNet, StorageTank, StreetCabinet, Surveillance, SurveyPoint, Telescope, Tower, Unclassified, WastewaterPlant, Watermill, WaterTower, WaterWell, WaterTap, WaterWorks, WildlifeCrossing, Windmill, Works],
    MilitaryType [Unclassified],
    NaturalType [Wood, TreeRow, Tree, Scrub, Heath, Moor, Grassland, Fell, BareRock, Scree, Shingle, Sand, Mud, Water, Wetland, Glacier, Bay, Strait, Cape, Beach, Coastline, Reef, Spring, HotSpring, Geyser, Blowhole, Peak, Volcano, Valley, Peninsula, Isthmus, Ridge, Arete, Cliff, Saddle, Dune, Rock, Stone, Sinkhole, CaveEntrance, Unclassified],
    OfficeType [Unclassified],
    PlaceType [Allotments, Archipelago, Borough, City, CityBlock, Continent, Country, County, District, Farm, Hamlet, Island, Islet, IsolatedDwelling, Locality, Municipality, Neighbourhood, Ocean, Plot, Province, Quarter, Region, Sea, Square, State, Suburb, Town, Unclassified, Village],
    PowerType [Cable, CatenaryMast, Compensator, Converter, Generator, Heliostat, Insulator, Line, MinorLine, Plant, Pole, Portal, Substation, Switch, Switchgear, Terminal, Tower, Transformer, Unclassified],
    PublicTransportType [Platform, Station, StopArea, StopPosition, Unclassified],
    RailwayType [Unclassified],
    RouteType [Bicycle, Bus, Canoe, Detour, Ferry, Foot, Hiking, Horse, IceSkate, InlineSkates, LightRail, MTB, Piste, Power, Railway, Road, Running, Ski, Subway, Train, Tracks, Tram, Trolleybus, Unclassified],
    ShopType [Unclassified],
    SportType [Unclassified],
    TelecomType [Unclassified],
    TourismType [AlpineHut, Apartment, Aquarium, Artwork, Attraction, CampPitch, CampSite, CaravanSite, Chalet, Gallery, GuestHouse, Hostel, Hotel, Information, Motel, Museum, PicnicSite, ThemePark, Tourism, Unclassified, Viewpoint, WildernessHut, Zoo],
    UnclassifiedType [Unclassified],
    WaterwayType [Unclassified],
);

implement_geotile!(
    Aerialway [],
    Aeroway [name, description, iata, icao, operator, surface],
    Amenity [name, access, amperage, backrest, beds, bottle, brand, brewery, building, capacity, cargo, colour, contact, covered, cuisine, date, delivery, denomination, description, diet, direction, drink, drinking_water, drive_through, emergency, fee, fuel, indoor, lit, material, network, opening_hours, operator, payment, phone, religion, seats, self_service, smoking, socket, voltage, website, wheelchair],
    Barrier [access, bicycle, fee, foot, two_sided, handrail, height, highway, historic, intermittent, lanes, locked, maxheight, maxwidth, motor_vehicle, operator, wheelchair, width],
    Boundary [name, admin_level, area, border_type, description, format, inscription, material, political_division, population, postal_code, protect_class, protection_title],
    Building [name, access, amenity, capacity, covered, entrance, height, levels, office, operator, power, public_transport, shop, sport],
    Craft [],
    Emergency [],
    Geological [],
    Highway [name, abutters, access, bicycle, bus, destination, expressway, foot, hgv, lanes, lit, maxspeed, motor_vehicle, motorcar, motorroad, oneway, operator, service, shelter, sidewalk, sport, smoothness, surface, tracktype, wheelchair, width],
    Historic [],
    Landuse [name, barrier, crop, denomination, genus, industrial, leaf_cycle, leaf_type, meadow, operator, plant, religion, resource, species, trees],
    Leisure [name, access, barrier, building, covered, fee, lit, seasonal, shelter, sport, surface],
    ManMade [name, access, bridge, capacity, color, content, country, covered, cutline, depth, direction, display, disused, drinking_water, elevation, floating, height, headframe, inscription, layer, landuse, length, location, material, mine, mineshaft_type, monitoring, mooring, operator, oven, power, product, pump, pumping_station, resource, species, start_date, street_cabinet, submerged, substance, support, surveillance, survey_point, tidal, tourism, tunnel, width],
    Military [],
    Natural [name, access, circumference, denotation, direction, elevation, height, intermittent, genus, leaf_type, leaf_cycle, managed, operator, salt, species, surface, taxon, width],
    Office [],
    Place [name, admin_level, architect, capital, is_in, population, reference, start_date, state_code],
    Power [name, busbar, cables, circuits, colour, compensator, design, frequency, height, gas_insulated, landuse, line, line_attachment, line_management, location, manufacturer, material, operator, phases, poles, start_date, structure, substation, switch, rating, voltage, windings, wires],
    PublicTransport [name, aerialway, area, bench, bin, building, bus, covered, departures_board, ferry, layer, level, local_ref, monorail, network, operator, passenger_information_display, shelter, subway, surface, tactile_paving, toilet, train, tram, trolleybus, uic_ref, uic_name, wheelchair],
    Railway [],
    Route [name, area, bicycle, colour, description, distance, duration, fee, foot, from, lit, network, oneway, operator, piste_difficulty, piste_type, roundtrip, seasonal, symbol, to],
    Shop [],
    Sport [],
    Telecom [],
    Tourism [name, access, artist_name, artwork_subject, artwork_type, attraction, backcountry, balcony, bar, beds, bbq, brand, cabins, camp_site, capacity, caravans, contact, covered, description, dog, drinking_water, ele, electricity, email, exhibit, fee, fireplace, group_only, heritage, hot_water, information, internet_access, kitchen, lit, material, mattress, motor_vehicle, museum, museum_type, nudism, number_of_apartments, openfire, opening_hours, operator, parking, payment, permanent_camping, picnic_table, phone, power_supply, reservation, rooms, sanitary_dump_station, scout, shower, smoking, stars, start_date, static_caravans, subject, surface, swimming_pool, tents, toilets, washing_machine, waste_disposal, website, wheelchair, wikipedia, winter_room, zoo],
    Waterway [],
    Unclassified [],
);