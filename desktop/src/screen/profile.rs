mod game;
mod ranked_overview;
mod summoner;
mod timeline;

use futures::TryFutureExt;
use iced::border;
use iced::widget::button;
use iced::widget::horizontal_space;
use iced::widget::pick_list;
use iced::widget::text;
use iced::widget::vertical_space;
use iced::Alignment;
use iced::Theme;
use ranked_overview::RankedOverview;
use summoner::Summoner;
use timeline::Timeline;

use crate::core;
use crate::screen::search_bar::{self, SearchBar};
use crate::theme;

use core::game::Queue;
use std::sync::Arc;

use iced::widget::{column, container, row};
use iced::{Element, Length, Task};

pub use core::summoner::Data;

pub type Error = String;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueueFilter {
    #[default]
    All,
    Specific(Queue),
}

impl QueueFilter {
    pub const ALTERNATIVE: [QueueFilter; 7] = [
        QueueFilter::Specific(Queue::Custom),
        QueueFilter::Specific(Queue::Blind),
        QueueFilter::Specific(Queue::Draft),
        QueueFilter::Specific(Queue::Clash),
        QueueFilter::Specific(Queue::BotIntro),
        QueueFilter::Specific(Queue::BotBeginner),
        QueueFilter::Specific(Queue::BotIntermediate),
    ];
}

impl PartialEq<Queue> for QueueFilter {
    fn eq(&self, other: &Queue) -> bool {
        match self {
            Self::All => true,
            Self::Specific(queue) => queue == other,
        }
    }
}

impl std::fmt::Display for QueueFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueueFilter::All => f.write_str("All"),
            QueueFilter::Specific(queue) => write!(f, "{queue}"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    FetchedData(Result<Data, String>),

    Timeline(timeline::Message),
    Summoner(summoner::Message),
    SearchBar(search_bar::Message),
    RankedOverview(ranked_overview::Message),

    QueueFilterChanged(QueueFilter),
    ThemeChanged(Theme),
}

#[derive(Debug, Clone)]
pub struct Profile {
    region: core::Region,
    queue_filter: QueueFilter,

    timeline: Timeline,
    summoner: Summoner,
    search_bar: SearchBar,
    ranked_overview: RankedOverview,
    data: Arc<Data>,
    theme: Theme,
}

impl Profile {
    pub fn from_profile(assets: &mut crate::Assets, profile: Data) -> Self {
        let queue_filter = QueueFilter::default();
        let timeline = Timeline::from_profile(assets, &profile, &queue_filter);
        let theme = Theme::Moonfly;

        Self {
            region: core::Region::default(),
            queue_filter,

            search_bar: SearchBar::new(),
            summoner: Summoner::from_profile(&profile),
            timeline,
            ranked_overview: RankedOverview::from_profile(assets, &profile),
            data: Arc::new(profile),
            theme,
        }
    }

    pub fn update(&mut self, message: Message, assets: &mut crate::Assets) -> Task<Message> {
        match message {
            Message::ThemeChanged(theme) => {
                self.theme = theme;
            }
            Message::QueueFilterChanged(new_filter) => {
                self.timeline = Timeline::from_profile(assets, &self.data, &new_filter);
                self.queue_filter = new_filter;
            }
            Message::FetchedData(Ok(data)) => {
                self.summoner = Summoner::from_profile(&data);
                self.timeline = Timeline::from_profile(assets, &data, &self.queue_filter);
                self.ranked_overview = RankedOverview::from_profile(assets, &data);
                self.data = Arc::new(data);
            }
            Message::FetchedData(Err(error)) => panic!("failed: {error}"),
            Message::Timeline(message) => {
                if let Some(game::Event::NamePressed(riot_id)) = self.timeline.update(message) {
                    return riot_id
                        .name
                        .as_ref()
                        .zip(riot_id.tagline.as_ref())
                        .map(|(name, tag)| {
                            Task::perform(
                                fetch_data(format!("{name}#{tag}"), self.region),
                                Message::FetchedData,
                            )
                        })
                        .unwrap_or(Task::none());
                }
            }
            Message::Summoner(message) => {
                if let Some(event) = self.summoner.update(message) {
                    match event {
                        summoner::Event::UpdateProfile(name) => {
                            return Task::perform(
                                fetch_data(name, self.region),
                                Message::FetchedData,
                            );
                        }
                    }
                }
            }
            Message::SearchBar(message) => {
                if let Some(event) = self.search_bar.update(message) {
                    match event {
                        search_bar::Event::SearchRequested { riot_id, region } => {
                            self.region = region;

                            return Task::perform(
                                fetch_data(riot_id, region),
                                Message::FetchedData,
                            );
                        }
                    }
                }
            }
            Message::RankedOverview(message) => self.ranked_overview.update(message),
        }

        Task::none()
    }

