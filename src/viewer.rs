use geo_types as gt;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, Text},
    Terminal,
};

use crate::{
    features::TILE_SCALE, nominatim, openstreetmap, operations, osmtogeojson,
    viewer::details::geo_tile_text_lines, CLIOptions,
};

mod actions;
mod details;
mod input;
mod theme;
mod viewport;

async fn get_geojson_file_by_lat_lon(
    lat: f64,
    lon: f64,
    size: Option<f64>,
) -> Result<String, Box<dyn std::error::Error>> {
    let sizef = size.unwrap_or(0.005);
    let left = lon - sizef;
    let bottom = lat - sizef;
    let right = lon + sizef;
    let top = lat + sizef;
    let osm_file = openstreetmap::download_osm_data_by_bbox(left, bottom, right, top).await?;
    let geojson_file = format!("{}.geojson", osm_file);
    osmtogeojson::convert_osm_to_geojson(osm_file, geojson_file.clone())?;
    Ok(geojson_file)
}

pub async fn run_crossterm(
    mut terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    options: CLIOptions,
) -> Result<(), Box<dyn std::error::Error>> {
    let geojson_file: String;
    let lat: f64;
    let lon: f64;
    if options.geojson_file.is_some() {
        lat = options.latitude.unwrap();
        lon = options.longitude.unwrap();
        geojson_file = format!("{:?}", options.geojson_file.unwrap());
    } else if options.latitude.is_some() && options.longitude.is_some() {
        lat = options.latitude.unwrap();
        lon = options.longitude.unwrap();
        geojson_file = get_geojson_file_by_lat_lon(lat, lon, options.size).await?;
    } else if options.address.is_some() {
        let (latitude, longitude) =
            nominatim::get_address_lat_lon(options.address.unwrap()).await?;
        lat = latitude;
        lon = longitude;
        geojson_file = get_geojson_file_by_lat_lon(lat, lon, Some(0.002)).await?;
    } else {
        panic!("Need to provide one of geojson_file, latitude/longitude, or address")
    };
    let geojson = operations::parse_geojson_file(&geojson_file);
    let data_structure = operations::process_geojson(&geojson);
    let coordinates = gt::Coordinate {
        x: (lon * TILE_SCALE) as i32,
        y: (lat * TILE_SCALE) as i32,
    };
    let zoom = 1;
    let mut viewport = viewport::Viewport {
        data_structure: &data_structure,
        coordinates,
        zoom,
    };
    terminal.clear()?;
    // Display loop.
    loop {
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
                // Valid user input received and processed.
                if input_action == actions::PlayerAction::Quit {
                    // User asking to quit.
                    println!("Exiting");
                    return Ok(());
                }
                // Back to draw loop.
                break;
            }
            // Invalid/unrecognized user input, continue user input loop.
            continue;
        }
    }
}
