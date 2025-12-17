use crate::app::App;
use std::path::PathBuf;

use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    prelude::Buffer,
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph, Widget, Wrap},
};

pub struct PathStates {
    save_a: PathBuf,
    save_d: Option<PathBuf>,
    save_g: Option<PathBuf>,
}
pub struct Instructs {
    state: PathStates,
}
impl PathStates {}

impl Instructs {
    pub fn display(mut self, area: Rect, buf: &mut Buffer) {
        //        self.state.save_a = app_state.save_path_a.clone();
        self.render(area, buf);
    }
    pub fn new(area: Rect, app_state: &App, buf: &mut Buffer) -> Instructs {
        Instructs {
            state: PathStates {
                save_a: app_state.save_path_a.clone(),
                save_d: app_state.save_path_d.clone(),
                save_g: app_state.save_path_g.clone(),
            },
        }
    }
}
impl Widget for Instructs {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let controls_split = Layout::horizontal([
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ]);

        let [save_a, save_d, save_g, scrub, delete] = controls_split.areas(area);

        let save_a_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title_bottom("'a' save")
            .title_alignment(Alignment::Center);
        Paragraph::new(format!(
            "Press a\r\nto save to\r\n\r\n{:?}",
            self.state.save_a
        ))
        .block(save_a_block)
        .centered()
        .fg(Color::White)
        .bg(Color::LightBlue)
        .wrap(Wrap { trim: true })
        .render(save_a, buf);
        let save_d_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title_bottom("'d' save")
            .title_alignment(Alignment::Center);
        Paragraph::new(format!(
            "Press a\r\nto save to\r\n\r\n{:?}",
            self.state.save_a
        ))
        .block(save_d_block)
        .centered()
        .fg(Color::White)
        .bg(Color::Cyan)
        .wrap(Wrap { trim: true })
        .render(save_d, buf);
        let save_g_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title_bottom("'g' save")
            .title_alignment(Alignment::Center);
        Paragraph::new(format!(
            "Press a\r\nto save to\r\n\r\n{:?}",
            self.state.save_a
        ))
        .block(save_g_block)
        .centered()
        .fg(Color::White)
        .bg(Color::LightBlue)
        .wrap(Wrap { trim: true })
        .render(save_g, buf);
        let scrub_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title_bottom("how to scrub")
            .title_alignment(Alignment::Center);
        Paragraph::new(format!("use numbers\r\n1-9 to scrub\r\nthrough the track"))
            .block(scrub_block)
            .centered()
            .fg(Color::White)
            .bg(Color::LightCyan)
            .wrap(Wrap { trim: true })
            .render(scrub, buf);
        let delete_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title_bottom("delete")
            .title_alignment(Alignment::Center);
        Paragraph::new(format!("press backspace\r\nto delete\r\nthis track"))
            .block(delete_block)
            .centered()
            .fg(Color::White)
            .bg(Color::LightRed)
            .wrap(Wrap { trim: true })
            .render(delete, buf);
    }
}
