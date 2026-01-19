use crate::App;
use crate::app::PauseMode;
use directories::ProjectDirs;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Offset, Rect},
    style::{Style, Stylize},
    text::{Line, Text},
    widgets::{
        Bar, BarChart, BarGroup, Block, Borders, Clear, List, ListState, Paragraph,
        StatefulWidgetRef, Widget,
    },
};
use std::path::PathBuf;

use derive_setters::Setters;
#[derive(Debug, Default, Setters)]
pub struct Popup {
    vol: f32,
}

impl Popup {
    pub fn show(mut self, area: Rect, app_state: &App, buf: &mut Buffer) {
        self.vol = app_state.music_player.lock().unwrap().volume();
        self.render(area, buf);
    }
}
impl Widget for Popup {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let pop_per = Layout::horizontal([Constraint::Percentage(8)]).margin(5);
        let new_pop: [Rect; 1] = pop_per.areas(area);

        let bar_area = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([Constraint::Percentage(90)])
            .margin(2)
            .split(new_pop[0]);
        Clear.render(new_pop[0], buf);

        let vol_bar = Bar::default().value(20 as u64).label(Line::from("Volume"));

        BarChart::default()
            .direction(Direction::Vertical)
            .data(&[("", 40)])
            .max(100)
            .bar_width(5)
            .block(Block::new().title("volume"))
            .render(bar_area[0], buf);

        let select_error_area_big = Layout::vertical([Constraint::Percentage(55)]).margin(9);
        let select_error_rect: [Rect; 1] = select_error_area_big.areas(area);
        let select_error_area = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([Constraint::Percentage(63)])
            .split(select_error_rect[0]);
        let error_para = Paragraph::new(
            "You must select a FOLDER with enter, \r\nDon't select a file!\r\n\r\n\r\nPress esc or space to try again",
        );
        let error_block = Block::new()
            .title("Warning: Over-Driving track, may distort")
            .borders(Borders::ALL);

        if self.vol > 1.0 {
            Clear.render(select_error_area[0], buf);
            error_block.render(select_error_area[0], buf);
            error_para.render(select_error_area[0].offset(Offset { x: 1, y: 1 }), buf);
        }
    }
}