    pub fn view(&self) -> Element<Message> {
        let top_bar = container(
            row![
                theme::logo(),
                horizontal_space().width(Length::FillPortion(2)),
                self.search_bar.view().map(Message::SearchBar),
                horizontal_space().width(Length::FillPortion(2)),
                pick_list(Theme::ALL, Some(self.theme.clone()), Message::ThemeChanged)
                    .style(|theme, status| theme::queue_picklist(false, theme, status))
                    .menu_style(theme::region_menu)
                    .text_size(12),
            ]
            .align_y(Alignment::Center),
        )
        .padding(8)
        .style(|theme| container::Style {
            border: border::rounded(0),
            ..theme::dark(theme)
        });

        let timeline = row![
            self.ranked_overview.view().map(Message::RankedOverview),
            self.timeline.view().map(Message::Timeline),
        ]
        .spacing(8);

        let content = column![
            self.summoner.view().map(Message::Summoner),
            filter_bar(self.queue_filter),
            timeline,
        ]
        .width(968)
        .spacing(8)
        .padding(8);

        container(column![
            top_bar,
            vertical_space().height(16),
            container(content).center_x(Length::Fill),
        ])
        .style(theme::timeline)
        .into()
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

fn filter_bar<'a>(selected: QueueFilter) -> Element<'a, Message> {
    let queue_button = |queue: QueueFilter| -> Element<Message> {
        button(text!("{queue}").size(12))
            .style(move |theme, status| theme::queue_filter(theme, status, selected == queue))
            .on_press(Message::QueueFilterChanged(queue))
            .into()
    };

    let picked = Some(selected).filter(|queue| QueueFilter::ALTERNATIVE.contains(queue));

    container(
        row![
            queue_button(QueueFilter::All),
            queue_button(QueueFilter::Specific(Queue::RankedSolo)),
            queue_button(QueueFilter::Specific(Queue::RankedFlex)),
            queue_button(QueueFilter::Specific(Queue::ARAM)),
            pick_list(
                QueueFilter::ALTERNATIVE,
                picked,
                Message::QueueFilterChanged
            )
            .text_size(12)
            .placeholder("Queue type")
            .style(move |theme, status| theme::queue_picklist(picked.is_some(), theme, status))
            .menu_style(theme::region_menu),
        ]
        .spacing(4),
    )
    .width(Length::Fill)
    .padding(8)
    .style(theme::dark)
    .into()
}

pub async fn fetch_data(
    name: String,
    region: core::Region,
) -> Result<core::summoner::Data, String> {
    let worker_url = dotenv::var("WORKER_URL").map_err(|e| e.to_string())?;
    let path = format!("{worker_url}/summoner/{region}/{}", name.replace("#", "-"));
    tracing::info!("Requesting `{name}` ({region}) to {path}");

    reqwest::get(path)
        .map_err(|e| e.to_string())
        .await?
        .json::<Data>()
        .map_err(|e| e.to_string())
        .await
}
