use color_eyre::config::Frame;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Margin, Offset, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget, Wrap},
};

use crate::app::App;
use crate::app::PauseMode;
use crate::pause;
use crate::pause::Popup;
impl Widget for &App<'_> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title_alignment(Alignment::Center)
            .title("Track to Sort")
            .border_type(BorderType::Rounded);
        let vertical = Layout::vertical([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ]);
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
            .margin(3)
            .split(new_pop[0]);
        let [playing, list, controls] = vertical.areas(area);

        let instruct = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([Constraint::Percentage(50)])
            .margin(1)
            .split(inner_menu[1]);

        let text = format!(
            "Now Playing\n\
                : {:?}... it's this long: {:?}",
            self.playing, self.length
        );
        let listformat = format!("{:#?}", self.display_list);
        let trace = format!("{:#?}", self.playing);

        let paragraph = Paragraph::new(text)
            .fg(Color::White)
            .bg(Color::DarkGray)
            .centered();
        let paragraph2 = Paragraph::new(listformat)
            .fg(Color::Blue)
            .bg(Color::White)
            .centered()
            .block(block);
        let para3 = Paragraph::new(trace)
            .fg(Color::White)
            .bg(Color::LightBlue)
            .centered();
        paragraph.render(playing, buf);
        paragraph2.render(list, buf);
        //        Line::from(trace).bold().render(controls, buf);

        para3.render(controls, buf);
        let popup = pause::Popup::default()
            .content("Hello world!")
            .style(Style::new().yellow())
            .title("Pause Menu... Press Space to Resume Sorting")
            .title_style(Style::new().white().bold())
            .border_style(Style::new().red());

        if self.paused {
            popup.show(area, self, buf)
        };

        match self.pause_mode {
            PauseMode::SaveSelect => {
                self.explorer
                    .widget()
                    .render(inner_menu[2].offset(Offset { x: 0, y: 0 }), buf);
                Paragraph::new(
                    "Pick a Folder to store saved tracks. \r\n\r\n Use arrow keys (or hjkl) to navigate the explorer. \r\n\r\n Select a foler with Enter",
                )
                .wrap(Wrap { trim: true })
                .render(instruct[0], buf);
            }
            PauseMode::IncomingSelect => {
                //                self.explorer.set_cwd(self.explorer_path);
                //                self.explorer.set_selected_idx(self.explorer_index);
                self.explorer
                    .widget()
                    .render(inner_menu[2].offset(Offset { x: 0, y: 0 }), buf);
                Paragraph::new(
                    "Select a folder to sort \r\nUse arrow keys (or hjkl) \r\n\r\n to navigate the explorer. \r\n\r\n Select a foler with Enter",
                )
                .wrap(Wrap { trim: true })
                .render(instruct[0], buf);
            }
            _ => {}
        }
    }
}
