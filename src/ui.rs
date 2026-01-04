use color_eyre::owo_colors::OwoColorize;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Offset, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, List, Padding, Paragraph, Widget, Wrap},
};
use roundable::{Roundable, SECOND, Tie};

use crate::app::App;
use crate::app::PauseMode;
use crate::instructs;
use crate::pause;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([
            Constraint::Percentage(20),
            Constraint::Percentage(50),
            Constraint::Percentage(30),
        ]);
        let [playing, list, controls] = vertical.areas(area);
        let block = Block::bordered()
            .title_alignment(Alignment::Center)
            .title(format!(
                "Sorting Tracks in folder \r\n{}",
                self.incoming.to_str().unwrap()
            ))
            .title_style(Style::new().dark_gray().bold())
            .border_type(BorderType::Rounded);
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

        let playblock = Block::new().padding(Padding::vertical(playing.height / 4));

        let pause_instruct = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([Constraint::Percentage(50)])
            .margin(1)
            .split(inner_menu[1]);

        let text = format!(
            "Now Playing:\n\
                 {:?}... it's {:?} long",
            self.playing.file_name().unwrap(),
            self.length.round_to(SECOND, Tie::Up)
        );

        let now_playing = Paragraph::new(text)
            .fg(Color::White)
            .block(playblock)
            .bg(Color::DarkGray)
            .centered();
        List::new(self.display_list.clone())
            .fg(Color::Blue)
            .bg(Color::Gray)
            .block(block)
            .render(list, buf);

        now_playing.render(playing, buf);

        let bottom_section = instructs::Instructs::new(controls, self, buf);
        instructs::Instructs::display(bottom_section, controls, buf);

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
            PauseMode::SaveSelect(save_path) => {
                self.explorer
                    .widget()
                    .render(inner_menu[2].offset(Offset { x: 0, y: 0 }), buf);
                Paragraph::new(
                    "Pick a Folder to store saved tracks. \r\n Use arrow keys (or hjkl) to navigate the explorer. 
                        \r\nLeft (or h) goes to the parent directory.
                        \r\nRight (or l) goes into the selected child directory.
                        \r\n\r\n Select a foler with Enter.",
                )
                .wrap(Wrap { trim: true })
                .render(pause_instruct[0], buf);
            }
            PauseMode::IncomingSelect => {
                self.explorer
                    .widget()
                    .render(inner_menu[2].offset(Offset { x: 0, y: 0 }), buf);
                Paragraph::new(
                    "Select a folder to sort! \r\n\r\nUse arrow keys (or hjkl) \r\n to navigate the explorer. 
                         \r\nLeft (or h) goes to the parent directory.
                        \r\nRight (or l) goes into the selected child directory.
                        \r\n\r\n Select a foler with Enter \r\n\r\n Select one with ONLY wav, flac, & mp3 files... the program crashes otherwise!",
                )
                .wrap(Wrap { trim: true })
                .render(pause_instruct[0], buf);
            }
            _ => {}
        }
    }
}
