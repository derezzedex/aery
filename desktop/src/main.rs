mod assets;
mod component;
mod theme;
mod widget;

use std::cmp::Reverse;

use futures::stream;
use futures::FutureExt;
use futures::StreamExt;

use iced::{
    widget::{column, container, row},
    Application, Command, Element, Length, Settings,
};

use assets::Assets;
use component::ranked_overview::{self, RankedOverview};
use component::search_bar::{self, SearchBar};
use component::summoner::{self, Summoner};
use component::timeline::{self, Timeline};

use aery_core as core;

pub fn main() -> iced::Result {
    Aery::run(Settings {
        antialiasing: true,
        window: iced::window::Settings {
            min_size: Some([1024, 768].into()),
            ..Default::default()
        },
        ..Default::default()
    })
}

struct Aery {
    client: core::Client,
    assets: Assets,
    profile: Option<Profile>,

    timeline: Timeline,
    summoner: Summoner,
    search_bar: SearchBar,
    ranked_overview: RankedOverview,
}

#[derive(Debug, Clone)]
struct Profile {
    summoner: core::Summoner,
    leagues: Vec<core::summoner::League>,
    games: Vec<core::GameMatch>,
}

#[derive(Debug, Clone)]
enum Message {
    FetchedProfile(Result<Profile, String>),
    Timeline(timeline::Message),
    Summoner(summoner::Message),
    SearchBar(search_bar::Message),
    RankedOverview(ranked_overview::Message),
}

impl Application for Aery {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let api_key =
            dotenv::var("RGAPI_KEY").expect("Unable to find `RGAPI_KEY` environment variable");
        let assets = Assets::new();

        (
            Self {
                client: core::Client::new(api_key),
                profile: None,

                timeline: Timeline::new(&assets),
                summoner: Summoner::new(5843),
                search_bar: SearchBar::new(),
                ranked_overview: RankedOverview::new(&assets),
                assets,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Aery")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::FetchedProfile(Ok(profile)) => {
                self.summoner = Summoner::from_profile(&profile);
                self.timeline = Timeline::from_profile(&self.assets, &profile);
                self.ranked_overview = RankedOverview::from_profile(&self.assets, &profile);
                self.profile = Some(profile);
            }
            Message::FetchedProfile(Err(error)) => panic!("failed: {error}"),
            Message::Timeline(message) => self.timeline.update(message),
            Message::Summoner(message) => {
                if let Some(event) = self.summoner.update(message) {
                    match event {
                        summoner::Event::UpdateProfile(name) => {
                            let client = self.client.clone();

                            return Command::perform(
                                core::Summoner::from_name(client, name),
                                |summoner| {
                                    Message::Summoner(summoner::Message::SummonerFetched(summoner))
                                },
                            );
                        }
                    }
                }
            }
            Message::SearchBar(message) => match self.search_bar.update(message) {
                Some(event) => match event {
                    search_bar::Event::SearchRequested(name) => {
                        let client = self.client.clone();

                        return Command::perform(
                            fetch_profile(client, name),
                            Message::FetchedProfile,
                        );
                    } // search_bar::Event::SearchRequested(content) => {
                      //     let client = self.client.clone();

                      //     return Command::perform(
                      //         aery_core::Summoner::from_name(client, content),
                      //         |summoner| {
                      //             Message::Summoner(summoner::Message::SummonerFetched(summoner))
                      //         },
                      //     );
                      // }
                },
                None => {}
            },
            Message::RankedOverview(message) => self.ranked_overview.update(message),
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let timeline = container(
            row![
                self.ranked_overview.view().map(Message::RankedOverview),
                self.timeline.view().map(Message::Timeline),
            ]
            .padding(8)
            .spacing(8),
        )
        .center_x()
        .width(Length::Fill);

        container(
            column![
                self.search_bar.view().map(Message::SearchBar),
                self.summoner.view().map(Message::Summoner),
                timeline,
            ]
            .spacing(16),
        )
        .style(theme::timeline_container())
        .into()
    }
}

async fn fetch_profile(client: core::Client, name: String) -> Result<Profile, String> {
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

    let mut games: Vec<core::GameMatch> = stream::iter(leagues.iter())
        .filter_map(|league| {
            summoner
                .matches(&client, 0..10, league.queue_kind())
                .map(Result::ok)
        })
        .flat_map(|game_ids| {
            stream::iter(game_ids)
                .filter_map(|id| core::GameMatch::from_id(&client, id).map(Result::ok))
        })
        .collect()
        .await;

    games.sort_unstable_by_key(|game| Reverse(*game.created_at().as_ref()));

    Ok(Profile {
        summoner,
        leagues,
        games,
    })
}
