#![allow(clippy::write_with_newline)]

use paste::paste;
use osmpbfreader::Tags;
use osm_geo_mapper_macros::{ create_enum, implement_geotile, print_geotile_attributes };
use osm_xml::Tag;
use geo_types as gt;
use serde_json::{Map, Value as JsonValue};
use std::collections::HashMap;
use std::fmt::Debug;
use std::{fmt, cmp::Ordering, sync::{Arc, RwLock}};

pub mod aerialway_feature;
pub mod aeroway_feature;
pub mod amenity_feature;
pub mod barrier_feature;
pub mod boundary_feature;
pub mod building_feature;
pub mod craft_feature;
pub mod emergency_feature;
pub mod geological_feature;
pub mod healthcare_feature;
pub mod historic_feature;
pub mod highway_feature;
pub mod landuse_feature;
pub mod leisure_feature;
pub mod man_made_feature;
pub mod military_feature;
pub mod natural_feature;
pub mod office_feature;
pub mod place_feature;
pub mod power_feature;
pub mod public_transport_feature;
pub mod railway_feature;
pub mod route_feature;
pub mod shop_feature;
pub mod sport_feature;
pub mod telecom_feature;
pub mod tourism_feature;
pub mod water_feature;
pub mod waterway_feature;

pub const TILE_SCALE: f64 = 100_000.0;
pub type GeoTilesDataStructure = Arc<RwLock<HashMap<gt::Coordinate<i32>, Vec<Arc<GeoTile>>>>>;

pub trait GeoTileProperties {
    fn has(&self, key: &str) -> bool;
    fn fetch(&self, key: &str) -> Option<&str>;
    fn print_debug(&self) -> String;
}

impl GeoTileProperties for Map<String, JsonValue> {
    fn has(&self, key: &str) -> bool {
        self.contains_key(key)
    }
    fn fetch(&self, key: &str) -> Option<&str> {
        match self.get(key) {
            Some(value) => value.as_str(),
            _ => None
        }
    }
    fn print_debug(&self) -> String {
        let mut output: String = "{".to_string();
        for (k, v) in self {
            output.push_str(k);
            output.push_str(": ");
            output.push_str(&v.to_string());
            output.push_str(",");
        }
        output.push_str("}");
        output
    }
}

impl GeoTileProperties for Vec<Tag> {
    fn has(&self, key: &str) -> bool {
        for tag in self.iter() {
            if tag.key.as_str() == key {
                return true;
            }
        }
        false
    }
    fn fetch(&self, key: &str) -> Option<&str> {
        for tag in self.iter() {
            if tag.key.as_str() == key {
                return Some(tag.val.as_str());
            }
        }
        None
    }
    fn print_debug(&self) -> String {
        let mut output: String = "{".to_string();
        for tag in self.iter() {
            output.push_str(tag.key.as_str());
            output.push_str(": ");
            output.push_str(&tag.val.to_string());
            output.push_str(",");
        }
        output.push_str("}");
        output
    }
}

impl GeoTileProperties for Tags {
    fn has(&self, key: &str) -> bool {
        self.contains_key(key)
    }
    fn fetch(&self, key: &str) -> Option<&str> {
        match self.get(key) {
            Some(value) => Some(value.as_str()),
            _ => None
        }
    }
    fn print_debug(&self) -> String {
        let mut output: String = "{".to_string();
        for k in self.keys() {
            let v = self.get(k).unwrap();
            output.push_str(k.as_str());
            output.push_str(": ");
            output.push_str(&v.to_string());
            output.push_str(",");
        }
        output.push_str("}");
        output
    }
}

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

