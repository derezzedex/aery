mod game;
mod ranked_overview;
mod search_bar;
mod summoner;
mod timeline;

use iced::widget::button;
use iced::widget::pick_list;
use iced::widget::text;
use iced::widget::vertical_space;
use ranked_overview::RankedOverview;
use search_bar::SearchBar;
use summoner::Summoner;
use timeline::Timeline;

use crate::core;
use crate::theme;

use core::game::Queue;

use iced::widget::{column, container, row};
use iced::{Element, Length, Task};

use futures::stream;
use futures::FutureExt;
use futures::StreamExt;
use std::cmp::Reverse;

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
pub struct Data {
    summoner: core::Summoner,
    leagues: Vec<core::summoner::League>,
    games: Vec<core::Game>,
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

pub struct Profile {
    region: core::Region,
    queue_filter: QueueFilter,

    timeline: Timeline,
    summoner: Summoner,
    search_bar: SearchBar,
    ranked_overview: RankedOverview,
}

impl Profile {
    pub fn dummy(assets: &crate::Assets) -> Self {
        Self {
            region: core::Region::default(),
            queue_filter: QueueFilter::default(),
            timeline: Timeline::new(assets),
            summoner: Summoner::new(5843),
            search_bar: SearchBar::new(),
            ranked_overview: RankedOverview::new(assets),
        }
    }

    pub fn update(
        &mut self,
        message: Message,
        client: &core::Client,
        assets: &mut crate::Assets,
    ) -> Task<Message> {
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
                if let Some(event) = self.summoner.update(assets, message) {
                    match event {
                        summoner::Event::UpdateProfile(name) => {
                            let client = client.clone();

                            return Task::perform(
                                core::Summoner::from_name(client, name, self.region),
                                |summoner| {
                                    Message::Summoner(summoner::Message::SummonerFetched(summoner))
                                },
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

                            let client = client.clone();
                            return Task::perform(
                                fetch_data(client, riot_id, region),
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

        container(column![
            self.search_bar.view().map(Message::SearchBar),
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

    container(
        container(
            row![
                queue_button(QueueFilter::All),
                queue_button(QueueFilter::Specific(Queue::RankedSolo)),
                queue_button(QueueFilter::Specific(Queue::RankedFlex)),
                queue_button(QueueFilter::Specific(Queue::ARAM)),
                pick_list(
                    QueueFilter::ALTERNATIVE,
                    Some(selected).filter(|queue| QueueFilter::ALTERNATIVE.contains(queue)),
                    Message::QueueFilterChanged
                )
                .text_size(12)
                .placeholder("Queue type")
                .style(theme::region)
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

async fn fetch_data(
    client: core::Client,
    name: String,
    region: core::Region,
) -> Result<Data, String> {
    let Ok(summoner) = core::Summoner::from_name(client.clone(), name, region).await else {
        return Err(String::from("Summoner not found!"));
    };

    let Ok(leagues) = summoner
        .leagues(&client, region)
        .await
        .map(|leagues| leagues.collect::<Vec<_>>())
    else {
        return Err(String::from("Failed to fetch summoner leagues."));
    };

    let mut games: Vec<core::Game> = stream::iter(summoner.matches(&client, 0..10, None).await)
        .flat_map(|game_ids| {
            stream::iter(game_ids).filter_map(|id| core::Game::from_id(&client, id).map(Result::ok))
        })
        .collect()
        .await;

    games.sort_unstable_by_key(|game| Reverse(game.created_at()));

    Ok(Data {
        summoner,
        leagues,
        games,
    })
}
