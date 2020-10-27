#![allow(clippy::write_with_newline)]

use geo_types as gt;
use serde_json::{Map, Value as JsonValue};
use std::collections::HashMap;
use std::{fmt, rc::Rc};

pub mod aeroway_feature;
pub mod amenity_feature;
pub mod barrier_feature;
pub mod boundary_feature;
pub mod building_feature;
pub mod highway_feature;
pub mod landuse_feature;
pub mod leisure_feature;
pub mod natural_feature;
pub mod route_feature;

pub const TILE_SCALE: f64 = 100_000.0;
pub type GeoTilesDataStructure = HashMap<gt::Coordinate<i32>, Rc<GeoTile>>;
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
        geometry: Geometry,
        osm_id: String,
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
        ele: Option<String>,
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
        bicycle: Option<String>,
        colour: Option<String>,
        description: Option<String>,
        distance: Option<String>,
        duration: Option<String>,
        foot: Option<String>,
        from: Option<String>,
        geometry: Geometry,
        name: Option<String>,
        network: Option<String>,
        oneway: Option<String>,
        operator: Option<String>,
        osm_id: String,
        roundtrip: Option<String>,
        route_type: RouteType,
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
    CableBarrier,
    CityWall,
    Ditch,
    Fence,
    GuardRail,
    Handrail,
    Hedge,
    Kerb,
    RetainingWall,
    Wall,
    Block,
    Bollard,
    BorderControl,
    BumpGate,
    BusTrap,
    CattleGrid,
    Chain,
    CycleBarrier,
    Debris,
    Entrance,
    FullHeightTurnstile,
    Gate,
    HampshireGate,
    HeightRestrictor,
    HorseStile,
    JerseyBarrier,
    KissingGate,
    LiftGate,
    Log,
    MotorcycleBarrier,
    Rope,
    SallyPort,
    Spikes,
    Stile,
    SumpBuster,
    SwingGate,
    TollBooth,
    Turnstile,
    Unclassified,
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
    Bungalow,
    Cabin,
    Detached,
    Dormitory,
    Farm,
    Ger,
    Hotel,
    House,
    Houseboat,
    Residential,
    SemidetachedHouse,
    StaticCaravan,
    Terrace,
    Commercial,
    Industrial,
    Kiosk,
    Office,
    Retail,
    Supermarket,
    Warehouse,
    Cathedral,
    Chapel,
    Church,
    Mosque,
    Religious,
    Shrine,
    Synagogue,
    Temple,
    Bakehouse,
    Civic,
    FireStation,
    Government,
    Hospital,
    Kindergarten,
    Public,
    School,
    PortableClassroom,
    Toilets,
    TrainStation,
    Transportation,
    University,
    Barn,
    Conservatory,
    Cowshed,
    FarmAuxiliary,
    Greenhouse,
    SlurryTank,
    Stable,
    Sty,
    Grandstand,
    Pavilion,
    RidingHall,
    SportsHall,
    Stadium,
    Hangar,
    Hut,
    Shed,
    Carport,
    Garage,
    Garages,
    Parking,
    Digester,
    Service,
    TransformerTower,
    WaterTower,
    Bunker,
    Bridge,
    Construction,
    Gatehouse,
    Roof,
    Ruins,
    TreeHouse,
    Unclassified,
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
pub enum RouteType {
    Bicycle,
    Bus,
    Canoe,
    Detour,
    Ferry,
    Foot,
    Hiking,
    Horse,
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
            GeoTile::ManMade { .. } => write!(f, "ManMade\n",),
            GeoTile::Military { .. } => write!(f, "Military\n",),
            GeoTile::Natural {
                access,
                circumference,
                denotation,
                direction,
                ele,
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
                if let Some(ele) = ele {
                    write!(f, "Elevation: {}\n", ele)?;
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
            GeoTile::Route { .. } => write!(f, "Route\n",),
            GeoTile::Shop { .. } => write!(f, "Shop\n",),
            GeoTile::Sport { .. } => write!(f, "Sport\n",),
            GeoTile::Telecom { .. } => write!(f, "Telecom\n",),
            GeoTile::Tourism { .. } => write!(f, "Tourism\n",),
            GeoTile::Waterway { .. } => write!(f, "Waterway\n",),
            GeoTile::Unclassified { .. } => write!(f, "Unclassified\n",),
        }
    }
}
