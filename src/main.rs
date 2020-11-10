/*
 * A self-contained library for parsing geojson files and outputting the resulting geo features to a 2D tile data structure.
*/

use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use structopt::StructOpt;

pub mod features;
pub mod nominatim;
pub mod openstreetmap;
pub mod operations;
pub mod osmtogeojson;
pub mod viewer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = viewer::cli::CLIOptions::from_args();
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.hide_cursor().unwrap();
    viewer::run_crossterm(terminal, opt)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test() {
        let opt = viewer::cli::CLIOptions {
            amenity: false,
            landuse: false,
            leisure: false,
            boundary: false,
            geojson_file: None,
            latitude: None,
            longitude: None,
            size: Some(0.0002),
            address: Some("ottawa ontario".to_string()),
        };
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.hide_cursor().unwrap();
        viewer::run_crossterm(terminal, opt);
    }
}