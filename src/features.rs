#![allow(clippy::write_with_newline)]

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

// You can find all features at https://wiki.openstreetmap.org/wiki/Map_Features
#[derive(Debug, Clone)]
pub enum GeoTile {
    Aerialway {
        aerialway_type: AerialwayType,
        geometry: Geometry,
        osm_id: String,
    },
    Aeroway {
        aeroway_type: AerowayType,
        description: Option<String>,
        geometry: Geometry,
        iata: Option<String>,
        icao: Option<String>,
        name: Option<String>,
        operator: Option<String>,
        osm_id: String,
        surface: Option<String>,
    },
    Amenity {
        access: Option<String>,
        amenity_type: AmenityType,
        amperage: Option<String>,
        backrest: Option<String>,
        beds: Option<String>,
        bottle: Option<String>,
        brand: Option<String>,
        brewery: Option<String>,
        building: Option<String>,
        capacity: Option<String>,
        cargo: Option<String>,
        colour: Option<String>,
        contact: Option<String>,
        covered: Option<String>,
        cuisine: Option<String>,
        date: Option<String>,
        delivery: Option<String>,
        denomination: Option<String>,
        description: Option<String>,
        diet: Option<String>,
        direction: Option<String>,
        drink: Option<String>,
        drinking_water: Option<String>,
        drive_through: Option<String>,
        emergency: Option<String>,
        fee: Option<String>,
        fuel: Option<String>,
        indoor: Option<String>,
        geometry: Geometry,
        lit: Option<String>,
        material: Option<String>,
        name: Option<String>,
        network: Option<String>,
        opening_hours: Option<String>,
        operator: Option<String>,
        osm_id: String,
        payment: Option<String>,
        phone: Option<String>,
        religion: Option<String>,
        seats: Option<String>,
        self_service: Option<String>,
        smoking: Option<String>,
        socket: Option<String>,
        voltage: Option<String>,
        website: Option<String>,
        wheelchair: Option<String>,
    },
    Barrier {
        access: Option<String>,
        barrier_type: BarrierType,
        bicycle: Option<String>,
        fee: Option<String>,
        foot: Option<String>,
        geometry: Geometry,
        two_sided: Option<String>,
        handrail: Option<String>,
        height: Option<String>,
        highway: Option<String>,
        historic: Option<String>,
        intermittent: Option<String>,
        lanes: Option<String>,
        locked: Option<String>,
        maxheight: Option<String>,
        maxwidth: Option<String>,
        motor_vehicle: Option<String>,
        operator: Option<String>,
        osm_id: String,
        wheelchair: Option<String>,
        width: Option<String>,
    },
    Boundary {
        admin_level: Option<String>,
        area: Option<String>,
        border_type: Option<String>,
        boundary_type: BoundaryType,
        description: Option<String>,
        format: Option<String>,
        geometry: Geometry,
        inscription: Option<String>,
        material: Option<String>,
        name: Option<String>,
        osm_id: String,
        political_division:  Option<String>,
        population:  Option<String>,
        postal_code: Option<String>,
        protect_class: Option<String>,
        protection_title: Option<String>,
    },
    Building {
        access: Option<String>,
        address: Option<Address>,
        amenity: Option<String>,
        building_type: BuildingType,
        capacity: Option<String>,
        covered: Option<String>,
        entrance: Option<String>,
        geometry: Geometry,
        height: Option<String>,
        levels: Option<String>,
        name: Option<String>,
        office: Option<String>,
        operator: Option<String>,
        osm_id: String,
        power: Option<String>,
        public_transport: Option<String>,
        shop: Option<String>,
        sport: Option<String>,
    },
    Craft {
        craft_type: CraftType,
        geometry: Geometry,
        osm_id: String,
    },
    Emergency {
        emergency_type: EmergencyType,
        geometry: Geometry,
        osm_id: String,
    },
    Geological {
        geological_type: GeologicalType,
        geometry: Geometry,
        osm_id: String,
    },
    Highway {
        abutters: Option<String>,
        access: Option<String>,
        bicycle: Option<String>,
        bus: Option<String>,
        destination: Option<String>,
        expressway: Option<String>,
        foot: Option<String>,
        geometry: Geometry,
        hgv: Option<String>,
        highway_type: HighwayType,
        lanes: Option<String>,
        lit: Option<String>,
        maxspeed: Option<String>,
        motor_vehicle: Option<String>,
        motorcar: Option<String>,
        motorroad: Option<String>,
        name: Option<String>,
        oneway: Option<String>,
        operator: Option<String>,
        osm_id: String,
        service: Option<String>,
        shelter: Option<String>,
        sidewalk: Option<String>,
        sport: Option<String>,
        smoothness: Option<String>,
        surface: Option<String>,
        tracktype: Option<String>,
        wheelchair: Option<String>,
        width: Option<String>,
    },
    Historic {
        historic_type: HistoricType,
        geometry: Geometry,
        osm_id: String,
    },
    Landuse {
        barrier: Option<String>,
        crop: Option<String>,
        denomination: Option<String>,
        genus: Option<String>,
        geometry: Geometry,
        industrial: Option<String>,
        landuse_type: LanduseType,
        leaf_cycle: Option<String>,
        leaf_type: Option<String>,
        meadow: Option<String>,
        name: Option<String>,
        operator: Option<String>,
        osm_id: String,
        plant: Option<String>,
        religion: Option<String>,
        resource: Option<String>,
        species: Option<String>,
        trees: Option<String>,
    },
    Leisure {
        access: Option<String>,
        barrier: Option<String>,
        building: Option<String>,
        covered: Option<String>,
        fee: Option<String>,
        geometry: Geometry,
        leisure_type: LeisureType,
        lit: Option<String>,
        name: Option<String>,
        osm_id: String,
        seasonal: Option<String>,
        shelter: Option<String>,
        sport: Option<String>,
        surface: Option<String>,
    },
    ManMade {
        access: Option<String>,
        bridge: Option<String>,
        capacity: Option<String>,
        color: Option<String>,
        content: Option<String>,
        country: Option<String>,
        covered: Option<String>,
        cutline: Option<String>,
        depth: Option<String>,
        direction: Option<String>,
        display: Option<String>,
        disused: Option<String>,
        drinking_water: Option<String>,
        elevation: Option<String>,
        floating: Option<String>,
        geometry: Geometry,
        height: Option<String>,
        headframe: Option<String>,
        inscription: Option<String>,
        layer: Option<String>,
        landuse: Option<String>,
        length: Option<String>,
        location: Option<String>,
        man_made_type: ManMadeType,
        material: Option<String>,
        mine: Option<String>,
        mineshaft_type: Option<String>,
        monitoring: Option<String>,
        mooring: Option<String>,
        name: Option<String>,
        operator: Option<String>,
        osm_id: String,
        oven: Option<String>,
        power: Option<String>,
        product: Option<String>,
        pump: Option<String>,
        pumping_station: Option<String>,
        resource: Option<String>,
        species: Option<String>,
        start_date: Option<String>,
        street_cabinet: Option<String>,
        submerged: Option<String>,
        substance: Option<String>,
        support: Option<String>,
        surveillance: Option<String>,
        survey_point: Option<String>,
        tidal: Option<String>,
        tourism: Option<String>,
        tunnel: Option<String>,
        width: Option<String>,
    },
    Military {
        military_type: MilitaryType,
        geometry: Geometry,
        osm_id: String,
    },
    Natural {
        access: Option<String>,
        circumference: Option<String>,
        denotation: Option<String>,
        direction: Option<String>,
        elevation: Option<String>,
        height: Option<String>,
        intermittent: Option<String>,
        genus: Option<String>,
        geometry: Geometry,
        leaf_type: Option<String>,
        leaf_cycle: Option<String>,
        managed: Option<String>,
        natural_type: NaturalType,
        name: Option<String>,
        operator: Option<String>,
        osm_id: String,
        salt: Option<String>,
        species: Option<String>,
        surface: Option<String>,
        taxon: Option<String>,
        width: Option<String>,
    },
    Office {
        office_type: OfficeType,
        geometry: Geometry,
        osm_id: String,
    },
    Place {
        admin_level: Option<String>,
        architect: Option<String>,
        capital: Option<String>,
        geometry: Geometry,
        is_in: Option<String>,
        name: Option<String>,
        osm_id: String,
        place_type: PlaceType,
        population: Option<String>,
        reference: Option<String>, // "ref"
        start_date: Option<String>,
        state_code: Option<String>,
    },
    Power {
        busbar: Option<String>,
        cables: Option<String>,
        circuits: Option<String>,
        colour: Option<String>,
        compensator: Option<String>,
        design: Option<String>,
        frequency: Option<String>,
        height: Option<String>,
        gas_insulated: Option<String>,
        geometry: Geometry,
        landuse: Option<String>,
        line: Option<String>,
        line_attachment: Option<String>,
        line_management: Option<String>,
        location: Option<String>,
        manufacturer: Option<String>,
        material: Option<String>,
        name: Option<String>,
        operator: Option<String>,
        osm_id: String,
        phases: Option<String>,
        poles: Option<String>,
        power_type: PowerType,
        start_date: Option<String>,
        structure: Option<String>,
        substation: Option<String>,
        switch: Option<String>,
        rating: Option<String>,
        voltage: Option<String>,
        windings: Option<String>,
        wires: Option<String>,
    },
    PublicTransport {
        aerialway: Option<String>,
        area: Option<String>,
        bench: Option<String>,
        bin: Option<String>,
        building: Option<String>,
        bus: Option<String>,
        covered: Option<String>,
        departures_board: Option<String>,
        ferry: Option<String>,
        geometry: Geometry,
        layer: Option<String>,
        level: Option<String>,
        local_ref: Option<String>,
        monorail: Option<String>,
        name: Option<String>,
        network: Option<String>,
        operator: Option<String>,
        osm_id: String,
        passenger_information_display: Option<String>,
        public_transport_type: PublicTransportType,
        shelter: Option<String>,
        subway: Option<String>,
        surface: Option<String>,
        tactile_paving: Option<String>,
        toilet: Option<String>,
        train: Option<String>,
        tram: Option<String>,
        trolleybus: Option<String>,
        uic_ref: Option<String>,
        uic_name: Option<String>,
        wheelchair: Option<String>,
    },
    Railway {
        railway_type: RailwayType,
        geometry: Geometry,
        osm_id: String,
    },
    Route {
        area: Option<String>,
        bicycle: Option<String>,
        colour: Option<String>,
        description: Option<String>,
        distance: Option<String>,
        duration: Option<String>,
        fee: Option<String>,
        foot: Option<String>,
        from: Option<String>,
        geometry: Geometry,
        lit: Option<String>,
        name: Option<String>,
        network: Option<String>,
        oneway: Option<String>,
        operator: Option<String>,
        osm_id: String,
        piste_difficulty: Option<String>,
        piste_type: Option<String>,
        roundtrip: Option<String>,
        route_type: RouteType,
        seasonal: Option<String>,
        symbol: Option<String>,
        to: Option<String>,
    },
    Shop {
        shop_type: ShopType,
        geometry: Geometry,
        osm_id: String,
    },
    Sport {
        sport_type: SportType,
        geometry: Geometry,
        osm_id: String,
    },
    Telecom {
        telecom_type: TelecomType,
        geometry: Geometry,
        osm_id: String,
    },
    Tourism {
        aerialway: Option<String>,
        access: Option<String>,
        address: Option<Address>,
        artist_name: Option<String>,
        artwork_subject: Option<String>,
        artwork_type: Option<String>,
        attraction: Option<String>,
        backcountry: Option<String>,
        balcony: Option<String>,
        bar: Option<String>,
        beds: Option<String>,
        bbq: Option<String>,
        brand: Option<String>,
        cabins: Option<String>,
        camp_site: Option<String>,
        capacity: Option<String>,
        caravans: Option<String>,
        contact: Option<String>,
        covered: Option<String>,
        description: Option<String>,
        dog: Option<String>,
        drinking_water: Option<String>,
        ele: Option<String>,
        electricity: Option<String>,
        email: Option<String>,
        exhibit: Option<String>,
        fee: Option<String>,
        fireplace: Option<String>,
        geometry: Geometry,
        group_only: Option<String>,
        heritage: Option<String>,
        hot_water: Option<String>,
        information: Option<String>,
        internet_access: Option<String>,
        kitchen: Option<String>,
        lit: Option<String>,
        material: Option<String>,
        mattress: Option<String>,
        motor_vehicle: Option<String>,
        museum: Option<String>,
        museum_type: Option<String>,
        name: Option<String>,
        nudism: Option<String>,
        number_of_apartments: Option<String>,
        openfire: Option<String>,
        opening_hours: Option<String>,
        operator: Option<String>,
        osm_id: String,
        parking: Option<String>,
        payment: Option<String>,
        permanent_camping: Option<String>,
        picnic_table: Option<String>,
        phone: Option<String>,
        power_supply: Option<String>,
        reservation: Option<String>,
        rooms: Option<String>,
        sanitary_dump_station: Option<String>,
        scout: Option<String>,
        shower: Option<String>,
        smoking: Option<String>,
        stars: Option<String>,
        start_date: Option<String>,
        static_caravans: Option<String>,
        subject: Option<String>,
        surface: Option<String>,
        swimming_pool: Option<String>,
        tents: Option<String>,
        toilets: Option<String>,
        tourism_type: TourismType,
        washing_machine: Option<String>,
        waste_disposal: Option<String>,
        website: Option<String>,
        wheelchair: Option<String>,
        wikipedia: Option<String>,
        winter_room: Option<String>,
        zoo: Option<String>,
    },
    Waterway {
        waterway_type: WaterwayType,
        geometry: Geometry,
        osm_id: String,
    },
    Unclassified {
        unclassified_type: UnclassifiedType,
        geometry: Geometry,
        osm_id: String,
    },
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

macro_rules! implement_display_for_geotile {
    ($($variant:ident<$type:ident>[$($attr:ident),*]),*$(,)*) => {
        impl fmt::Display for GeoTile {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $(
                        GeoTile::$variant {
                            $(
                                $attr,
                            )*
                            $type,
                            ..
                        } => {
                            let variant_str = stringify!($variant);
                            write!(f, "Feature: {}\n", variant_str)?;
                            write!(f, "Type: {:?}\n", $type)?;
                            print_geotile_attributes!(f => $($attr),*);
                            Ok(())
                        },
                    )*
                }
            }
        }
    }
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

