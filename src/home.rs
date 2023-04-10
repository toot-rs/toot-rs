// #[derive(Debug)]
// pub struct HomeView {
//     pub username: String,
//     pub url: String,
//     pub mastodon_client: Mastodon,
//     pub timeline: Option<Vec<Status>>,
//     pub selected: usize,
//     pub status: String,
// }

// impl From<LoginDetails> for HomeView {
//     fn from(login_details: LoginDetails) -> Self {
//         Self {
//             username: login_details.account.username,
//             url: login_details.url,
//             mastodon_client: login_details.mastodon,
//             timeline: None,
//             selected: 0,
//             status: String::new(),
//         }
//     }
// }

// impl HomeView {
//     pub async fn run(&mut self, sender: mpsc::Sender<Event>) -> anyhow::Result<()> {
//         self.status = "Loading timeline...".to_string();
//         let page = self
//             .mastodon_client
//             .get_home_timeline()
//             .await
//             .context("Failed to load timeline")?;
//         self.status = "Timeline loaded".to_string();
//         self.timeline = Some(page.initial_items);
//         Ok(())
//     }

//     pub fn title(&self) -> String {
//         format!(
//             "{}@{}",
//             self.username,
//             self.url.trim_start_matches("https://")
//         )
//     }

//     pub fn draw(&self, frame: &mut Frame<impl Backend>, area: Rect) {
//         let mut items = vec![];
//         if let Some(timeline) = &self.timeline {
//             // debugging for width and selected item
//             // items.push(ListItem::new(
//             //     "12345678901234567890123456789012345678901234567890123456789012345678901234567890",
//             // ));
//             // items.push(ListItem::new(format!("{}", self.selected)));
//             for status in timeline {
//                 items.push(ListItem::new(format_status(status, area.width)));
//             }
//         } else {
//             items.push(ListItem::new("Loading timeline..."));
//         }
//         // this looks great on a dark theme, but not so much on a light one
//         let style = Style::default().bg(Color::Rgb(16, 32, 64));
//         let list = List::new(items).highlight_style(style);
//         let mut state = ListState::default();
//         state.select(Some(self.selected));
//         frame.render_stateful_widget(list, area, &mut state);
//     }

//     pub fn handle_event(&mut self, event: CrosstermEvent) {
//         if let CrosstermEvent::Key(key) = event {
//             match (key.modifiers, key.code) {
//                 (KeyModifiers::NONE, KeyCode::Char('j')) => {
//                     self.scroll_down();
//                 }
//                 (KeyModifiers::NONE, KeyCode::Char('k')) => {
//                     self.scroll_up();
//                 }
//                 _ => {}
//             }
//         }
//     }

//     fn scroll_down(&mut self) {
//         self.selected += 1;
//         self.update_status()
//     }

//     fn scroll_up(&mut self) {
//         self.selected = self.selected.saturating_sub(1);
//         self.update_status()
//     }

//     pub fn status(&self) -> String {
//         self.status.clone()
//     }

//     fn update_status(&mut self) {
//         if let Some(timeline) = &self.timeline {
//             if let Some(status) = timeline.get(self.selected) {
//                 let date_format =
//                     format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]")
//                         .unwrap_or_default();
//                 let date = status.created_at.format(&date_format).unwrap_or_default();
//                 let url = status
//                     .reblog
//                     .as_ref()
//                     .map_or(status.url.clone(), |reblog| reblog.url.clone())
//                     .unwrap_or_default();
//                 self.status = format!("({}) {}", date, url);
//             }
//         }
//     }
// }

// fn format_status(status: &Status, width: u16) -> Text {
//     let account = &status.account;
//     let reblog = status.reblog.as_ref();
//     let acct = reblog.map_or(account.acct.clone(), |reblog| reblog.account.acct.clone());
//     let display_name = reblog.map_or(account.display_name.clone(), |reblog| {
//         reblog.account.display_name.clone()
//     });
//     let mut text = Text::from(Spans::from(vec![
//         Span::styled(format!("{} ", acct), Style::default().fg(Color::Yellow)),
//         Span::styled(
//             format!("({})", display_name),
//             Style::default()
//                 .fg(Color::Green)
//                 .add_modifier(Modifier::ITALIC),
//         ),
//     ]));
//     let html = reblog.map_or(status.content.clone(), |reblog| reblog.content.clone());
//     let content = html2text::from_read(html.as_bytes(), width as usize);
//     text.extend(Text::from(content));
//     text.extend(Text::raw(""));
//     text
// }