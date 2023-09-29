mod assets;
mod component;
mod theme;
mod widget;

use iced::{
    widget::image::Handle,
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
    timeline: Timeline,
    summoner: Summoner,
    search_bar: SearchBar,
    ranked_overview: RankedOverview,
}

impl Aery {
    fn set_summoner_icon(&mut self, icon: u16) {
        let path = format!(
            "{}{}.png",
            concat!(env!("CARGO_MANIFEST_DIR"), "\\assets\\img\\profileicon\\"),
            icon
        );
        self.summoner.set_icon_handle(Handle::from_path(path));
    }
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
        let assets = Assets::new();

        (
            Self {
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
                        summoner::Event::FetchSummonerIcon(icon) => {
                            self.set_summoner_icon(icon);
                        }
                    }
                }
            }
            Message::SearchBar(message) => self.search_bar.update(message),
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