implement_display_for_geotile!(
    Aerialway<aerialway_type>[],
    Aeroway<aeroway_type>[],
    Amenity<amenity_type>[name, access, amperage, backrest, beds, bottle, brand, brewery, building, capacity, cargo, colour, contact, covered, cuisine, date, delivery, denomination, description, diet, direction, drink, drinking_water, drive_through, emergency, fee, fuel, indoor, lit, material, network, opening_hours, operator, payment, phone, religion, seats, self_service, smoking, socket, voltage, website, wheelchair],
    Barrier<barrier_type>[],
    Boundary<boundary_type>[name, admin_level, area, border_type, description, format, inscription, material, political_division, population, postal_code, protect_class, protection_title],
    Building<building_type>[name, access, address, amenity, capacity, covered, entrance, height, levels, office, operator, power, public_transport, shop, sport],
    Craft<craft_type>[],
    Emergency<emergency_type>[],
    Geological<geological_type>[],
    Highway<highway_type>[name, abutters, access, bicycle, bus, destination, expressway, foot, hgv, lanes, lit, maxspeed, motor_vehicle, motorcar, motorroad, oneway, operator, service, shelter, sidewalk, sport, smoothness, surface, tracktype, wheelchair, width],
    Historic<historic_type>[],
    Landuse<landuse_type>[],
    Leisure<leisure_type>[],
    ManMade<man_made_type>[name, access, bridge, capacity, color, content, country, covered, cutline, depth, direction, display, disused, drinking_water, elevation, floating, height, headframe, inscription, layer, landuse, length, location, material, mine, mineshaft_type, monitoring, mooring, operator, oven, power, product, pump, pumping_station, resource, species, start_date, street_cabinet, submerged, substance, support, surveillance, survey_point, tidal, tourism, tunnel, width],
    Military<military_type>[],
    Natural<natural_type>[name, access, circumference, denotation, direction, elevation, height, intermittent, genus, leaf_type, leaf_cycle, managed, operator, salt, species, surface, taxon, width],
    Office<office_type>[],
    Place<place_type>[name, admin_level, architect, capital, is_in, population, reference, start_date, state_code],
    Power<power_type>[name, busbar, cables, circuits, colour, compensator, design, frequency, height, gas_insulated, landuse, line, line_attachment, line_management, location, manufacturer, material, operator, phases, poles, start_date, structure, substation, switch, rating, voltage, windings, wires],
    PublicTransport<public_transport_type>[name, aerialway, area, bench, bin, building, bus, covered, departures_board, ferry, layer, level, local_ref, monorail, network, operator, passenger_information_display, shelter, subway, surface, tactile_paving, toilet, train, tram, trolleybus, uic_ref, uic_name, wheelchair],
    Railway<railway_type>[],
    Route<route_type>[name, area, bicycle, colour, description, distance, duration, fee, foot, from, lit, network, oneway, operator, piste_difficulty, piste_type, roundtrip, seasonal, symbol, to],
    Shop<shop_type>[],
    Sport<sport_type>[],
    Telecom<telecom_type>[],
    Tourism<tourism_type>[name, aerialway, access, address, artist_name, artwork_subject, artwork_type, attraction, backcountry, balcony, bar, beds, bbq, brand, cabins, camp_site, capacity, caravans, contact, covered, description, dog, drinking_water, ele, electricity, email, exhibit, fee, fireplace, group_only, heritage, hot_water, information, internet_access, kitchen, lit, material, mattress, motor_vehicle, museum, museum_type, nudism, number_of_apartments, openfire, opening_hours, operator, parking, payment, permanent_camping, picnic_table, phone, power_supply, reservation, rooms, sanitary_dump_station, scout, shower, smoking, stars, start_date, static_caravans, subject, surface, swimming_pool, tents, toilets, washing_machine, waste_disposal, website, wheelchair, wikipedia, winter_room, zoo],
    Waterway<waterway_type>[],
    Unclassified<unclassified_type>[],
);