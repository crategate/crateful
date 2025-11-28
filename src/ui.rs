use color_eyre::config::Frame;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget, Wrap},
};

use crate::app::App;
use crate::pause;
use crate::pause::Popup;
impl Widget for &App<'_> {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title_alignment(Alignment::Center)
            .title("Track to Sort")
            .border_type(BorderType::Rounded);
        let vertical = Layout::vertical([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ]);

        let [playing, list, controls] = vertical.areas(area);

        let text = format!(
            "Now Playing\n\
                : {:?}... it's this long: {:?}",
            self.playing, self.length
        );
        let listformat = format!("{:#?}", self.display_list);
        let trace = format!("{:#?}", self.playing);

        let paragraph = Paragraph::new(text)
            .fg(Color::Cyan)
            .bg(Color::Red)
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
    }
}
