use crate::utility::Uvec2;

use ratatui::{
    buffer::{Buffer}, layout::{Rect}, widgets::Widget
};

pub struct World {
    size: Uvec2,
    frame: Vec<char>,
}

impl Widget for &World {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let width: usize  = self.size.x + 2;
        let height: usize = self.size.y + 2;

        for y in 0..height {
            for x in 0..width {
                let idx: usize = y * width + x;
                let ch: char = self.frame[idx];

                if let Some(cell) = buf.cell_mut((area.x + x as u16, area.y + y as u16)) {
                    cell.set_char(ch);
                }
            }
        }
    }
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        Self { size: Uvec2 { x: width - 2, y: height - 2 }, frame: Self::build_frame(width, height) }
    }

    pub fn write_frame(&mut self, pos: Uvec2, c: char) {
        self.frame[pos.y * self.size.x + pos.x] = c;
    } 

    pub fn get_size(&self) -> Uvec2 {
        self.size
    }

    pub fn print_frame(&self) {
        let s: String = self.frame.iter().collect();
        println!("{}", s);
    }

    fn build_frame(width: usize, height: usize) -> Vec<char> {
        let mut frame : Vec<char> = vec![' '; width * height];
        // putting in corners
        frame[0]                    = '\u{2554}'; // ╔
        frame[width - 1]            = '\u{2557}'; // ╗
        frame[(height - 1) * width] = '\u{255A}'; // ╚
        frame[height * width - 1]   = '\u{255D}'; // ╝
        
        // iterate column wise
        for i in 1..height - 1 {
            // left edge
            frame[i * width]           = '\u{2551}'; // ║
            // right edge
            frame[(i + 1) * width - 1] = '\u{2551}'; // ║
        }
        // iterate row wise
        let offset = (height - 1) * width;
        for i in 1..width - 1 {
            // top edge
            frame[i]          = '\u{2550}'; // ═
            // bottom edge
            frame[offset + i] = '\u{2550}'; // ═
        }
        frame
    }
}
