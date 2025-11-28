use crate::app::App;
use color_eyre::config::Frame;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Offset, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{
        Block, BorderType, Borders, Clear, List, ListState, Paragraph, StatefulWidgetRef, Tabs,
        Widget, Wrap,
    },
};

use derive_setters::Setters;
use ratatui_explorer::{FileExplorer, Theme};
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
}

impl Popup<'_> {
    pub fn show(mut self, area: Rect, app_state: &App, buf: &mut Buffer) {
        self.pause_menu = app_state.pause_menu.clone();
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
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
            .margin(1)
            .split(new_pop[0]);
        // ensure that all cells under the popup are cleared to avoid leaking content
        Clear.render(new_pop[0], buf);

        let block = Block::new()
            .title(self.title)
            .title_style(self.title_style)
            .borders(Borders::ALL)
            .border_style(self.border_style);
        Paragraph::new("asdfjdklsa;")
            .wrap(Wrap { trim: true })
            .style(self.style)
            .block(block)
            .render(new_pop[0], buf);
        let theme = Theme::default().add_default_title();
        let mut file_explore = FileExplorer::new().unwrap();
        file_explore
            .widget()
            .render(inner_menu[2].offset(Offset { x: 0, y: 5 }), buf);

        let selects = [
            "Select folder to sort",
            "Set save folders",
            "Resume sorting (press Space)",
        ];
        List::new(selects)
            .block(Block::bordered().title("options"))
            .highlight_style(Style::new().white())
            .highlight_symbol(">>")
            .render_ref(
                inner_menu[0].offset(Offset { x: 0, y: 0 }),
                buf,
                &mut self.pause_menu,
            );
        Block::new()
            .title("another")
            .borders(Borders::ALL)
            .render(inner_menu[1].offset(Offset { x: 0, y: 0 }), buf);
        Block::new()
            .title("this ")
            .borders(Borders::ALL)
            .render(inner_menu[2].offset(Offset { x: 0, y: 5 }), buf);
        Block::new()
            .title("final")
            .borders(Borders::ALL)
            .render(inner_menu[3].offset(Offset { x: 0, y: 0 }), buf);
    }
}
