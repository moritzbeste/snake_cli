use crate::utility::Uvec2;

use ratatui::{
    buffer::{Buffer, Cell}, layout::{Position, Rect}, widgets::Widget
};

pub struct World {
    size: Uvec2,
    frame: Buffer,
}

impl Widget for World {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized {
        todo!()
    }
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        let buf = Buffer::filled(
            Rect {
                x: 0,
                y: 0,
                width: width as u16,
                height: height as u16,
            }, 
            Cell::new(" "));
        Self { size: Uvec2 { x: width - 2, y: height - 2 }, frame: buf }
    }

    pub fn write_frame(&mut self, pos: Uvec2, c: char) {
        self.frame[Position { 
            x: (pos.x + 1) as u16, 
            y: (pos.y + 1) as u16 
        }].set_symbol(&c.to_string());
    } 

    pub fn get_size(&self) -> Uvec2 {
        self.size
    }

    fn build_frame(width: usize, height: usize) -> String {




        let mut frame = String::from("\u{2554}"); // ╔
        let mut row = String::from("\u{2551}"); // ║
        let mut bottom = String::from("\u{255A}"); // ╚
        for _i in 0..(width - 2) {
            frame += "\u{2550}"; // ═
            row += " ";
            bottom += "\u{2550}"; // ═
        }
        frame += "\u{2557}\n"; // ╗
        row += "\u{2551}\n"; // ║
        bottom += "\u{255D}"; // ╝
        for _i in 0..(height - 2) {
            frame += &row;
        }
        frame += &bottom;

        frame
    }
}