create_enum!(
    AerialwayType [CableCar, ChairLift, DragLift, Gondola, Goods, JBar, MagicCarpet, MixedLift, Platter, Pylon, RopeTow, TBar, Station, Unclassified, ZipLine],
    AerowayType [Aerodrome, Apron, Gate, Hangar, Helipad, Heliport, Navigationaid, Runway, Spaceport, Taxiway, Terminal, Windsock, Unclassified],
    AmenityType [Bar, AmeBBQ, Biergarten, Cafe, DrinkingWater, FastFood, FoodCourt, IceCream, Pub, Restaurant, College, DrivingSchool, Kindergarten, LanguageSchool, Library, ToyLibrary, MusicSchool, School, University, BicycleParking, BicycleRepairStation, BicycleRental, BoatRental, BoatSharing, BusStation, CarRental, CarSharing, CarWash, VehicleInspection, ChargingStation, FerryTerminal, Fuel, GritBin, MotorcycleParking, Parking, ParkingEntrance, ParkingSpace, Taxi, ATM, Bank, BureauDeChange, BabyHatch, Clinic, Dentist, Doctors, Hospital, NursingHome, Pharmacy, SocialFacility, Veterinary, ArtsCentre, Brothel, Casino, Cinema, CommunityCentre, Fountain, Gambling, Nightclub, Planetarium, PublicBookcase, SocialCentre, Stripclub, Studio, Swingerclub, Theatre, AnimalBoarding, AnimalShelter, BakingOven, Bench, Childcare, Clock, ConferenceCentre, Courthouse, Crematorium, DiveCentre, Embassy, FireStation, Firepit, GiveBox, GraveYard, Gym, HuntingStand, InternetCafe, Kitchen, KneippWaterCure, Marketplace, Monastery, PhotoBooth, PlaceOfWorship, Police, PostBox, PostDepot, PostOffice, Prison, PublicBath, PublicBuilding, RangerStation, Recycling, RefugeeSite, SanitaryDumpStation, Sauna, Shelter, Shower, Telephone, Toilets, Townhall, Unclassified, VendingMachine, WasteBasket, WasteDisposal, WasteTransferStation, WateringPlace, WaterPoint],
    BarrierType [Block, Bollard, BorderControl, BumpGate, BusTrap, CableBarrier, CattleGrid, Chain, CityWall, CycleBarrier, Debris, Ditch, Entrance, Fence, FullHeightTurnstile, Gate, GuardRail, HampshireGate, Handrail, Hedge, HeightRestrictor, HorseStile, JerseyBarrier, Kerb, KissingGate, LiftGate, Log, MotorcycleBarrier, RetainingWall, Rope, SallyPort, Spikes, Stile, SumpBuster, SwingGate, TollBooth, Turnstile, Unclassified, Wall],
    BoundaryType [AboriginalLands, Administrative, Maritime, Marker, NationalPark, Political, PostalCode, ProtectedArea, UserDefined, Unclassified],
    BuildingType [Apartments, Bakehouse, Barn, Bridge, Building, Bunker, Bungalow, Cabin, Carport, Cathedral, Chapel, Church, Civic, Commercial, Conservatory, Construction, Cowshed, Detached, Digester, Dormitory, Farm, FarmAuxiliary, FireStation, Garage, Garages, Gatehouse, Ger, Government, Grandstand, Greenhouse, Hangar, Hospital, Hotel, House, Houseboat, Hut, Industrial, Kindergarten, Kiosk, Mosque, Office, Parking, Pavilion, PortableClassroom, Public, Religious, Residential, Retail, RidingHall, Roof, Ruins, School, SemidetachedHouse, Service, Shed, Shrine, SportsHall, SlurryTank, Stable, Stadium, StaticCaravan, Sty, Supermarket, Synagogue, Temple, Terrace, Toilets, TrainStation, TransformerTower, Transportation, TreeHouse, Unclassified, University, Warehouse, WaterTower],
    CraftType [AgriculturalEngines, Atelier, Bakery, BasketMaker, Beekeeper, Blacksmith, Boatbuilder, Bookbinder, Brewery, Builder, CabinetMaker, CarPainter, Carpenter, CarpetLayer, Caterer, ChimneySweeper, Clockmaker, Confectionery, Cooper, DentalTechnician, Distillery, DoorConstruction, Dressmaker, ElectronicsRepair, Embroiderer, Electrician, Engraver, Floorer, Gardener, Glaziery, Goldsmith, GrindingMill, Handicraft, Hvac, Insulation, InteriorWork, Jeweller, Joiner, KeyCutter, Locksmith, MetalConstruction, Mint, MusicalInstrument, OilMill, Optician, OrganBuilder, Painter, ParquetLayer, Paver, Photographer, PhotographicLaboratory, PianoTuner, Plasterer, Plumber, Pottery, Printer, Printmaker, Rigger, Roofer, Saddler, Sailmaker, Sawmill, Scaffolder, Sculptor, Shoemaker, Signmaker, StandBuilder, Stonemason, SunProtection, Tailor, Tiler, Tinsmith, Toolmaker, Turner, Unclassified, Upholsterer, Watchmaker, WaterWellDrilling, WindowConstruction, Winery],
    EmergencyType [AmbulanceStation, AssemblyPoint, Defibrillator, DrinkingWater, DryRiserInlet, EmergencyWardEntrance, FireAlarmBox, FireExtinguisher, FireHose, FireHydrant, LandingSite, Lifeguard, LifeguardBase, LifeguardPlatform, LifeguardTower, LifeRing, Phone, SuctionPoint, Siren, Unclassified, WaterTank],
    GeologicalType [Moraine, Outcrop, PalaeontologicalSite, Unclassified],
    HealthcareType [Alternative, Audiologist, BirthingCenter, BloodBank, BloodDonation, Counselling, Dialysis, Hospice, Laboratory, Midwife, Nurse, OccupationalTherapist, Optometrist, Physiotherapist, Podiatrist, Psychotherapist, Rehabilitation, SampleCollection, SpeechTherapist, VaccinationCentre, Unclassified],
    HighwayType [Bridleway, BusGuideway, BusStop, Construction, Corridor, Crossing, Cycleway, Escape, Footway, LivingStreet, Motorway, MotorwayLink, Path, Pedestrian, Primary, PrimaryLink, Proposed, Raceway, Road, Residential, Secondary, SecondaryLink, Service, Steps, Stop, StreetLamp, Tertiary, TertiaryLink, Track, TrafficSignals, Trunk, TrunkLink, TurningCircle, Unclassified],
    HistoricType [Aircraft, Aqueduct, ArchaeologicalSite, Battlefield, BombCrater, BoundaryStone, Building, Cannon, Castle, CastleWall, CharcoalPile, Church, CityGate, Citywalls, Farm, Fort, Gallows, HighwaterMark, Locomotive, Manor, Memorial, Milestone, Monastery, Monument, OpticalTelegraph, Pillory, RailwayCar, Ruins, RuneStone, Ship, Tank, Tomb, Tower, Unclassified, WaysideCross, WaysideShrine, Wreck],
    LanduseType [Allotments, Basin, Brownfield, Cemetery, Commercial, Conservation, Construction, Depot, Farmland, Farmyard, Flowerbed, Forest, Garages, Grass, Greenfield, GreenhouseHorticulture, Industrial, Landfill, Meadow, Military, Orchard, PeatCutting, PlantNursery, Port, Quarry, Railway, RecreationGround, Religious, Reservoir, Residential, Retail, SaltPond, Unclassified, VillageGreen, Vineyard],
    LeisureType [AdultGamingCentre, AmusementArcade, BeachResort, Bandstand, BirdHide, Common, Dance, DiscGolfCourse, DogPark, EscapeGame, Firepit, Fishing, FitnessCentre, FitnessStation, Garden, Hackerspace, HorseRiding, IceRink, Marina, MiniatureGolf, NatureReserve, Park, PicnicTable, Pitch, Playground, Slipway, SportsCentre, Stadium, SummerCamp, SwimmingArea, SwimmingPool, Track, Unclassified, WaterPark],
    ManMadeType [Adit, Beacon, Breakwater, Bridge, BunkerSilo, CarpetHanger, Chimney, CommunicationsTower, Crane, Cross, Cutline, Clearcut, Dovecote, Dyke, Embankment, Flagpole, Gasometer, GoodsConveyor, Groyne, Kiln, Lighthouse, Mast, Mineshaft, MonitoringStation, Obelisk, Observatory, OffshorePlatform, PetroleumWell, Pier, Pipeline, PumpingStation, ReservoirCovered, Silo, SnowFence, SnowNet, StorageTank, StreetCabinet, Surveillance, SurveyPoint, Telescope, Tower, Unclassified, WastewaterPlant, Watermill, WaterTower, WaterWell, WaterTap, WaterWorks, WildlifeCrossing, Windmill, Works],
    MilitaryType [Airfield, Bunker, Barracks, Checkpoint, DangerArea, NavalBase, NuclearExplosionSite, ObstacleCourse, Office, Range, TrainingArea, Trench, Unclassified],
    NaturalType [Wood, TreeRow, Tree, Scrub, Heath, Moor, Grassland, Fell, BareRock, Scree, Shingle, Sand, Mud, Water, Wetland, Glacier, Bay, Strait, Cape, Beach, Coastline, Reef, Spring, HotSpring, Geyser, Blowhole, Peak, Volcano, Valley, Peninsula, Isthmus, Ridge, Arete, Cliff, Saddle, Dune, Rock, Stone, Sinkhole, CaveEntrance, Unclassified],
    OfficeType [Accountant, AdvertisingAgency, Architect, Association, Charity, Company, Consulting, Courier, Coworking, Diplomatic, EducationalInstitution, EmploymentAgency, EnergySupplier, Engineer, EstateAgent, Financial, FinancialAdvisor, Forestry, Foundation, Government, Guide, Insurance, It, Lawyer, Logistics, MovingCompany, Newspaper, Ngo, Notary, PoliticalParty, PropertyManagement, Quango, Religion, Research, Surveyor, TaxAdvisor, Telecommunication, Unclassified, Visa, WaterUtility],
    PlaceType [Allotments, Archipelago, Borough, City, CityBlock, Continent, Country, County, District, Farm, Hamlet, Island, Islet, IsolatedDwelling, Locality, Municipality, Neighbourhood, Ocean, Plot, Province, Quarter, Region, Sea, Square, State, Suburb, Town, Unclassified, Village],
    PowerType [Cable, CatenaryMast, Compensator, Converter, Generator, Heliostat, Insulator, Line, MinorLine, Plant, Pole, Portal, Substation, Switch, Switchgear, Terminal, Tower, Transformer, Unclassified],
    PublicTransportType [Platform, Station, StopArea, StopPosition, Unclassified],
    RailwayType [Abandoned, BufferStop, Construction, Crossing, Derail, Disused, Funicular, Halt, LevelCrossing, LightRail, Miniature, Monorail, NarrowGauge, Platform, Preserved, Rail, RailwayCrossing, Roundhouse, Signal, Station, Subway, SubwayEntrance, Switch, Tram, TramStop, Traverser, Turntable, Unclassified, Wash],
    RouteType [Bicycle, Bus, Canoe, Detour, Ferry, Foot, Hiking, Horse, IceSkate, InlineSkates, LightRail, MTB, Piste, Power, Railway, Road, Running, Ski, Subway, Train, Tracks, Tram, Trolleybus, Unclassified],
    ShopType [Agrarian, Alcohol, Anime, Antiques, Appliance, Art, Atv, BabyGoods, Bag, Bakery, BathroomFurnishing, Beauty, Bed, Beverages, Bicycle, Boat, Bookmaker, Books, Boutique, BrewingSupplies, Butcher, Camera, Candles, Cannabis, Car, Caravan, CarParts, Carpet, CarRepair, Charity, Cheese, Chemist, Chocolate, Clothes, Coffee, Collector, Computer, Confectionery, Convenience, Copyshop, Cosmetics, Craft, Curtain, Dairy, Deli, DepartmentStore, Doityourself, Doors, Drugstore, DryCleaning, ECigarette, Electrical, Electronics, Energy, Erotic, Fabric, Farm, Fashion, FashionAccessories, Fireplace, Fishing, Flooring, Florist, Frame, FrozenFood, Fuel, FuneralDirectors, Furniture, Games, GardenCentre, GardenFurniture, Gas, General, Gift, Glaziery, Golf, Greengrocer, Groundskeeping, Hairdresser, HairdresserSupply, Hardware, HealthFood, HearingAids, Herbalist, Hifi, HouseholdLinen, Houseware, Hunting, IceCream, InteriorDecoration, Jetski, Jewelry, Kiosk, Kitchen, Lamps, Laundry, Leather, Lighting, Locksmith, Lottery, Mall, Massage, MedicalSupply, MilitarySurplus, MobilePhone, Model, MoneyLender, Motorcycle, Music, MusicalInstrument, Newsagent, NutritionSupplements, Optician, Organic, Outdoor, Outpost, Paint, Party, Pasta, Pastry, Pawnbroker, Perfumery, PestControl, Pet, PetGrooming, Photo, Pyrotechnics, Radiotechnics, Religion, ScubaDiving, Seafood, SecondHand, Security, Sewing, Shoes, Ski, Snowmobile, Spices, Sports, Stationery, StorageRental, Supermarket, SwimmingPool, Tailor, Tattoo, Tea, Ticket, Tiles, Tobacco, Toys, Trade, Trailer, TravelAgency, Trophy, Tyres, Unclassified, User, Vacant, VacuumCleaner, VarietyStore, Video, VideoGames, Watches, Water, Weapons, Wholesale, WindowBlind, Windows, Wine, Wool],
    SportType [AmericanFootball, Aikido, Archery, Athletics, AustralianFootball, Badminton, Bandy, Baseball, Basketball, Beachvolleyball, Biathlon, Billiards, Bmx, Bobsleigh, Boules, Bowls, Boxing, Bullfighting, CanadianFootball, Canoe, Chess, CliffDiving, Climbing, ClimbingAdventure, Cockfighting, Cricket, Crossfit, Croquet, Curling, Cycling, Darts, DogAgility, DogRacing, Equestrian, Fencing, FieldHockey, Fitness, Floorball, FreeFlying, Futsal, GaelicGames, Golf, Gymnastics, Handball, Hapkido, Horseshoes, HorseRacing, IceHockey, IceSkating, IceStock, Jiu, Judo, Karate, Karting, Kickboxing, Kitesurfing, Korfball, Krachtbal, Lacrosse, MartialArts, MiniatureGolf, ModelAerodrome, Motocross, Motor, Multi, Netball, NinePin, ObstacleCourse, Orienteering, PaddleTennis, Padel, Parachuting, Parkour, Pelota, Pesapallo, Pickleball, Pilates, PoleDance, Racquet, RcCar, RollerSkating, Rowing, RugbyLeague, RugbyUnion, Running, Sailing, ScubaDiving, Shooting, Shot, Skateboard, SkiJumping, Skiing, Snooker, Soccer, Speedway, Squash, Sumo, Surfing, Swimming, TableTennis, TableSoccer, Taekwondo, Tennis, TenPin, Toboggan, Ultimate, Unclassified, Volleyball, Wakeboarding, WaterPolo, WaterSki, Weightlifting, Wrestling, Yoga],
    TelecomType [ConnectionPoint, DataCenter, DistributionPoint, Exchange, ServiceDevice, Unclassified],
    TourismType [AlpineHut, Apartment, Aquarium, Artwork, Attraction, CampPitch, CampSite, CaravanSite, Chalet, Gallery, GuestHouse, Hostel, Hotel, Information, Motel, Museum, PicnicSite, ThemePark, Tourism, Unclassified, Viewpoint, WildernessHut, Zoo],
    UnclassifiedType [Unclassified],
    WaterType [Basin, Canal, Ditch, FishPass, Lagoon, Lake, Lock, Moat, Oxbow, Pond, ReflectingPool, Reservoir, River, StreamPool, Unclassified, Wastewater],
    WaterwayType [Boatyard, Canal, Dam, Ditch, Dock, Drain, Fairway, Fuel, LockGate, Pressurised, River, Riverbank, Stream, TidalChannel, TurningPoint, Unclassified, Waterfall, WaterPoint, Weir],
);

