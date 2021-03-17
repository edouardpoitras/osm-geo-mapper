use geo_types as gt;
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
};

use crate::{features::GeoTilesDataStructure, viewer::theme::get_geo_tile_theme};

pub struct Viewport {
    pub data_structure: GeoTilesDataStructure,
    pub coordinates: gt::Coordinate<i32>,
    pub zoom: u8,
    pub loading: bool,
}

impl Viewport {
    pub fn move_up(&mut self, amount: i32) {
        self.coordinates.y += amount;
    }
    pub fn move_down(&mut self, amount: i32) {
        self.coordinates.y -= amount;
    }
    pub fn move_left(&mut self, amount: i32) {
        self.coordinates.x -= amount;
    }
    pub fn move_right(&mut self, amount: i32) {
        self.coordinates.x += amount;
    }
}

impl Widget for &mut Viewport {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let left = area.left();
        let top = area.top();
        let width = area.right() - left;
        let height = area.bottom() - top;
        let x_offset = self.coordinates.x - (self.zoom as i32 * (width as i32 / 2));
        let y_offset = self.coordinates.y - (self.zoom as i32 * (height as i32 / 2));
        let locked_data_structure = self.data_structure.read().unwrap();
        for x in 0..width {
            let coord_x = ((x * self.zoom as u16) as i32) + x_offset;
            for y in 0..height {
                let coord_y = (((height - 1 - y) * self.zoom as u16) as i32) + y_offset;
                let current_coordinates = gt::Coordinate {
                    x: coord_x,
                    y: coord_y,
                };
                let mut character = ' ';
                let mut style = Style::default();
                if let Some(geo_tiles) = locked_data_structure.get(&current_coordinates) {
                    if geo_tiles.len() > 0 {
                        let geo_tile = geo_tiles[0].clone();
                        let theme = get_geo_tile_theme(&geo_tile);
                        character = theme.character;
                        style = theme.style;
                    }
                }
                buf.get_mut(x + left, y + top)
                   .set_char(character)
                   .set_style(style);
            }
        }
        // Redraw the center cursor (necessary when zoomed).
        // TODO/BUG: Sometimes the center cursor appears one unit below where it should be.
        //           Previously I had fixed it by doing `top - 1 - (height / 2) as u16` below.
        //           However, it would eventually start working normally again and offset the
        //           cursor by one above. Need to figure out why this is happening.
        //           May be related to the data received by OpenStreetMap.
        buf.get_mut(left + (width / 2) as u16, top + (height / 2) as u16)
           .set_char('+')
           .set_style(Style::default().fg(Color::Red));
    }
}
