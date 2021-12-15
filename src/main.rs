/*
 * A self-contained library for parsing geojson files and outputting the resulting geo features to a 2D tile data structure.
*/

use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use structopt::StructOpt;
#[cfg(debug_assertions)] use log4rs;

pub mod features;
pub mod geojson_parser;
pub mod nominatim;
pub mod openstreetmap;
pub mod operations;
pub mod osmtogeojson;
pub mod osm_parser;
pub mod pbf_parser;
pub mod viewer;
pub mod interface;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)] log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let opt = viewer::cli::CLIOptions::from_args();
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.hide_cursor().unwrap();
    viewer::run_crossterm(terminal, opt)
}