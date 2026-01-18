use crate::app::{App, Indicator};
use std::{path::PathBuf, vec};
use tokio::time::{Duration, sleep};

use ratatui::{
    layout::{Alignment, Constraint, Layout, Offset, Rect},
    prelude::Buffer,
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph, Widget, Wrap},
};

pub struct PathStates {
    save_a: Option<PathBuf>,
    save_d: Option<PathBuf>,
    save_g: Option<PathBuf>,
}
pub struct Instructs {
    state: PathStates,
    last_action: Option<Indicator>,
    offset_indicator: Vec<i32>,
}
impl PathStates {}

impl Instructs {
    pub fn display(self, area: Rect, buf: &mut Buffer) {
        //        self.state.save_a = app_state.save_path_a.clone();
        self.render(area, buf);
    }
    pub fn new(_area: Rect, app_state: &App, _buf: &mut Buffer) -> Instructs {
        Instructs {
            state: PathStates {
                save_a: app_state.save_path_a.clone(),
                save_d: app_state.save_path_d.clone(),
                save_g: app_state.save_path_g.clone(),
            },
            last_action: app_state.visual_action_indicator.clone(),
            offset_indicator: vec![0, 0, 0, 0, 0],
        }
    }
}
impl Widget for Instructs {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let controls_split = Layout::horizontal([
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ]);

        match self.last_action {
            Some(indicator) => match indicator {
                Indicator::SavedA => self.offset_indicator[0] = -1,
                Indicator::SavedD => self.offset_indicator[1] = -1,
                Indicator::SavedG => self.offset_indicator[2] = -1,
                Indicator::Scrubbed => self.offset_indicator[3] = -1,
                Indicator::Deleted => self.offset_indicator[4] = -1,
            },
            None => (),
        }

        let [save_a, save_d, save_g, scrub, delete] = controls_split.areas(area);

        let save_a_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title_bottom("'a' save")
            .title_alignment(Alignment::Center);
        fn folder_select_substitute(which_save_path: String) -> String {
            return format!(
                "Press {:?}\r\nto select a\r\nfolder to save to\r\nfor this button",
                which_save_path
            );
        }
        Paragraph::new(
            if self
                .state
                .save_a
                .as_ref()
                .unwrap()
                .clone()
                .into_os_string()
                .is_empty()
            {
                folder_select_substitute(String::from("a"))
            } else {
                format!(
                    "Press a\r\nto save to\r\n\r\n{:?}",
                    self.state.save_a.unwrap()
                )
            },
        )
        .block(save_a_block)
        .centered()
        .fg(Color::White)
        .bg(Color::Reset)
        .bg(Color::Indexed(14))
        .wrap(Wrap { trim: true })
        .render(
            save_a.offset(Offset {
                x: 0,
                y: self.offset_indicator[0],
            }),
            buf,
        );
        let save_d_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title_bottom("'d' save")
            .title_alignment(Alignment::Center);
        Paragraph::new(
            if self
                .state
                .save_d
                .as_ref()
                .unwrap()
                .clone()
                .into_os_string()
                .is_empty()
            {
                folder_select_substitute(String::from("d"))
            } else {
                format!(
                    "Press d\r\nto save to\r\n\r\n{:?}",
                    self.state.save_d.unwrap()
                )
            },
        )
        .block(save_d_block)
        .centered()
        .fg(Color::White)
        .bg(Color::Cyan)
        .wrap(Wrap { trim: true })
        .render(
            save_d.offset(Offset {
                x: 0,
                y: self.offset_indicator[1],
            }),
            buf,
        );
        let save_g_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title_bottom("'g' save")
            .title_alignment(Alignment::Center);
        Paragraph::new(
            if self
                .state
                .save_g
                .as_ref()
                .unwrap()
                .clone()
                .into_os_string()
                .is_empty()
            {
                folder_select_substitute(String::from("g"))
            } else {
                format!(
                    "Press g\r\nto save to\r\n\r\n{:?}",
                    self.state.save_g.unwrap()
                )
            },
        )
        .block(save_g_block)
        .centered()
        .fg(Color::White)
        .bg(Color::LightBlue)
        .wrap(Wrap { trim: true })
        .render(
            save_g.offset(Offset {
                x: 0,
                y: self.offset_indicator[2],
            }),
            buf,
        );
        let scrub_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title_bottom("how to scrub")
            .title_alignment(Alignment::Center);
        Paragraph::new(format!(
            "use numbers\r\n1-9 to seek\r\nthrough the track\r\n \r\narrows (or j & l)\r\nfor 2sec\r\nscrubs"
        ))
        .block(scrub_block)
        .centered()
        .fg(Color::White)
        .bg(Color::Blue)
        .wrap(Wrap { trim: true })
        .render(
            scrub.offset(Offset {
                x: 0,
                y: self.offset_indicator[3],
            }),
            buf,
        );
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
            .render(
                delete.offset(Offset {
                    x: 0,
                    y: self.offset_indicator[4],
                }),
                buf,
            );
    }
}
