use std::{error::Error, fmt};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, Text},
    Terminal,
};

use crate::{
    nominatim, operations, viewer::details::geo_tile_text_lines
};

mod actions;
mod details;
mod input;
mod viewport;

pub mod cli;
pub mod theme;

#[derive(Debug)]
struct MissingConfigurationError {
    message: String
}
impl Error for MissingConfigurationError {}
impl fmt::Display for MissingConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub fn cli_options_to_mapper(options: cli::CLIOptions) -> Result<operations::Mapper, Box<dyn std::error::Error>> {
    let geojson_file: String;
    let lat: f64;
    let lon: f64;
    let radius: u32;
    if let Some(r) = options.radius {
        radius = r;
    } else {
        radius = 200;
    }
    if options.geojson_file.is_some() {
        lat = options.latitude.unwrap();
        lon = options.longitude.unwrap();
        geojson_file = format!("{:?}", options.geojson_file.unwrap());
    } else if options.latitude.is_some() && options.longitude.is_some() {
        lat = options.latitude.unwrap();
        lon = options.longitude.unwrap();
        geojson_file = operations::get_geojson_file_by_lat_lon(lat, lon, operations::from_tile_scale(radius as i32))?;
    } else if options.address.is_some() {
        let (latitude, longitude) =
            nominatim::get_address_lat_lon(options.address.unwrap())?;
        lat = latitude;
        lon = longitude;
        geojson_file = operations::get_geojson_file_by_lat_lon(lat, lon, operations::from_tile_scale(radius as i32))?;
    } else {
        return Err(Box::new(MissingConfigurationError { message: "Need to provide one of geojson_file, latitude/longitude, or address (try --help)".to_string() }));
    }
    operations::get_mapper_from_geojson_file(
        geojson_file,
        radius,
        Some(
            operations::Location::Coordinates {
                latitude: lat,
                longitude: lon
            }
        )
    )
}

pub fn run_crossterm(
    mut terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    options: cli::CLIOptions,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut mapper = cli_options_to_mapper(options)?;
    let mut zoom = 1;
    terminal.clear()?;
    // Display loop.
    loop {
        // Create our viewport with mapper data.
        let mut viewport = viewport::Viewport {
            data_structure: &mapper.data_structure,
            coordinates: mapper.coordinates,
            zoom,
        };
        // Redraw entities in terminal.
        terminal.draw(|mut f| {
            // Get current geo tile text.
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
                .split(f.size());
            let block = Block::default().title("Details").borders(Borders::ALL);
            let geo_tile_lines = geo_tile_text_lines(&viewport);
            let geo_tile_text_lines: Vec<Text> = geo_tile_lines.iter().map(Text::raw).collect();
            let paragraph = Paragraph::new(geo_tile_text_lines.iter())
                .block(block)
                .alignment(Alignment::Center)
                .wrap(true)
                .scroll(0);
            f.render_widget(paragraph, chunks[0]);
            f.render_widget(&mut viewport, chunks[1]);
        })?;
        // User input loop.
        loop {
            if let Some(input_action) = input::process_user_input(&mut viewport) {
                // Store our new coordinates in the mapper again and update our zoom level.
                mapper.coordinates = viewport.coordinates;
                zoom = viewport.zoom;
                // Valid user input received and processed.
                if input_action == actions::PlayerAction::Quit {
                    // User asking to quit.
                    println!("Exiting");
                    return Ok(());
                }
                // Check if we need to load more data
                else if input_action == actions::PlayerAction::LoadMoreData {
                    let geojson_file = operations::get_geojson_file_by_lat_lon(
                        operations::from_tile_scale(mapper.coordinates.y),
                        operations::from_tile_scale(mapper.coordinates.x),
                        operations::from_tile_scale(mapper.radius as i32)
                    )?;
                    let geojson = operations::parse_geojson_file(&geojson_file.to_string());
                    operations::process_geojson_with_data_structure(&geojson, &mut mapper.data_structure);
                }
                // Back to draw loop.
                break;
            }
            // Invalid/unrecognized user input, continue user input loop.
            continue;
        }
    }
}