implement_geotile!(
    Aerialway [access, duration, ele, fee, foot, incline, maxspeed, maxweight, name, oneway, opening_hours, operator, toll, usage, website],
    Aeroway [name, description, iata, icao, operator, surface],
    Amenity [name, access, amperage, backrest, beds, bottle, brand, brewery, building, capacity, cargo, colour, contact, covered, cuisine, date, delivery, denomination, description, diet, direction, drink, drinking_water, drive_through, fee, fuel, indoor, lit, material, network, opening_hours, operator, payment, phone, religion, seats, self_service, smoking, socket, voltage, website, wheelchair],
    Barrier [access, bicycle, fee, foot, two_sided, handrail, height, highway, historic, intermittent, lanes, locked, maxheight, maxwidth, motor_vehicle, operator, wheelchair, width],
    Boundary [name, admin_level, area, border_type, description, format, inscription, material, political_division, population, postal_code, protect_class, protection_title],
    Building [name, access, amenity, capacity, covered, entrance, height, levels, office, operator, power, public_transport, shop, sport],
    Craft [builder, brand, carpenter, contact, distillery, electronics, electronics_repair, fax, healthcare, industrial, microbrewery, musical_instrument, name, opening_hours, operator, phone, produce, repair, studio, website, wheelchair],
    Emergency [access, colour, couplings, defibrillator, description, direction, entrance, height, indoor, manufacturer, model, name, opening_hours, operator, phone, support, water_source],
    Geological [name, surface],
    Healthcare [name, opening_hours, operator, phone, vaccination, website, wheelchair],
    Highway [name, abutters, access, bicycle, bus, destination, expressway, foot, hgv, lanes, lit, maxspeed, motor_vehicle, motorcar, motorroad, oneway, operator, service, shelter, sidewalk, sport, smoothness, surface, tracktype, wheelchair, width],
    Historic [architect, artist_name, bridge, castle_type, collection, date, denomination, depth, description, disused, ele, flood_date, format, gauge, height, heritage, image, inscription, location, manufacturer, material, memorial, moved, name, network, operator, optical_telegraph, railway_car, religion, ruins, site_type, start_date, support, tomb, website, wikipedia, year],
    Landuse [name, barrier, crop, denomination, genus, industrial, leaf_cycle, leaf_type, meadow, operator, plant, religion, resource, species, trees],
    Leisure [name, access, barrier, building, covered, fee, lit, seasonal, shelter, sport, surface],
    ManMade [name, access, bridge, capacity, color, content, country, covered, cutline, depth, direction, display, disused, drinking_water, ele, floating, height, headframe, inscription, layer, landuse, length, location, material, mine, mineshaft_type, monitoring, mooring, operator, oven, power, product, pump, pumping_station, resource, species, start_date, street_cabinet, submerged, substance, support, surveillance, survey_point, tidal, tourism, tunnel, width],
    Military [access, bunker_type, description, distance, end_date, gun_turret, iata, icao, location, military_service, name, office, opening_hours, operator, start_date, surface, trench],
    Natural [name, access, circumference, denotation, direction, ele, height, intermittent, genus, leaf_type, leaf_cycle, managed, operator, salt, species, surface, taxon, width],
    Office [admin_level, advertising, association, brand, cargo, club, consulate, consulting, country, denomination, department, diplomatic, email, embassy, faculty, fax, fee, function, government, hgv, industrial, insurance, internet_access, liaison, name, opening_hours, operator, owner, payment, phone, religion, research, social_facility, target, website, wheelchair],
    Place [name, admin_level, architect, capital, is_in, population, reference, start_date, state_code],
    Power [name, busbar, cables, circuits, colour, compensator, design, frequency, height, gas_insulated, landuse, line, line_attachment, line_management, location, manufacturer, material, operator, phases, poles, start_date, structure, substation, switch, rating, voltage, windings, wires],
    PublicTransport [name, aerialway, area, bench, bin, building, bus, covered, departures_board, ferry, layer, level, local_ref, monorail, network, operator, passenger_information_display, shelter, subway, surface, tactile_paving, toilet, train, tram, trolleybus, uic_ref, uic_name, wheelchair],
    Railway [access, area, bench, bicycle, bin, bridge, capacity, colour, control, crossing, cutting, disused, electrified, elevator, embankment, embedded_rails, fee, frequency, funicular, gauge, highspeed, incline, layer, length, light_rail, maxspeed, monorail, network, oneway, opening_hours, operator, passenger, public_transport, rack, request_stop, service, shelter, subway, supervised, surface, surveillance, tactile_paving, toilets, tracks, tram, tunnel, usage, voltage, wheelchair, width, workrules],
    Route [name, area, bicycle, colour, description, distance, duration, fee, foot, from, lit, network, oneway, operator, piste_difficulty, piste_type, roundtrip, seasonal, symbol, to],
    Shop [agrarian, alcohol, authorization, bakehouse, beauty, books, branch, brand, brewery, bulk_purchase, butcher, cash_withdrawal, clothes, coffee, collector, cuisine, delivery, denomination, description, diet, distillery, drink, dry_cleaning, email, fair_trade, female, fuel, furniture, ice_cream, industrial, laundry_service, lgbtq, licensed, lottery, male, massage, medical_supply, membership, min_age, music, music_genre, musical_instrument, name, opening_hours, operator, organic, origin, oven, ownership, parts, payment, pet, phone, produce, product, religion, rental, repair, reservation, sales, salt, second_hand, self_service, service, shoes, stamps, tobacco, trade, unisex, vending, video_games, website, wheelchair, wholesale, winery],
    Sport [access, alt_name, archery, area, athletics, baseball, billiards, boules, capacity, climbing, club, cricket_nets, darts, depth, ele, height, hoops, lanes, length, lit, name, note, opening_hours, operator, shooting, source, surface, takeoff, tidal, wave, website, width],
    Telecom [capacity, connection_point, location, manufacturer, name, operator, owner, street_cabinet, support],
    Tourism [name, access, artist_name, artwork_subject, artwork_type, attraction, backcountry, balcony, bar, beds, bbq, brand, cabins, camp_site, capacity, caravans, contact, covered, description, dog, drinking_water, ele, electricity, email, exhibit, fee, fireplace, group_only, heritage, hot_water, information, internet_access, kitchen, lit, material, mattress, motor_vehicle, museum, museum_type, nudism, number_of_apartments, openfire, opening_hours, operator, parking, payment, permanent_camping, picnic_table, phone, power_supply, reservation, rooms, sanitary_dump_station, scout, shower, smoking, stars, start_date, static_caravans, subject, surface, swimming_pool, tents, toilets, washing_machine, waste_disposal, website, wheelchair, wikipedia, winter_room, zoo],
    Unclassified [],
    Water [basin, intermittent, lock, name, reservoir_type, salt, seasonal],
    Waterway [access, boat, canoe, cemt, depth, diameter, dock, draft, fuel, height, industrial, intermittent, layer, location, lock, maxheight, maxlength, maxspeed, maxwidth, motorboat, name, operator, salt, ship, tidal, tunnel, usage, width],
);

