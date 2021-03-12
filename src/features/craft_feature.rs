use crate::{
    features::{CraftType, GeoTile, GeoTileProperties, Geometry},
    operations::{address_from_properties, property_to_option_string},
};
use osm_geo_mapper_macros::{ extract_type_from_string, geotile_from_properties };
use paste::paste; // Required for the extract_type_from_string macro.
use log::warn;

pub fn get_craft_geo_tile(props: &GeoTileProperties, geometry: Geometry) -> GeoTile {
    let craft_type_str = props["craft"].as_str().unwrap();
    let craft_type = extract_type_from_string!(craft_type_str<props> => CraftType [AgriculturalEngines, Atelier, Bakery, BasketMaker, Beekeeper, Blacksmith, Boatbuilder, Bookbinder, Brewery, Builder, CabinetMaker, CarPainter, Carpenter, CarpetLayer, Caterer, ChimneySweeper, Clockmaker, Confectionery, Cooper, DentalTechnician, Distillery, DoorConstruction, Dressmaker, ElectronicsRepair, Embroiderer, Electrician, Engraver, Floorer, Gardener, Glaziery, Goldsmith, GrindingMill, Handicraft, Hvac, Insulation, InteriorWork, Jeweller, Joiner, KeyCutter, Locksmith, MetalConstruction, Mint, MusicalInstrument, OilMill, Optician, OrganBuilder, Painter, ParquetLayer, Paver, Photographer, PhotographicLaboratory, PianoTuner, Plasterer, Plumber, Pottery, Printer, Printmaker, Rigger, Roofer, Saddler, Sailmaker, Sawmill, Scaffolder, Sculptor, Shoemaker, Signmaker, StandBuilder, Stonemason, SunProtection, Tailor, Tiler, Tinsmith, Toolmaker, Turner, Unclassified, Upholsterer, Watchmaker, WaterWellDrilling, WindowConstruction, Winery]);
    geotile_from_properties!(geometry<props> => Craft<craft_type> [builder, brand, carpenter, contact, distillery, electronics, electronics_repair, fax, healthcare, industrial, microbrewery, musical_instrument, name, opening_hours, operator, phone, produce, repair, studio, website, wheelchair]);
}