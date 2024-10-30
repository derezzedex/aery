mod game;
mod ranked_overview;
mod search_bar;
mod summoner;
mod timeline;

use ranked_overview::RankedOverview;
use search_bar::SearchBar;
use summoner::Summoner;
use timeline::Timeline;

use crate::core;
use crate::theme;

use iced::widget::{column, container, row};
use iced::{Element, Length, Task};

use futures::stream;
use futures::FutureExt;
use futures::StreamExt;
use std::cmp::Reverse;

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
}

pub struct Profile {
    timeline: Timeline,
    summoner: Summoner,
    search_bar: SearchBar,
    ranked_overview: RankedOverview,
}

impl Profile {
    pub fn dummy(assets: &crate::Assets) -> Self {
        Self {
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
                                core::Summoner::from_name(client, name),
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
                        search_bar::Event::SearchRequested(name) => {
                            let client = client.clone();

                            return Task::perform(fetch_data(client, name), Message::FetchedData);
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

        container(
            column![
                self.search_bar.view().map(Message::SearchBar),
                self.summoner.view().map(Message::Summoner),
                timeline,
            ]
            .spacing(16),
        )
        .style(theme::timeline)
        .into()
    }
}

async fn fetch_data(client: core::Client, name: String) -> Result<Data, String> {
    let Ok(summoner) = core::Summoner::from_name(client.clone(), name).await else {
        return Err(String::from("Summoner not found!"));
    };

    let Ok(leagues) = summoner
        .leagues(&client)
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
