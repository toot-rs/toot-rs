use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Paragraph, Widget},
};

pub struct StatusBar {
    text: String,
}

impl StatusBar {
    pub const HEIGHT: u16 = 1;
    pub const fn new(text: String) -> Self {
        Self { text }
    }
}

impl Widget for StatusBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let style = Style::default().fg(Color::White).bg(Color::Blue);
        let bold = Style::default().add_modifier(Modifier::BOLD);
        let text = Span::raw(self.text);
        let text = Spans::from(vec![
            Span::styled("Esc ", bold),
            Span::raw("quit | "),
            Span::styled("J", bold),
            Span::raw(" down | "),
            Span::styled("K", bold),
            Span::raw(" up | "),
            text,
        ]);
        Paragraph::new(text).style(style).render(area, buf);
    }
}

#[derive(Debug, Default)]
pub struct TitleBar<'a> {
    title: &'a str,
}

impl<'a> TitleBar<'a> {
    pub const HEIGHT: u16 = 1;
    pub const fn new(title: &'a str) -> Self {
        Self { title }
    }
}

impl<'a> Widget for TitleBar<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let style = Style::default().fg(Color::White).bg(Color::Blue);
        let bold = Style::default().add_modifier(Modifier::BOLD);
        let gray = Style::default().fg(Color::Gray);
        let text = Spans::from(vec![
            Span::styled("Tooters", bold),
            Span::raw(" | "),
            Span::styled(self.title, gray),
        ]);
        Paragraph::new(text).style(style).render(area, buf);
    }
}