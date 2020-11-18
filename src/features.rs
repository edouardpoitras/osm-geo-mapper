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
pub mod route_feature;

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
        geometry: Geometry,
        osm_id: String,
    },
    Emergency {
        geometry: Geometry,
        osm_id: String,
    },
    Geological {
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
        historic: Option<String>,
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
        natural: Option<String>,
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
        geometry: Geometry,
        osm_id: String,
    },
    Place {
        geometry: Geometry,
        osm_id: String,
    },
    Power {
        geometry: Geometry,
        osm_id: String,
    },
    PublicTransport {
        geometry: Geometry,
        osm_id: String,
    },
    Railway {
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
        geometry: Geometry,
        osm_id: String,
    },
    Sport {
        geometry: Geometry,
        osm_id: String,
    },
    Telecom {
        geometry: Geometry,
        osm_id: String,
    },
    Tourism {
        geometry: Geometry,
        osm_id: String,
    },
    Waterway {
        geometry: Geometry,
        osm_id: String,
    },
    Unclassified {
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

#[derive(Debug, Clone, Copy)]
pub enum AerialwayType {
    CableCar,
    Gondola,
    // ...
}

#[derive(Debug, Clone, Copy)]
pub enum AerowayType {
    Aerodrome,
    Apron,
    Gate,
    Hangar,
    Helipad,
    Heliport,
    Navigationaid,
    Runway,
    Spaceport,
    Taxiway,
    Terminal,
    Windsock,
    Unclassified,
}

#[derive(Debug, Clone, Copy)]
pub enum AmenityType {
    Bar,
    AmeBBQ,
    Biergarten,
    Cafe,
    DrinkingWater,
    FastFood,
    FoodCourt,
    IceCream,
    Pub,
    Restaurant,
    College,
    DrivingSchool,
    Kindergarten,
    LanguageSchool,
    Library,
    ToyLibrary,
    MusicSchool,
    School,
    University,
    BicycleParking,
    BicycleRepairStation,
    BicycleRental,
    BoatRental,
    BoatSharing,
    BusStation,
    CarRental,
    CarSharing,
    CarWash,
    VehicleInspection,
    ChargingStation,
    FerryTerminal,
    Fuel,
    GritBin,
    MotorcycleParking,
    Parking,
    ParkingEntrance,
    ParkingSpace,
    Taxi,
    ATM,
    Bank,
    BureauDeChange,
    BabyHatch,
    Clinic,
    Dentist,
    Doctors,
    Hospital,
    NursingHome,
    Pharmacy,
    SocialFacility,
    Veterinary,
    ArtsCentre,
    Brothel,
    Casino,
    Cinema,
    CommunityCentre,
    Fountain,
    Gambling,
    Nightclub,
    Planetarium,
    PublicBookcase,
    SocialCentre,
    Stripclub,
    Studio,
    Swingerclub,
    Theatre,
    AnimalBoarding,
    AnimalShelter,
    BakingOven,
    Bench,
    Childcare,
    Clock,
    ConferenceCentre,
    Courthouse,
    Crematorium,
    DiveCentre,
    Embassy,
    FireStation,
    Firepit,
    GiveBox,
    GraveYard,
    Gym,
    HuntingStand,
    InternetCafe,
    Kitchen,
    KneippWaterCure,
    Marketplace,
    Monastery,
    PhotoBooth,
    PlaceOfWorship,
    Police,
    PostBox,
    PostDepot,
    PostOffice,
    Prison,
    PublicBath,
    PublicBuilding,
    RangerStation,
    Recycling,
    RefugeeSite,
    SanitaryDumpStation,
    Sauna,
    Shelter,
    Shower,
    Telephone,
    Toilets,
    Townhall,
    Unclassified,
    VendingMachine,
    WasteBasket,
    WasteDisposal,
    WasteTransferStation,
    WateringPlace,
    WaterPoint,
}

#[derive(Debug, Clone, Copy)]
pub enum BarrierType {
    Block,
    Bollard,
    BorderControl,
    BumpGate,
    BusTrap,
    CableBarrier,
    CattleGrid,
    Chain,
    CityWall,
    CycleBarrier,
    Debris,
    Ditch,
    Entrance,
    Fence,
    FullHeightTurnstile,
    Gate,
    GuardRail,
    HampshireGate,
    Handrail,
    Hedge,
    HeightRestrictor,
    HorseStile,
    JerseyBarrier,
    Kerb,
    KissingGate,
    LiftGate,
    Log,
    MotorcycleBarrier,
    RetainingWall,
    Rope,
    SallyPort,
    Spikes,
    Stile,
    SumpBuster,
    SwingGate,
    TollBooth,
    Turnstile,
    Unclassified,
    Wall,
}

#[derive(Debug, Clone, Copy)]
pub enum BoundaryType {
    AboriginalLands,
    Administrative,
    Maritime,
    Marker,
    NationalPark,
    Political,
    PostalCode,
    ProtectedArea,
    UserDefined,
    Unclassified,
}

#[derive(Debug, Clone, Copy)]
pub enum BuildingType {
    Apartments,
    Bakehouse,
    Barn,
    Bridge,
    Building, // Not sure how else to handle properties of "building: yes"
    Bunker,
    Bungalow,
    Cabin,
    Carport,
    Cathedral,
    Chapel,
    Church,
    Civic,
    Commercial,
    Conservatory,
    Construction,
    Cowshed,
    Detached,
    Digester,
    Dormitory,
    Farm,
    FarmAuxiliary,
    FireStation,
    Garage,
    Garages,
    Gatehouse,
    Ger,
    Government,
    Grandstand,
    Greenhouse,
    Hangar,
    Hospital,
    Hotel,
    House,
    Houseboat,
    Hut,
    Industrial,
    Kindergarten,
    Kiosk,
    Mosque,
    Office,
    Parking,
    Pavilion,
    PortableClassroom,
    Public,
    Religious,
    Residential,
    Retail,
    RidingHall,
    Roof,
    Ruins,
    School,
    SemidetachedHouse,
    Service,
    Shed,
    Shrine,
    SportsHall,
    SlurryTank,
    Stable,
    Stadium,
    StaticCaravan,
    Sty,
    Supermarket,
    Synagogue,
    Temple,
    Terrace,
    Toilets,
    TrainStation,
    TransformerTower,
    Transportation,
    TreeHouse,
    Unclassified,
    University,
    Warehouse,
    WaterTower,
}

#[derive(Debug, Clone, Copy)]
pub enum NaturalType {
    Wood,
    TreeRow,
    Tree,
    Scrub,
    Heath,
    Moor,
    Grassland,
    Fell,
    BareRock,
    Scree,
    Shingle,
    Sand,
    Mud,
    Water,
    Wetland,
    Glacier,
    Bay,
    Strait,
    Cape,
    Beach,
    Coastline,
    Reef,
    Spring,
    HotSpring,
    Geyser,
    Blowhole,
    Peak,
    Volcano,
    Valley,
    Peninsula,
    Isthmus,
    Ridge,
    Arete,
    Cliff,
    Saddle,
    Dune,
    Rock,
    Stone,
    Sinkhole,
    CaveEntrance,
    Unclassified,
}

#[derive(Debug, Clone, Copy)]
pub enum HighwayType {
    Motorway,
    MotorwayLink,
    Trunk,
    TrunkLink,
    Primary,
    PrimaryLink,
    Secondary,
    SecondaryLink,
    Tertiary,
    TertiaryLink,
    Residential,
    LivingStreet,
    Service,
    Pedestrian,
    Track,
    BusGuideway,
    Escape,
    Raceway,
    Road,
    Footway,
    Bridleway,
    Steps,
    Corridor,
    Path,
    Cycleway,
    Proposed,
    Construction,
    Unclassified,
}

#[derive(Debug, Clone, Copy)]
pub enum LanduseType {
    Allotments,
    Basin,
    Brownfield,
    Cemetery,
    Commercial,
    Conservation,
    Construction,
    Depot,
    Farmland,
    Farmyard,
    Flowerbed,
    Forest,
    Garages,
    Grass,
    Greenfield,
    GreenhouseHorticulture,
    Industrial,
    Landfill,
    Meadow,
    Military,
    Orchard,
    PeatCutting,
    PlantNursery,
    Port,
    Quarry,
    Railway,
    RecreationGround,
    Religious,
    Reservoir,
    Residential,
    Retail,
    SaltPond,
    Unclassified,
    VillageGreen,
    Vineyard,
}

#[derive(Debug, Clone, Copy)]
pub enum LeisureType {
    AdultGamingCentre,
    AmusementArcade,
    BeachResort,
    Bandstand,
    BirdHide,
    Common,
    Dance,
    DiscGolfCourse,
    DogPark,
    EscapeGame,
    Firepit,
    Fishing,
    FitnessCentre,
    FitnessStation,
    Garden,
    Hackerspace,
    HorseRiding,
    IceRink,
    Marina,
    MiniatureGolf,
    NatureReserve,
    Park,
    PicnicTable,
    Pitch,
    Playground,
    Slipway,
    SportsCentre,
    Stadium,
    SummerCamp,
    SwimmingArea,
    SwimmingPool,
    Track,
    Unclassified,
    WaterPark,
}

#[derive(Debug, Clone, Copy)]
pub enum ManMadeType {
    Adit,
    Beacon,
    Breakwater,
    Bridge,
    BunkerSilo,
    CarpetHanger,
    Chimney,
    CommunicationsTower,
    Crane,
    Cross,
    Cutline,
    Clearcut,
    Dovecote,
    Dyke,
    Embankment,
    Flagpole,
    Gasometer,
    GoodsConveyor,
    Groyne,
    Kiln,
    Lighthouse,
    Mast,
    Mineshaft,
    MonitoringStation,
    Obelisk,
    Observatory,
    OffshorePlatform,
    PetroleumWell,
    Pier,
    Pipeline,
    PumpingStation,
    ReservoirCovered,
    Silo,
    SnowFence,
    SnowNet,
    StorageTank,
    StreetCabinet,
    Surveillance,
    SurveyPoint,
    Telescope,
    Tower,
    Unclassified,
    WastewaterPlant,
    Watermill,
    WaterTower,
    WaterWell,
    WaterTap,
    WaterWorks,
    WildlifeCrossing,
    Windmill,
    Works,
}

#[derive(Debug, Clone, Copy)]
pub enum RouteType {
    Bicycle,
    Bus,
    Canoe,
    Detour,
    Ferry,
    Foot,
    Hiking,
    Horse,
    IceSkate,
    InlineSkates,
    LightRail,
    MTB,
    Piste,
    Power,
    Railway,
    Road,
    Running,
    Ski,
    Subway,
    Train,
    Tracks,
    Tram,
    Trolleybus,
    Unclassified,
}

impl fmt::Display for GeoTile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GeoTile::Aerialway { .. } => write!(f, "Aerialway\n"),
            GeoTile::Aeroway { .. } => write!(f, "Aeroway\n"),
            GeoTile::Amenity { .. } => write!(f, "Amenity\n",),
            GeoTile::Barrier { .. } => write!(f, "Barrier\n",),
            GeoTile::Boundary {
                admin_level,
                area,
                border_type,
                boundary_type,
                description,
                format,
                geometry: _,
                inscription,
                material,
                name,
                osm_id: _,
                political_division,
                population,
                postal_code,
                protect_class,
                protection_title,
            } => {
                write!(f, "Feature: Boundary\n")?;
                write!(f, "Type: {:?}\n", boundary_type)?;
                if let Some(admin_level) = admin_level {
                    write!(f, "Admin Level: {}\n", admin_level)?;
                }
                if let Some(area) = area {
                    write!(f, "Area: {}\n", area)?;
                }
                if let Some(border_type) = border_type {
                    write!(f, "Border Type: {}\n", border_type)?;
                }
                if let Some(description) = description {
                    write!(f, "Description: {}\n", description)?;
                }
                if let Some(format) = format {
                    write!(f, "Format: {}\n", format)?;
                }
                if let Some(inscription) = inscription {
                    write!(f, "Inscription: {}\n", inscription)?;
                }
                if let Some(material) = material {
                    write!(f, "Material: {}\n", material)?;
                }
                if let Some(name) = name {
                    write!(f, "Name: {}\n", name)?;
                }
                if let Some(political_division) = political_division{
                    write!(f, "Political Division: {}\n", political_division)?;
                }
                if let Some(population) = population {
                    write!(f, "Population: {}\n", population)?;
                }
                if let Some(postal_code) = postal_code {
                    write!(f, "Postal Code: {}\n", postal_code)?;
                }
                if let Some(protect_class) = protect_class {
                    write!(f, "Protect Class: {}\n", protect_class)?;
                }
                if let Some(protection_title) = protection_title{
                    write!(f, "Protection Title: {}\n", protection_title)?;
                }
                Ok(())
            }
            GeoTile::Building {
                access,
                address,
                amenity,
                building_type,
                capacity,
                covered,
                entrance,
                height,
                levels,
                name,
                office,
                operator,
                power,
                public_transport,
                shop,
                sport,
                geometry: _,
                osm_id: _,
            } => {
                write!(f, "Feature: Building\n")?;
                write!(f, "Type: {:?}\n", building_type)?;
                if let Some(name) = name {
                    write!(f, "Name: {}\n", name)?;
                }
                if let Some(access) = access {
                    write!(f, "Access: {}\n", access)?;
                }
                if let Some(address) = address {
                    write!(f, "Address: {}\n", address)?;
                }
                if let Some(amenity) = amenity {
                    write!(f, "Amenity: {}\n", amenity)?;
                }
                if let Some(capacity) = capacity {
                    write!(f, "Capacity: {}\n", capacity)?;
                }
                if let Some(covered) = covered {
                    write!(f, "Covered: {}\n", covered)?;
                }
                if let Some(entrance) = entrance {
                    write!(f, "Entrance: {}\n", entrance)?;
                }
                if let Some(height) = height {
                    write!(f, "Height: {}\n", height)?;
                }
                if let Some(levels) = levels {
                    write!(f, "Levels: {}\n", levels)?;
                }
                if let Some(office) = office {
                    write!(f, "Office: {}\n", office)?;
                }
                if let Some(operator) = operator {
                    write!(f, "Operator: {}\n", operator)?;
                }
                if let Some(power) = power {
                    write!(f, "Power: {}\n", power)?;
                }
                if let Some(public_transport) = public_transport {
                    write!(f, "Public Transport: {}\n", public_transport)?;
                }
                if let Some(shop) = shop {
                    write!(f, "Shop: {}\n", shop)?;
                }
                if let Some(sport) = sport {
                    write!(f, "Sport: {}\n", sport)?;
                }
                Ok(())
            }
            GeoTile::Craft { .. } => write!(f, "Craft\n",),
            GeoTile::Emergency { .. } => write!(f, "Emergency\n",),
            GeoTile::Geological { .. } => write!(f, "Geological\n",),
            GeoTile::Highway {
                abutters,
                access,
                bicycle,
                bus,
                destination,
                expressway,
                foot,
                hgv,
                highway_type,
                lanes,
                lit,
                maxspeed,
                motor_vehicle,
                motorcar,
                motorroad,
                name,
                oneway,
                operator,
                service,
                shelter,
                sidewalk,
                sport,
                smoothness,
                surface,
                tracktype,
                wheelchair,
                width,
                geometry: _,
                osm_id: _,
            } => {
                write!(f, "Feature: Highway\n")?;
                write!(f, "Type: {:?}\n", highway_type)?;
                if let Some(name) = name {
                    write!(f, "Name: {}\n", name)?;
                }
                if let Some(abutters) = abutters {
                    write!(f, "Abutters: {}\n", abutters)?;
                }
                if let Some(access) = access {
                    write!(f, "Access: {}\n", access)?;
                }
                if let Some(bicycle) = bicycle {
                    write!(f, "Bicycle: {}\n", bicycle)?;
                }
                if let Some(bus) = bus {
                    write!(f, "Bus: {}\n", bus)?;
                }
                if let Some(destination) = destination {
                    write!(f, "Destination: {}\n", destination)?;
                }
                if let Some(expressway) = expressway {
                    write!(f, "Expressway: {}\n", expressway)?;
                }
                if let Some(foot) = foot {
                    write!(f, "Foot: {}\n", foot)?;
                }
                if let Some(hgv) = hgv {
                    write!(f, "Heavy Goods Vehicles: {}\n", hgv)?;
                }
                if let Some(lanes) = lanes {
                    write!(f, "Lanes: {}\n", lanes)?;
                }
                if let Some(lit) = lit {
                    write!(f, "Lit: {}\n", lit)?;
                }
                if let Some(maxspeed) = maxspeed {
                    write!(f, "Max Speed: {}\n", maxspeed)?;
                }
                if let Some(motor_vehicle) = motor_vehicle {
                    write!(f, "Motor Vehicle: {}\n", motor_vehicle)?;
                }
                if let Some(motorcar) = motorcar {
                    write!(f, "Motorcar: {}\n", motorcar)?;
                }
                if let Some(motorroad) = motorroad {
                    write!(f, "Motorroad: {}\n", motorroad)?;
                }
                if let Some(oneway) = oneway {
                    write!(f, "Oneway: {}\n", oneway)?;
                }
                if let Some(operator) = operator {
                    write!(f, "Operator: {}\n", operator)?;
                }
                if let Some(service) = service {
                    write!(f, "Service: {}\n", service)?;
                }
                if let Some(shelter) = shelter {
                    write!(f, "Shelter: {}\n", shelter)?;
                }
                if let Some(sidewalk) = sidewalk {
                    write!(f, "Sidewalk: {}\n", sidewalk)?;
                }
                if let Some(sport) = sport {
                    write!(f, "Sport: {}\n", sport)?;
                }
                if let Some(smoothness) = smoothness {
                    write!(f, "Smoothness: {}\n", smoothness)?;
                }
                if let Some(surface) = surface {
                    write!(f, "Surface: {}\n", surface)?;
                }
                if let Some(tracktype) = tracktype {
                    write!(f, "Tracktype: {}\n", tracktype)?;
                }
                if let Some(wheelchair) = wheelchair {
                    write!(f, "Wheelchair: {}\n", wheelchair)?;
                }
                if let Some(width) = width {
                    write!(f, "Width: {}\n", width)?;
                }
                Ok(())
            }
            GeoTile::Historic { .. } => write!(f, "Historic\n",),
            GeoTile::Landuse { .. } => write!(f, "Landuse\n",),
            GeoTile::Leisure { .. } => write!(f, "Leisure\n",),
            GeoTile::ManMade {
                access,
                bridge,
                capacity,
                color,
                content,
                country,
                covered,
                cutline,
                depth,
                direction,
                display,
                disused,
                drinking_water,
                elevation,
                floating,
                geometry: _,
                height,
                headframe,
                historic,
                inscription,
                layer,
                landuse,
                length,
                location,
                man_made_type,
                material,
                mine,
                mineshaft_type,
                monitoring,
                mooring,
                natural,
                name,
                operator,
                osm_id: _,
                oven,
                power,
                product,
                pump,
                pumping_station,
                resource,
                species,
                start_date,
                street_cabinet,
                submerged,
                substance,
                support,
                surveillance,
                survey_point,
                tidal,
                tourism,
                tunnel,
                width,
            } => {
                write!(f, "Feature: Man Made\n")?;
                write!(f, "Type: {:?}\n", man_made_type)?;
                if let Some(name) = name {
                    write!(f, "Name: {}\n", name)?;
                }
                if let Some(access) = access {
                    write!(f, "Access: {}\n", access)?;
                }
                if let Some(bridge) = bridge {
                    write!(f, "Bridge: {}\n", bridge)?;
                }
                if let Some(capacity) = capacity {
                    write!(f, "Capacity: {}\n", capacity)?;
                }
                if let Some(color) = color {
                    write!(f, "Color: {}\n", color)?;
                }
                if let Some(content) = content {
                    write!(f, "Content: {}\n", content)?;
                }
                if let Some(country) = country {
                    write!(f, "Country: {}\n", country)?;
                }
                if let Some(covered) = covered {
                    write!(f, "Covered: {}\n", covered)?;
                }
                if let Some(cutline) = cutline {
                    write!(f, "Cutline: {}\n", cutline)?;
                }
                if let Some(depth) = depth {
                    write!(f, "Depth: {}\n", depth)?;
                }
                if let Some(direction) = direction {
                    write!(f, "Direction: {}\n", direction)?;
                }
                if let Some(display) = display {
                    write!(f, "Display: {}\n", display)?;
                }
                if let Some(disused) = disused {
                    write!(f, "Disused: {}\n", disused)?;
                }
                if let Some(drinking_water) = drinking_water {
                    write!(f, "Drinking Water: {}\n", drinking_water)?;
                }
                if let Some(elevation) = elevation {
                    write!(f, "Elevation: {}\n", elevation)?;
                }
                if let Some(floating) = floating {
                    write!(f, "Floating: {}\n", floating)?;
                }
                if let Some(height) = height {
                    write!(f, "Height: {}\n", height)?;
                }
                if let Some(headframe) = headframe {
                    write!(f, "Headframe: {}\n", headframe)?;
                }
                if let Some(historic) = historic {
                    write!(f, "Historic: {}\n", historic)?;
                }
                if let Some(inscription) = inscription {
                    write!(f, "Inscription: {}\n", inscription)?;
                }
                if let Some(layer) = layer {
                    write!(f, "Layer: {}\n", layer)?;
                }
                if let Some(landuse) = landuse {
                    write!(f, "Landuse: {}\n", landuse)?;
                }
                if let Some(length) = length {
                    write!(f, "Length: {}\n", length)?;
                }
                if let Some(location) = location {
                    write!(f, "Location: {}\n", location)?;
                }
                if let Some(material) = material {
                    write!(f, "Material: {}\n", material)?;
                }
                if let Some(mine) = mine {
                    write!(f, "Mine: {}\n", mine)?;
                }
                if let Some(mineshaft_type) = mineshaft_type {
                    write!(f, "Mineshaft Type: {}\n", mineshaft_type)?;
                }
                if let Some(monitoring) = monitoring {
                    write!(f, "Monitoring: {}\n", monitoring)?;
                }
                if let Some(mooring) = mooring {
                    write!(f, "Mooring: {}\n", mooring)?;
                }
                if let Some(natural) = natural {
                    write!(f, "Natural: {}\n", natural)?;
                }
                if let Some(operator) = operator {
                    write!(f, "Operator: {}\n", operator)?;
                }
                if let Some(oven) = oven {
                    write!(f, "Oven: {}\n", oven)?;
                }
                if let Some(power) = power {
                    write!(f, "Power: {}\n", power)?;
                }
                if let Some(product) = product {
                    write!(f, "Product: {}\n", product)?;
                }
                if let Some(pump) = pump {
                    write!(f, "Pump: {}\n", pump)?;
                }
                if let Some(pumping_station) = pumping_station {
                    write!(f, "Pumping Station: {}\n", pumping_station)?;
                }
                if let Some(resource) = resource {
                    write!(f, "Resource: {}\n", resource)?;
                }
                if let Some(species) = species {
                    write!(f, "Species: {}\n", species)?;
                }
                if let Some(start_date) = start_date {
                    write!(f, "Start Date: {}\n", start_date)?;
                }
                if let Some(street_cabinet) = street_cabinet {
                    write!(f, "Street Cabinet: {}\n", street_cabinet)?;
                }
                if let Some(submerged) = submerged {
                    write!(f, "Submerged: {}\n", submerged)?;
                }
                if let Some(substance) = substance {
                    write!(f, "Substance: {}\n", substance)?;
                }
                if let Some(support) = support {
                    write!(f, "Support: {}\n", support)?;
                }
                if let Some(surveillance) = surveillance {
                    write!(f, "Surveillance: {}\n", surveillance)?;
                }
                if let Some(survey_point) = survey_point {
                    write!(f, "Survey Point: {}\n", survey_point)?;
                }
                if let Some(tidal) = tidal {
                    write!(f, "Tidal: {}\n", tidal)?;
                }
                if let Some(tourism) = tourism {
                    write!(f, "Tourism: {}\n", tourism)?;
                }
                if let Some(tunnel) = tunnel {
                    write!(f, "Tunnel: {}\n", tunnel)?;
                }
                if let Some(width) = width {
                    write!(f, "Width: {}\n", width)?;
                }
                Ok(())
            },
            GeoTile::Military { .. } => write!(f, "Military\n",),
            GeoTile::Natural {
                access,
                circumference,
                denotation,
                direction,
                elevation,
                height,
                intermittent,
                genus,
                leaf_type,
                leaf_cycle,
                managed,
                natural_type,
                name,
                operator,
                salt,
                species,
                surface,
                taxon,
                width,
                geometry: _,
                osm_id: _,
            } => {
                write!(f, "Feature: Natural\n")?;
                write!(f, "Type: {:?}\n", natural_type)?;
                if let Some(name) = name {
                    write!(f, "Name: {}\n", name)?;
                }
                if let Some(access) = access {
                    write!(f, "Access: {}\n", access)?;
                }
                if let Some(circumference) = circumference {
                    write!(f, "Circumference: {}\n", circumference)?;
                }
                if let Some(denotation) = denotation {
                    write!(f, "Denotation: {}\n", denotation)?;
                }
                if let Some(direction) = direction {
                    write!(f, "Direction: {}\n", direction)?;
                }
                if let Some(elevation) = elevation {
                    write!(f, "Elevation: {}\n", elevation)?;
                }
                if let Some(height) = height {
                    write!(f, "Height: {}\n", height)?;
                }
                if let Some(width) = width {
                    write!(f, "Width: {}\n", width)?;
                }
                if let Some(intermittent) = intermittent {
                    write!(f, "Intermittent: {}\n", intermittent)?;
                }
                if let Some(genus) = genus {
                    write!(f, "Genus: {}\n", genus)?;
                }
                if let Some(leaf_type) = leaf_type {
                    write!(f, "Leaf Type: {}\n", leaf_type)?;
                }
                if let Some(leaf_cycle) = leaf_cycle {
                    write!(f, "Leaf Cycle: {}\n", leaf_cycle)?;
                }
                if let Some(managed) = managed {
                    write!(f, "Managed: {}\n", managed)?;
                }
                if let Some(operator) = operator {
                    write!(f, "Operator: {}\n", operator)?;
                }
                if let Some(salt) = salt {
                    write!(f, "Salt: {}\n", salt)?;
                }
                if let Some(species) = species {
                    write!(f, "Species: {}\n", species)?;
                }
                if let Some(surface) = surface {
                    write!(f, "Surface: {}\n", surface)?;
                }
                if let Some(taxon) = taxon {
                    write!(f, "Taxon: {}\n", taxon)?;
                }
                Ok(())
            }
            GeoTile::Office { .. } => write!(f, "Office\n",),
            GeoTile::Place { .. } => write!(f, "Place\n",),
            GeoTile::Power { .. } => write!(f, "Power\n",),
            GeoTile::PublicTransport { .. } => write!(f, "PublicTransport\n",),
            GeoTile::Railway { .. } => write!(f, "Railway\n",),
            GeoTile::Route {
                area,
                bicycle,
                colour,
                description,
                distance,
                duration,
                fee,
                foot,
                from,
                lit,
                name,
                network,
                oneway,
                operator,
                piste_difficulty,
                piste_type,
                roundtrip,
                route_type,
                seasonal,
                symbol,
                to,
                geometry: _,
                osm_id: _,
            } => {
                write!(f, "Feature: Route\n")?;
                write!(f, "Type: {:?}\n", route_type)?;
                if let Some(name) = name {
                    write!(f, "Name: {}\n", name)?;
                }
                if let Some(area) = area {
                    write!(f, "Area: {}\n", area)?;
                }
                if let Some(bicycle) = bicycle {
                    write!(f, "Bicycle: {}\n", bicycle)?;
                }
                if let Some(colour) = colour {
                    write!(f, "Colour: {}\n", colour)?;
                }
                if let Some(description) = description {
                    write!(f, "Description: {}\n", description)?;
                }
                if let Some(distance) = distance {
                    write!(f, "Distance: {}\n", distance)?;
                }
                if let Some(duration) = duration {
                    write!(f, "Duration: {}\n", duration)?;
                }
                if let Some(fee) = fee {
                    write!(f, "Fee: {}\n", fee)?;
                }
                if let Some(foot) = foot {
                    write!(f, "Foot: {}\n", foot)?;
                }
                if let Some(from) = from {
                    write!(f, "From: {}\n", from)?;
                }
                if let Some(to) = to {
                    write!(f, "To: {}\n", to)?;
                }
                if let Some(lit) = lit {
                    write!(f, "Lit: {}\n", lit)?;
                }
                if let Some(network) = network {
                    write!(f, "Network: {}\n", network)?;
                }
                if let Some(oneway) = oneway {
                    write!(f, "Oneway: {}\n", oneway)?;
                }
                if let Some(operator) = operator {
                    write!(f, "Operator: {}\n", operator)?;
                }
                if let Some(piste_type) = piste_type {
                    write!(f, "Piste Type: {}\n", piste_type)?;
                }
                if let Some(piste_difficulty) = piste_difficulty {
                    write!(f, "Piste Difficulty: {}\n", piste_difficulty)?;
                }
                if let Some(roundtrip) = roundtrip {
                    write!(f, "Roundtrip: {}\n", roundtrip)?;
                }
                if let Some(seasonal) = seasonal {
                    write!(f, "Seasonal: {}\n", seasonal)?;
                }
                if let Some(symbol) = symbol {
                    write!(f, "Symbol: {}\n", symbol)?;
                }
                Ok(())
            }
            GeoTile::Shop { .. } => write!(f, "Shop\n",),
            GeoTile::Sport { .. } => write!(f, "Sport\n",),
            GeoTile::Telecom { .. } => write!(f, "Telecom\n",),
            GeoTile::Tourism { .. } => write!(f, "Tourism\n",),
            GeoTile::Waterway { .. } => write!(f, "Waterway\n",),
            GeoTile::Unclassified { .. } => write!(f, "Unclassified\n",),
        }
    }
}
