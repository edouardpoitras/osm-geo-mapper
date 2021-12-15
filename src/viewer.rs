use std::{error::Error, io::Stdout, fmt};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Wrap},
    text::{Spans, Text},
    Terminal,
};

use crate::{
    nominatim, interface, operations, viewer::details::geo_tile_text_lines
};

mod actions;
mod input;
mod viewport;

pub mod cli;
pub mod details;
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

pub fn cli_options_to_mapper(options: cli::CLIOptions) -> Result<interface::OSMGeoMapper, Box<dyn std::error::Error>> {
    let radius: u32;
    if let Some(r) = options.radius {
        radius = r;
    } else {
        radius = 200;
    }
    if options.geojson_file.is_some() {
        let geojson_file = format!("{}", options.geojson_file.unwrap().to_str().unwrap());
        interface::OSMGeoMapper::from_geojson_file_with_radius(
            geojson_file,
            radius,
            Some(
                interface::Location::Coordinates {
                    latitude: options.latitude.unwrap(),
                    longitude: options.longitude.unwrap()
                }
            )
        )
    } else if options.pbf_file.is_some() {
        let pbf_file = format!("{}", options.pbf_file.unwrap().to_str().unwrap());
        interface::OSMGeoMapper::from_pbf_file(pbf_file, None)
    } else if options.latitude.is_some() && options.longitude.is_some() {
        let lat = options.latitude.unwrap();
        let lon = options.longitude.unwrap();
        let geojson_file = operations::get_geojson_file_by_lat_lon(lat, lon, operations::from_tile_scale(radius as i32))?;
        interface::OSMGeoMapper::from_geojson_file_with_radius(
            geojson_file,
            radius,
            Some(
                interface::Location::Coordinates {
                    latitude: lat,
                    longitude: lon
                }
            )
        )
    } else if options.address.is_some() {
        let (latitude, longitude) = nominatim::get_address_lat_lon(options.address.unwrap())?;
        interface::OSMGeoMapper::from_geojson_file_with_radius(
            operations::get_geojson_file_by_lat_lon(latitude, longitude, operations::from_tile_scale(radius as i32))?,
            radius,
            Some(
                interface::Location::Coordinates {
                    latitude,
                    longitude
                }
            )
        )
    } else {
        return Err(Box::new(MissingConfigurationError { message: "Need to provide one of pbf_file, geojson_file, latitude/longitude, or address (try --help)".to_string() }));
    }
}

pub fn run_crossterm(
    mut terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    options: cli::CLIOptions,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut mapper = cli_options_to_mapper(options)?;
    let mut zoom = 1;
    let mut loading = false;
    terminal.clear()?;
    // Display loop.
    loop {
        // Create our viewport with mapper data.
        let mut viewport = viewport::Viewport {
            data_structure: mapper.data_structure.clone(),
            coordinates: mapper.coordinates,
            zoom,
            loading,
        };
        // Redraw entities in terminal.
        terminal.draw(|mut f| {
            draw(&mut f, &mut viewport);
        })?;
        // Load more data if requested by user.
        if loading {
            let geojson_file = operations::get_geojson_file_by_lat_lon(
                operations::from_tile_scale(mapper.coordinates.y),
                operations::from_tile_scale(mapper.coordinates.x),
                operations::from_tile_scale(mapper.radius as i32)
            )?;
            let geojson = operations::parse_geojson_file(&geojson_file.to_string());
            operations::process_geojson_with_data_structure(&geojson, mapper.data_structure.clone());
            loading = false;
            continue; // Go back to drawing with new data.
        }
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
                    loading = true;
                }
                // Back to draw loop.
                break;
            }
            // Invalid/unrecognized user input, continue user input loop.
            continue;
        }
    }
}

fn draw(f: &mut tui::Frame<CrosstermBackend<Stdout>>, viewport: &mut viewport::Viewport) {
    // Get current geo tile text.
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .split(f.size());
    draw_left_panel(f, viewport, chunks[0]);
    f.render_widget(viewport, chunks[1]);
}

fn draw_left_panel(f: &mut tui::Frame<CrosstermBackend<Stdout>>, viewport: &viewport::Viewport, area: Rect) {
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)].as_ref())
        .split(area);
    draw_details_panel(f, viewport, chunks[0]);
    draw_info_panel(f, viewport, chunks[1]);
}

fn draw_details_panel(f: &mut tui::Frame<CrosstermBackend<Stdout>>, viewport: &viewport::Viewport, area: Rect) {
    let block = Block::default().title("Details").borders(Borders::ALL);
    let geo_tile_lines = geo_tile_text_lines(&viewport);
    let paragraph = Paragraph::new(geo_tile_lines)
        .block(block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .scroll((0, 0));
    f.render_widget(paragraph, area);
}

fn draw_info_panel(f: &mut tui::Frame<CrosstermBackend<Stdout>>, viewport: &viewport::Viewport, area: Rect) {
    let block = Block::default().title("Info").borders(Borders::ALL);
    let lines;
    if viewport.loading {
        lines = Text::from("Loading more data at current location...\n");
    } else {
        lines = Text::from(vec![
            Spans::from("Movement: <Up>, <Down>, <Left>, <Right>\n"),
            Spans::from("10x Movement: <Shift> + Movement Key\n"),
            Spans::from("Zoom In/Out: Z\n"),
            Spans::from("Load More Data: <Enter>\n"),
            Spans::from("Quit: Q\n")
        ]);
    }
    let paragraph = Paragraph::new(lines)
        .block(block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .scroll((0, 0));
    f.render_widget(paragraph, area);
}