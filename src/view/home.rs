use std::fmt::Display;
use time::format_description;

use mastodon_async::{mastodon::Mastodon, prelude::Status};
use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{List, ListItem},
    Frame,
};
use tokio::sync::mpsc;

use crate::{Event, LoginDetails};

#[derive(Debug, Clone)]
pub struct HomeView {
    username: String,
    url: String,
    mastodon_client: Mastodon,
    timeline: Option<Vec<Status>>,
}

impl Display for HomeView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}@{}",
            self.username,
            self.url.trim_start_matches("https://")
        )
    }
}

impl From<LoginDetails> for HomeView {
    fn from(login_details: LoginDetails) -> Self {
        Self {
            username: login_details.account.username,
            url: login_details.url,
            mastodon_client: login_details.mastodon_client,
            timeline: None,
        }
    }
}

impl HomeView {
    pub async fn run(&mut self, tx: mpsc::Sender<Event>) {
        match self.mastodon_client.get_home_timeline().await {
            Ok(timeline) => self.timeline = Some(timeline.initial_items),
            Err(e) => {
                if let Err(send_err) = tx.send(Event::MastodonError(e)).await {
                    eprintln!("Error sending MastodonError event: {}", send_err);
                }
            }
        }
    }

    pub fn draw(&self, frame: &mut Frame<impl Backend>, area: Rect) {
        let mut items = vec![];
        if let Some(timeline) = &self.timeline {
            // items.push(ListItem::new("12345678901234567890123456789012345678901234567890123456789012345678901234567890"));
            for status in timeline {
                items.push(ListItem::new(format_status(status, area.width)));
            }
        } else {
            items.push(ListItem::new("Loading timeline..."));
        }
        let list = List::new(items);
        frame.render_widget(list, area);
    }
}

fn format_status(status: &Status, width: u16) -> Text {
    let account = &status.account;
    let acct = status
        .reblog
        .as_ref()
        .map_or(account.acct.clone(), |reblog| reblog.account.acct.clone());
    let display_name = status
        .reblog
        .as_ref()
        .map_or(account.display_name.clone(), |reblog| {
            reblog.account.display_name.clone()
        });
    let date_format =
        format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
    let date = status.created_at.format(&date_format).unwrap_or_default();
    let url = status
        .reblog
        .as_ref()
        .map_or(status.url.clone(), |reblog| reblog.url.clone())
        .unwrap_or_default();
    let mut text = Text::from(Spans::from(vec![
        Span::styled(format!("{} ", date), Style::default().fg(Color::DarkGray)),
        Span::styled(format!("{} ", acct), Style::default().fg(Color::Yellow)),
        Span::styled(
            format!("({})", display_name),
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::ITALIC),
        ),
        Span::styled(format!(" {}", url), Style::default().fg(Color::DarkGray)),
    ]));
    let html = status
        .reblog
        .as_ref()
        .map_or(status.content.clone(), |reblog| reblog.content.clone());
    let content = html2text::from_read(html.as_bytes(), width as usize);
    text.extend(Text::from(content));
    text.extend(Text::raw(""));
    text
}