// Another option for sorting is to have add a "priority" int field to GeoTile types and sort by that int.
pub fn geotile_sort(a: &Arc<GeoTile>, b: &Arc<GeoTile>) -> Ordering {
    match a.as_ref() {
        // These geotiles should always be displayed no matter what.
        GeoTile::Amenity { amenity_type: AmenityType::ATM | // Only certain amenities have high priority.
                                         AmenityType::AmeBBQ |
                                         AmenityType::Bench |
                                         AmenityType::BicycleParking |
                                         AmenityType::BusStation |
                                         AmenityType::Clock |
                                         AmenityType::FerryTerminal |
                                         AmenityType::Firepit |
                                         AmenityType::Fountain |
                                         AmenityType::Fuel |
                                         AmenityType::GiveBox |
                                         AmenityType::HuntingStand |
                                         AmenityType::MotorcycleParking |
                                         AmenityType::PhotoBooth |
                                         AmenityType::PostBox |
                                         AmenityType::SanitaryDumpStation |
                                         AmenityType::Shower |
                                         AmenityType::Taxi |
                                         AmenityType::Telephone |
                                         AmenityType::Unclassified |
                                         AmenityType::VendingMachine |
                                         AmenityType::WasteBasket |
                                         AmenityType::WateringPlace |
                                         AmenityType::WaterPoint, .. } |
        GeoTile::Barrier { .. } |
        GeoTile::Highway { .. } |
        GeoTile::Leisure { leisure_type: LeisureType::Firepit |
                                         LeisureType::IceRink |
                                         LeisureType::PicnicTable |
                                         LeisureType::SwimmingPool, .. } |
        GeoTile::Place { .. } |
        GeoTile::Railway { .. } |
        GeoTile::Route { .. } |
        GeoTile::Unclassified { .. } => Ordering::Less,
        // 2nd highest priority for display.
        GeoTile::Building { .. } |
        GeoTile::Boundary { .. } |
        GeoTile::Craft { .. } |
        GeoTile::Emergency { .. } |
        GeoTile::Healthcare { .. } |
        GeoTile::Historic { .. } |
        GeoTile::ManMade { .. } |
        GeoTile::Military { .. } |
        GeoTile::Natural { .. } |
        GeoTile::Office { .. } |
        GeoTile::Power { .. } |
        GeoTile::PublicTransport { .. } |
        GeoTile::Shop { .. } |
        GeoTile::Sport { .. } |
        GeoTile::Telecom { .. } |
        GeoTile::Tourism { .. } => {
            match b.as_ref() {
                GeoTile::Aerialway { .. } |
                GeoTile::Aeroway { .. } |
                GeoTile::Amenity { amenity_type: AmenityType::AnimalShelter |
                                                 AmenityType::Casino |
                                                 AmenityType::Cinema |
                                                 AmenityType::College |
                                                 AmenityType::CommunityCentre |
                                                 AmenityType::ConferenceCentre |
                                                 AmenityType::DrivingSchool |
                                                 AmenityType::FoodCourt |
                                                 AmenityType::Hospital |
                                                 AmenityType::LanguageSchool |
                                                 AmenityType::MusicSchool |
                                                 AmenityType::Prison |
                                                 AmenityType::School |
                                                 AmenityType::SocialCentre |
                                                 AmenityType::SocialFacility |
                                                 AmenityType::Townhall |
                                                 AmenityType::University, .. } |
                GeoTile::Geological { .. } |
                GeoTile::Landuse { .. } |
                GeoTile::Leisure { .. } |
                GeoTile::Water { .. } |
                GeoTile::Waterway { .. } => Ordering::Less,
                _ => Ordering::Greater,
            }
        },
        // Everything else should always be displayed last.
        // GeoTile::Aerialway { .. } |
        // GeoTile::Aeroway { .. } |
        // GeoTile::Geological { .. } |
        // GeoTile::Landuse { .. } |
        // GeoTile::Leisure { .. } |
        // GeoTile::Water { .. } |
        // GeoTile::Waterway { .. } |
        _ => Ordering::Greater,
    }
}

