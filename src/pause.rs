use crate::App;
use crate::app::PauseMode;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Offset, Rect},
    style::{Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Borders, Clear, List, ListState, Paragraph, StatefulWidgetRef, Widget},
};
use std::path::PathBuf;

use derive_setters::Setters;
#[derive(Debug, Default, Setters)]
pub struct Popup<'a> {
    #[setters(into)]
    title: Line<'a>,
    #[setters(into)]
    content: Text<'a>,
    border_style: Style,
    title_style: Style,
    style: Style,
    pause_menu: ListState,
    pause_mode: PauseMode,
    explorer_path: PathBuf,
    explorer_index: usize,
}

impl Popup<'_> {
    pub fn show(mut self, area: Rect, app_state: &App, buf: &mut Buffer) {
        self.pause_menu = app_state.pause_menu.clone();
        self.pause_mode = app_state.pause_mode.clone();
        self.explorer_index = app_state.explorer_index.clone();
        self.explorer_path = app_state.explorer_path.clone();
        self.render(area, buf);
    }
}
impl Widget for Popup<'_> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let pop_per = Layout::vertical([Constraint::Percentage(80)]).margin(5);
        let new_pop: [Rect; 1] = pop_per.areas(area);

        let inner_menu = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ])
            .margin(2)
            .split(new_pop[0]);
        Clear.render(new_pop[0], buf);
        let selects = [
            "Select folder to sort",
            "Set save folders",
            "Resume sorting (press Space)",
        ];
        List::new(selects)
            .block(Block::bordered().title("options (arrows/jk, Enter)"))
            .highlight_style(Style::new().white())
            .highlight_symbol(">>")
            .render_ref(
                inner_menu[0].offset(Offset { x: 0, y: 0 }),
                buf,
                &mut self.pause_menu,
            );

        let select_error_area_big = Layout::vertical([Constraint::Percentage(55)]).margin(9);
        let select_error_rect: [Rect; 1] = select_error_area_big.areas(area);
        let select_error_area = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([Constraint::Percentage(33)])
            .split(select_error_rect[0]);
        let error_para = Paragraph::new(
            "You must select a FOLDER with enter, \r\nDon't select a file!\r\n\r\n\r\nPress esc, space, or exit to try again",
        );
        let error_block = Block::new()
            .title("You Fucked up.")
            .title_style(self.title_style)
            .borders(Borders::ALL)
            .border_style(self.border_style);

        if self.pause_mode == PauseMode::SelectError {
            Clear.render(select_error_area[0], buf);
            error_block.render(select_error_area[0], buf);
            error_para.render(select_error_area[0].offset(Offset { x: 1, y: 1 }), buf);
        }
    }
}
