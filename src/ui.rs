use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Stylize},
    text::Line,
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::app::App;

impl Widget for &App<'_> {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("crateful")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);
        let vertical = Layout::vertical([
            Constraint::Length(4),
            Constraint::Length(33),
            Constraint::Min(1),
        ]);
        let [playing, list, controls] = vertical.areas(area);

        let text = format!(
            "This is a tui template.\n\
                Counter: {}",
            self.counter
        );
        let listformat = format!("{:#?}", self.track_list);
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
        paragraph.render(playing, buf);
        paragraph2.render(list, buf);
        Line::from(trace).bold().render(controls, buf);
    }
}