// For now, same GeoTile variant means duplicate.
pub fn geotile_dedup(a: &mut Arc<GeoTile>, b: &mut Arc<GeoTile>) -> bool {
    std::mem::discriminant(a.as_ref()) == std::mem::discriminant(b.as_ref())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geotile_dedup() {
        let a = GeoTile::Building {
            osm_id: "osm_id_a".to_owned(),
            geometry: Geometry::Point(geo_types::Point::new(0.0, 0.0)),
            building_type: BuildingType::Apartments,
            address: None,
            access: None,
            amenity: None,
            capacity: None,
            covered: None,
            entrance: None,
            height: None,
            levels: None,
            name: Some("Building A".to_owned()),
            office: None,
            operator: None,
            power: None,
            public_transport: None,
            shop: None,
            sport: None,
        };
        let b = GeoTile::Building {
            osm_id: "osm_id_b".to_owned(),
            geometry: Geometry::Point(geo_types::Point::new(0.0, 0.0)),
            building_type: BuildingType::Apartments,
            address: None,
            access: None,
            amenity: None,
            capacity: None,
            covered: None,
            entrance: None,
            height: None,
            levels: None,
            name: Some("Building B".to_owned()),
            office: None,
            operator: None,
            power: None,
            public_transport: None,
            shop: None,
            sport: None,
        };
        let c = GeoTile::Water {
            osm_id: "osm_id_c".to_owned(),
            geometry: Geometry::Point(geo_types::Point::new(0.0, 0.0)),
            water_type: WaterType::Basin,
            address: None,
            basin: None,
            intermittent: None,
            lock: None,
            name: None,
            reservoir_type: None,
            salt: None,
            seasonal: None,
        };
        let mut vec = Vec::new();
        vec.push(Arc::new(a));
        vec.push(Arc::new(b));
        assert_eq!(vec.len(), 2);
        vec.dedup_by(geotile_dedup);
        assert_eq!(vec.len(), 1);
        vec.push(Arc::new(c));
        vec.dedup_by(geotile_dedup);
        assert_eq!(vec.len(), 2);
    }
}