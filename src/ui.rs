use color_eyre::config::Frame;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget, Wrap},
};

use crate::app::App;
use derive_setters::Setters;

#[derive(Debug, Default, Setters)]
struct Popup<'a> {
    #[setters(into)]
    title: Line<'a>,
    #[setters(into)]
    content: Text<'a>,
    border_style: Style,
    title_style: Style,
    style: Style,
}

impl Widget for Popup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // ensure that all cells under the popup are cleared to avoid leaking content
        Clear.render(area, buf);
        let block = Block::new()
            .title(self.title)
            .title_style(self.title_style)
            .borders(Borders::ALL)
            .border_style(self.border_style);
        let menus = Layout::horizontal([Constraint::Percentage(40)]);
        Paragraph::new(self.content)
            .wrap(Wrap { trim: true })
            .style(self.style)
            .block(block)
            .render(area, buf);
        //     Paragraph::new("asdffdsasdf")
        //         .wrap(Wrap { trim: true })
        //         .style(self.style)
        //         .block(Block::new())
        //         .render(area, buf);
    }
}

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
        let popup_area = Rect::new(4, 5, buf.area.width - 9, buf.area.height - 12);
        let pop_per = Layout::horizontal([Constraint::Percentage(20)]);
        let new_pop: [Rect; 1] = pop_per.areas(area);

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

        let popup = Popup::default()
            .content("Hello world!")
            .style(Style::new().yellow())
            .title("With Clear")
            .title_style(Style::new().white().bold())
            .border_style(Style::new().red());
        if self.paused {
            popup.render(new_pop[0], buf)
        };
    }
}
