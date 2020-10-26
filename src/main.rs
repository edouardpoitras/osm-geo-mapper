/*
 * A self-contained library for parsing geojson files and outputting the resulting geo features to a 2D tile data structure.
*/

use std::io;
use std::path::PathBuf;
use structopt::StructOpt;
use tui::backend::CrosstermBackend;
use tui::Terminal;

pub mod features;
pub mod nominatim;
pub mod openstreetmap;
pub mod operations;
pub mod osmtogeojson;
pub mod viewer;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "osm-geo-mapper",
    about = "\nWill fetch OpenStreetMap data, convert to GeoJSON, and display the resulting lines/points/polygons in the terminal."
)]
pub struct CLIOptions {
    #[structopt(
        long = "show-landuse",
        help = "Display all landuse areas - can take a while and cover up overlapping features like buildings"
    )]
    landuse: bool,

    #[structopt(
        long = "show-leisure",
        help = "Display all leisure areas - can cover up overlapping features like buildings"
    )]
    leisure: bool,

    #[structopt(
        long = "show-amenities",
        help = "Display all amenities - can take a while and cover up overlapping features like buildings"
    )]
    amenity: bool,

    #[structopt(
        long = "show-boundaries",
        help = "Display all boundaries - can take a while and cover up overlapping features like roads"
    )]
    boundary: bool,

    #[structopt(
        short = "g",
        long = "geojson-file",
        parse(from_os_str),
        help = "Optionally provide a geojson file directly to be parsed and displayed in the terminal"
    )]
    geojson_file: Option<PathBuf>,

    #[structopt(
        long = "latitude",
        requires("longitude"),
        help = "The latitude that will be used when fetching OpenStreetMap data (ignored if address is provided)"
    )]
    latitude: Option<f64>,

    #[structopt(
        long = "longitude",
        requires("latitude"),
        help = "The longitude that will be used when fetching OpenStreetMap data (ignored if address is provided)"
    )]
    longitude: Option<f64>,

    #[structopt(
        short = "a",
        long = "address",
        help = "The address that will be used when fetching OpenStreetMap data"
    )]
    address: Option<String>,

    #[structopt(
        short = "s",
        long = "size",
        help = "The square area of land to display in degrees lat/lon - defaults to area of 0.002 latitude by 0.002 longitude. Significantly impacts loading times"
    )]
    size: Option<f64>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = CLIOptions::from_args();
    //let opt = CLIOptions {
        //amenity: false,
        //landuse: false,
        //leisure: false,
        //boundary: false,
        //geojson_file: None,
        //latitude: None,
        //longitude: None,
        //size: Some(0.002),
        //address: Some("ottawa ontario".to_string()),
    //};
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.hide_cursor().unwrap();
    viewer::run_crossterm(terminal, opt).await
}
