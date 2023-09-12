mod theme;
mod widget;

use std::fs;
use std::{collections::HashMap, io::Read};

use iced::{
    widget::image::Handle,
    widget::{column, container, horizontal_space, row},
    Application, Command, Element, Length, Settings,
};

use widget::ranked_overview::{self, RankedOverview};
use widget::search_bar::{self, SearchBar};
use widget::summoner::{self, Summoner};
use widget::timeline::{self, Timeline};

pub fn main() -> iced::Result {
    Aery::run(Settings {
        antialiasing: true,
        ..Default::default()
    })
}

mod assets;

struct Aery {
    timeline: Timeline,
    summoner: Summoner,
    search_bar: SearchBar,
    ranked_overview: RankedOverview,
}

fn chevron_down_icon() -> Handle {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "\\assets\\img\\icons\\chevron-down-white.png"
    );
    Handle::from_path(path)
}

fn chevron_up_icon() -> Handle {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "\\assets\\img\\icons\\chevron-up-white.png"
    );
    Handle::from_path(path)
}

fn search_icon() -> Handle {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "\\assets\\img\\icons\\search-white.png"
    );
    Handle::from_path(path)
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
        let timer = std::time::Instant::now();
        let mut sprites = HashMap::default();
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "\\assets\\img\\sprite");
        let img_path = fs::read_dir(path).unwrap();
        for sprite in img_path {
            let file = sprite.unwrap();
            let sprite = {
                let name = file.file_name().into_string().unwrap();
                name.try_into().unwrap()
            };
            let image = image::io::Reader::open(file.path())
                .unwrap()
                .decode()
                .unwrap();

            sprites.insert(sprite, image);
        }
        println!("Loaded sprites in {:?}", timer.elapsed());

        let json_timer = std::time::Instant::now();
        let mut data = HashMap::default();
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "\\assets\\data");
        let data_path = fs::read_dir(path).unwrap();
        for data_dir in data_path {
            let file = data_dir.unwrap();
            let sprite = {
                let name = file.file_name().into_string().unwrap();
                name.try_into().unwrap()
            };
            let mut bytes = Vec::new();
            fs::File::open(file.path())
                .unwrap()
                .read_to_end(&mut bytes)
                .unwrap();
            let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();

            data.insert(sprite, value);
        }
        println!("Loaded JSON data in {:?}", json_timer.elapsed());

        let runes_timer = std::time::Instant::now();
        let mut runes = HashMap::default();
        let runes_path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "\\assets\\data\\runesReforged.json"
        );
        let value: serde_json::Value =
            serde_json::from_reader(fs::File::open(runes_path).unwrap()).unwrap();

        for value in value.as_array().unwrap() {
            let path = value["icon"]
                .as_str()
                .unwrap()
                .trim_start_matches("perk-images/");
            let name = value["name"].as_str().unwrap();
            runes.insert(name.to_string(), path.to_string());

            for slots in value["slots"].as_array().unwrap() {
                for rune in slots["runes"].as_array().unwrap() {
                    let path = rune["icon"]
                        .as_str()
                        .unwrap()
                        .trim_start_matches("perk-images/");
                    let name = rune["name"].as_str().unwrap();
                    runes.insert(name.to_string(), path.to_string());
                }
            }
        }
        println!("Loaded rune data in {:?}", runes_timer.elapsed());

        let emblem_timer = std::time::Instant::now();
        let mut emblems = HashMap::default();
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "\\assets\\img\\emblems");
        let img_path = fs::read_dir(path).unwrap();
        for sprite in img_path {
            let file = sprite.unwrap();
            let sprite = {
                let name = file.file_name().into_string().unwrap();
                name.try_into().unwrap()
            };
            let image = iced::widget::image::Handle::from_path(file.path());

            emblems.insert(sprite, image);
        }
        println!("Loaded emblems in {:?}", emblem_timer.elapsed());
        println!("Total time: {:?}", timer.elapsed());

        let assets = assets::Assets {
            sprites,
            data,
            runes,
            emblems,
        };

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
