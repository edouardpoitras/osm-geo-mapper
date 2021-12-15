use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "osm-geo-mapper",
    about = "\nWill display OpenStreeMap data (PBF/OSM/GeoJSON) lines/points/polygons in the terminal."
)]
pub struct CLIOptions {
    #[structopt(
        short = "g",
        long = "geojson-file",
        parse(from_os_str),
        help = "Optionally provide a GeoJSON file directly to be parsed and displayed in the terminal"
    )]
    pub geojson_file: Option<PathBuf>,

    #[structopt(
        short = "o",
        long = "osm-file",
        parse(from_os_str),
        help = "Optionally provide a OSM file directly to be parsed and displayed in the terminal"
    )]
    pub osm_file: Option<PathBuf>,

    #[structopt(
        short = "p",
        long = "pbf-file",
        parse(from_os_str),
        help = "Optionally provide a PBF file directly to be parsed and displayed in the terminal"
    )]
    pub pbf_file: Option<PathBuf>,

    #[structopt(
        long = "latitude",
        requires("longitude"),
        help = "The latitude that will be used when fetching OpenStreetMap data (ignored if address or OSM/PBF/GeoJSON file is provided)"
    )]
    pub latitude: Option<f64>,

    #[structopt(
        long = "longitude",
        requires("latitude"),
        help = "The longitude that will be used when fetching OpenStreetMap data (ignored if address or OSM/PBF/GeoJSON file is provided)"
    )]
    pub longitude: Option<f64>,

    #[structopt(
        short = "a",
        long = "address",
        help = "The address that will be used when fetching OpenStreetMap data (ignored if OSM/PBF/GeoJSON file is provided)"
    )]
    pub address: Option<String>,

    #[structopt(
        short = "r",
        long = "radius",
        help = "The radius of the area of land to retrieve in 100,000th of a lat/lon degree (roughly a meter at the equator) - defaults to 200 (0.002 degrees or ~200m). Significantly impacts loading times. Ignored if PBF/GeoJSON file is provided"
    )]
    pub radius: Option<u32>,
}
