mod game;
mod ranked_overview;
mod summoner;
mod timeline;

use futures::TryFutureExt;
use iced::widget::button;
use iced::widget::horizontal_space;
use iced::widget::pick_list;
use iced::widget::text;
use iced::widget::vertical_space;
use iced::Alignment;
use ranked_overview::RankedOverview;
use summoner::Summoner;
use timeline::Timeline;

use crate::core;
use crate::screen::search_bar::{self, SearchBar};
use crate::theme;

use core::game::Queue;

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
}

#[derive(Debug, Clone)]
pub struct Profile {
    region: core::Region,
    queue_filter: QueueFilter,

    timeline: Timeline,
    summoner: Summoner,
    search_bar: SearchBar,
    ranked_overview: RankedOverview,
}

impl Profile {
    pub fn from_profile(assets: &mut crate::Assets, profile: Data) -> Self {
        Self {
            region: core::Region::default(),
            queue_filter: QueueFilter::default(),

            search_bar: SearchBar::new(),
            summoner: Summoner::from_profile(assets, &profile),
            timeline: Timeline::from_profile(assets, &profile),
            ranked_overview: RankedOverview::from_profile(assets, &profile),
        }
    }

    pub fn update(&mut self, message: Message, assets: &mut crate::Assets) -> Task<Message> {
        match message {
            Message::QueueFilterChanged(new_filter) => {
                self.queue_filter = new_filter;
            }
            Message::FetchedData(Ok(data)) => {
                self.summoner = Summoner::from_profile(assets, &data);
                self.timeline = Timeline::from_profile(assets, &data);
                self.ranked_overview = RankedOverview::from_profile(assets, &data);
            }
            Message::FetchedData(Err(error)) => panic!("failed: {error}"),
            Message::Timeline(message) => self.timeline.update(message),
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
        let timeline = container(
            row![
                self.ranked_overview.view().map(Message::RankedOverview),
                self.timeline.view().map(Message::Timeline),
            ]
            .padding(8)
            .spacing(8),
        )
        .center_x(Length::Fill);

        let top_bar = container(
            row![
                theme::logo(),
                horizontal_space().width(Length::FillPortion(2)),
                self.search_bar.view().map(Message::SearchBar),
                horizontal_space().width(Length::FillPortion(2)),
            ]
            .align_y(Alignment::Center),
        )
        .padding(8)
        .style(theme::dark);

        container(column![
            top_bar,
            vertical_space().height(16),
            self.summoner.view().map(Message::Summoner),
            vertical_space().height(16),
            filter_bar(self.queue_filter),
            timeline,
        ])
        .style(theme::timeline)
        .into()
    }
}

fn filter_bar<'a>(selected: QueueFilter) -> Element<'a, Message> {
    let queue_button = |queue: QueueFilter| -> Element<Message> {
        button(text!("{queue}").size(12))
            .style(move |_, status| theme::queue_filter(selected == queue, status))
            .on_press(Message::QueueFilterChanged(queue))
            .into()
    };

    let picked = Some(selected).filter(|queue| QueueFilter::ALTERNATIVE.contains(queue));

    container(
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
        .padding(8)
        .style(theme::dark)
        .max_width(970)
        .width(Length::Fill),
    )
    .center_x(Length::Fill)
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
