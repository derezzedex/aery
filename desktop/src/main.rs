mod assets;
mod component;
mod theme;
mod widget;

use iced::{
    widget::{column, container, horizontal_space, row},
    Application, Command, Element, Length, Settings,
};

use assets::Assets;
use component::ranked_overview::{self, RankedOverview};
use component::search_bar::{self, SearchBar};
use component::summoner::{self, Summoner};
use component::timeline::{self, Timeline};

pub fn main() -> iced::Result {
    Aery::run(Settings {
        antialiasing: true,
        ..Default::default()
    })
}

struct Aery {
    client: aery_core::Client,
    timeline: Timeline,
    summoner: Summoner,
    search_bar: SearchBar,
    ranked_overview: RankedOverview,
}

#[derive(Debug, Clone)]
enum Message {
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
                client: aery_core::Client::new(api_key),
                timeline: Timeline::new(&assets),
                summoner: Summoner::new(5843),
                search_bar: SearchBar::new(),
                ranked_overview: RankedOverview::new(&assets),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Aery")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Timeline(message) => self.timeline.update(message),
            Message::Summoner(message) => {
                if let Some(event) = self.summoner.update(message) {
                    match event {
                        summoner::Event::UpdateProfile(name) => {
                            let client = self.client.clone();

                            return Command::perform(
                                aery_core::Summoner::from_name(client, name),
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
                    search_bar::Event::SearchRequested(content) => {
                        let client = self.client.clone();

                        return Command::perform(
                            aery_core::Summoner::from_name(client, content),
                            |summoner| {
                                Message::Summoner(summoner::Message::SummonerFetched(summoner))
                            },
                        );
                    }
                },
                None => {}
            },
            Message::RankedOverview(message) => self.ranked_overview.update(message),
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        container(
            column![
                self.search_bar.view().map(Message::SearchBar),
                self.summoner.view().map(Message::Summoner),
                row![
                    horizontal_space(Length::Fill),
                    self.ranked_overview.view().map(Message::RankedOverview),
                    self.timeline.view().map(Message::Timeline),
                    horizontal_space(Length::Fill),
                ]
                .padding(8)
                .spacing(8),
            ]
            .spacing(16),
        )
        .style(theme::timeline_container())
        .into()
    }
}
