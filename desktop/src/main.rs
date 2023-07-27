use std::fs;
use std::{collections::HashMap, io::Read};

use iced::{
    widget::image::Handle,
    widget::{column, container, horizontal_space, row},
    Application, Command, Element, Length, Settings,
};
use image::GenericImageView;
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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum DataFile {
    Champion,
    Item,
    ProfileIcon,
    RuneReforged,
    SummonerSpell,
}

impl TryFrom<String> for DataFile {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.split(".").next().unwrap();
        match value.to_ascii_lowercase().as_str() {
            "champion" => Ok(Self::Champion),
            "item" => Ok(Self::Item),
            "profileicon" => Ok(Self::ProfileIcon),
            "runesreforged" => Ok(Self::RuneReforged),
            "summoner" => Ok(Self::SummonerSpell),
            _ => Err("unknown data type"),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Sprite {
    Champion(u8),
    Item(u8),
    SummonerSpell(u8),
    ProfileIcon(u8),
    RuneReforged(u8),
}

impl TryFrom<String> for Sprite {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.split(".").next().unwrap();
        let mut size = 0;
        let index: u8 = value
            .chars()
            .filter(|c| c.is_digit(10))
            .inspect(|_| size += 1)
            .collect::<String>()
            .parse()
            .map_err(|_| "index is not u8")?;
        let value = value[..value.len() - size].to_string();

        match value.to_ascii_lowercase().as_str() {
            "champion" => Ok(Self::Champion(index)),
            "item" => Ok(Self::Item(index)),
            "profileicon" => Ok(Self::ProfileIcon(index)),
            "runereforged" => Ok(Self::RuneReforged(index)),
            "spell" => Ok(Self::SummonerSpell(index)),
            _ => Err("unknown sprite type"),
        }
    }
}

pub type SpriteMap = HashMap<Sprite, image::DynamicImage>;
pub type DataMap = HashMap<DataFile, serde_json::Value>;
pub type RuneMap = HashMap<String, String>;
pub type EmblemMap = HashMap<String, Handle>;

pub struct Assets {
    pub sprites: SpriteMap,
    pub data: DataMap,
    pub runes: RuneMap,
    pub emblems: EmblemMap,
}

#[derive(Debug, Clone)]
enum Event {
    Summoner(summoner::Event),
}

struct Aery {
    assets: Assets,

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

// TODO: use champion id instead of name
fn load_champion_icon(assets: &Assets, champion: &str) -> Handle {
    let icon_data = assets.data.get(&DataFile::Champion).unwrap();
    let icon = &icon_data["data"][champion]["image"];
    let sprite = Sprite::try_from(icon["sprite"].as_str().unwrap().to_string()).unwrap();
    let x = icon["x"].as_u64().unwrap() as u32;
    let y = icon["y"].as_u64().unwrap() as u32;
    let w = icon["w"].as_u64().unwrap() as u32;
    let h = icon["h"].as_u64().unwrap() as u32;
    let offset = 3;

    let icon_sprite = assets.sprites.get(&sprite).unwrap();
    let icon = icon_sprite.view(x + offset, y + offset, w - offset * 2, h - offset * 2);
    Handle::from_pixels(icon.width(), icon.height(), icon.to_image().into_vec())
}

fn load_summoner_spell_icon(assets: &Assets, summoner_spell: &str) -> Handle {
    let icon_data = assets.data.get(&DataFile::SummonerSpell).unwrap();
    let icon = &icon_data["data"][summoner_spell]["image"];
    let sprite = Sprite::try_from(icon["sprite"].as_str().unwrap().to_string()).unwrap();
    let x = icon["x"].as_u64().unwrap() as u32;
    let y = icon["y"].as_u64().unwrap() as u32;
    let w = icon["w"].as_u64().unwrap() as u32;
    let h = icon["h"].as_u64().unwrap() as u32;
    let offset = 0;

    let icon_sprite = assets.sprites.get(&sprite).unwrap();
    let icon = icon_sprite.view(x + offset, y + offset, w - offset * 2, h - offset * 2);
    Handle::from_pixels(icon.width(), icon.height(), icon.to_image().into_vec())
}

fn load_runes_icon(assets: &Assets, rune: &str) -> Handle {
    let rune_path = assets.runes.get(rune).unwrap();
    let mut path = std::path::PathBuf::from(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "\\assets\\img\\runes\\"
    ));
    path.push(rune_path);

    Handle::from_path(path)
}

fn load_item_icon(assets: &Assets, item_id: &str) -> Handle {
    let icon_data = assets.data.get(&DataFile::Item).unwrap();
    let icon = &icon_data["data"][item_id]["image"];
    let sprite = Sprite::try_from(icon["sprite"].as_str().unwrap().to_string()).unwrap();
    let x = icon["x"].as_u64().unwrap() as u32;
    let y = icon["y"].as_u64().unwrap() as u32;
    let w = icon["w"].as_u64().unwrap() as u32;
    let h = icon["h"].as_u64().unwrap() as u32;
    let offset = 0;

    let icon_sprite = assets.sprites.get(&sprite).unwrap();
    let icon = icon_sprite.view(x + offset, y + offset, w - offset * 2, h - offset * 2);
    Handle::from_pixels(icon.width(), icon.height(), icon.to_image().into_vec())
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

        let assets = Assets {
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

mod widget {
    use crate::theme;
    use iced::widget::image::Handle;
    use iced::widget::{container, Space};
    use iced::Length;

    #[derive(Debug, Clone)]
    enum Queue {
        RankedFlex,
        RankedSolo,
    }

    impl ToString for Queue {
        fn to_string(&self) -> String {
            match self {
                Queue::RankedFlex => "Ranked Flex",
                Queue::RankedSolo => "Ranked Solo",
            }
            .to_string()
        }
    }

    #[derive(Debug, Clone)]
    enum Role {
        Bottom,
        Jungle,
        Mid,
        Support,
        Top,
    }

    impl Role {
        pub fn icon(&self) -> Handle {
            let role = self.to_string().to_ascii_lowercase();
            let path = format!(
                "{}\\assets\\img\\position\\{role}.png",
                env!("CARGO_MANIFEST_DIR"),
            );

            Handle::from_path(path)
        }
    }

    impl ToString for Role {
        fn to_string(&self) -> String {
            match self {
                Role::Bottom => "Bottom",
                Role::Jungle => "Jungle",
                Role::Mid => "Mid",
                Role::Support => "Support",
                Role::Top => "Top",
            }
            .to_string()
        }
    }

    #[derive(Debug, Clone)]
    struct Time(time::OffsetDateTime);

    impl ToString for Time {
        fn to_string(&self) -> String {
            let now = time::OffsetDateTime::now_utc();
            let duration = now - self.0;
            let seconds = duration.whole_seconds();
            let minutes = seconds / 60;
            let hours = minutes / 60;
            let days = hours / 24;
            let weeks = days / 7;
            let months = days / 30;
            let years = days / 365;

            if seconds < 60 {
                String::from("few seconds ago")
            } else if minutes < 60 {
                format!(
                    "{} minute{} ago",
                    minutes,
                    if minutes == 1 { "" } else { "s" }
                )
            } else if hours < 24 {
                format!("{} hour{} ago", hours, if hours == 1 { "" } else { "s" })
            } else if days < 7 {
                if days == 1 {
                    String::from("yesterday")
                } else {
                    format!("{} days ago", days)
                }
            } else if weeks < 4 {
                if weeks == 1 {
                    String::from("last week")
                } else {
                    format!("{} weeks ago", weeks)
                }
            } else if months < 12 {
                if months == 1 {
                    String::from("last month")
                } else {
                    format!("{} months ago", months)
                }
            } else {
                if years == 1 {
                    return String::from("last year");
                } else {
                    format!("{} years ago", years)
                }
            }
        }
    }

    #[derive(Debug, Clone)]
    struct Duration(time::Duration);

    impl ToString for Duration {
        fn to_string(&self) -> String {
            let minutes = self.0.whole_minutes();
            let seconds = self.0.whole_seconds();

            format!("{minutes}:{seconds}")
        }
    }

    #[derive(Debug, Clone)]
    pub struct Champion(u16);

    #[derive(Debug, Clone, Copy)]
    struct Item(u16);

    #[derive(Debug, Clone)]
    struct Inventory([Option<Item>; 6]);

    #[derive(Debug, Clone)]
    struct Summoner(String);

    impl ToString for Summoner {
        fn to_string(&self) -> String {
            self.0.clone()
        }
    }

    mod formatting {
        pub fn win(win: bool) -> String {
            if win { "Victory" } else { "Defeat" }.to_string()
        }

        pub fn kda(kills: u16, deaths: u16, assists: u16) -> String {
            let kda = (kills as f32 + assists as f32) / deaths as f32;
            format!("{kda:.2} KDA")
        }

        pub fn creep_score(creep_score: u16, minutes: u16) -> String {
            let cs_per_minute = creep_score as f32 / minutes as f32;

            format!("{creep_score} CS ({cs_per_minute:.1})")
        }

        pub fn vision_score(vision_score: u16) -> String {
            format!("{vision_score} vision")
        }
    }

    pub mod search_bar {
        use iced::{
            widget::{button, container, horizontal_space, image, row, text, text_input, Space},
            Alignment, Element, Length,
        };

        use crate::{search_icon, theme};

        use super::medium_large_icon;

        #[derive(Clone, Debug)]
        pub enum Message {
            TextChanged(String),
            SearchPressed,
            RegionPressed,
        }

        pub struct SearchBar {
            text: String,
        }

        fn logo<'a, Message: 'a>() -> iced::widget::Container<'a, Message> {
            container(Space::new(28.0, 28.0))
                .style(theme::icon_container())
                .max_width(28.0)
                .max_height(28.0)
        }

        impl SearchBar {
            pub fn new() -> SearchBar {
                SearchBar {
                    text: String::new(),
                }
            }

            pub fn update(&mut self, message: Message) {
                match message {
                    Message::TextChanged(text) => self.text = text,
                    Message::SearchPressed => todo!(),
                    Message::RegionPressed => todo!(),
                }
            }

            pub fn view(&self) -> Element<Message> {
                let region = "BR";

                let search = container(image(search_icon()).width(12.0).height(12.0)).padding(2);

                container(row![
                    logo(),
                    horizontal_space(Length::FillPortion(2)),
                    container(
                        row![
                            text_input("Search for a summoner or champion", &self.text)
                                .on_input(Message::TextChanged)
                                .style(theme::search_bar_text_input())
                                .size(12),
                            button(text(region).size(10))
                                .width(Length::Shrink)
                                .padding([2, 4, 2, 4])
                                .style(theme::region_button())
                                .on_press(Message::RegionPressed),
                            button(search)
                                .style(iced::theme::Button::Text)
                                .on_press(Message::SearchPressed),
                        ]
                        .align_items(Alignment::Center)
                    )
                    .style(theme::search_bar_container())
                    .width(Length::FillPortion(4)),
                    horizontal_space(Length::FillPortion(2))
                ])
                .padding(8)
                .style(theme::dark_container())
                .into()
            }
        }
    }

    pub mod summoner {
        use crate::text;
        use crate::theme;
        use crate::widget::bold;
        use iced::alignment;
        use iced::widget::column;
        use iced::widget::{button, container, image, row, text, vertical_space};
        use iced::Element;
        use iced::Length;

        #[derive(Debug, Clone)]
        pub enum Message {
            Update,
        }

        #[allow(dead_code)]
        #[derive(Debug, Copy, Clone)]
        pub enum Tier {
            Iron(Division),
            Bronze(Division),
            Silver(Division),
            Gold(Division),
            Platinum(Division),
            Diamond(Division),
            Master(u16),
            Grandmaster(u16),
            Challenger(u16),
        }

        impl Tier {
            pub fn points(&self) -> u16 {
                match self {
                    Tier::Challenger(points) | Tier::Grandmaster(points) | Tier::Master(points) => {
                        *points
                    }
                    Tier::Iron(division)
                    | Tier::Bronze(division)
                    | Tier::Silver(division)
                    | Tier::Gold(division)
                    | Tier::Platinum(division)
                    | Tier::Diamond(division) => match division {
                        Division::One(points)
                        | Division::Two(points)
                        | Division::Three(points)
                        | Division::Four(points) => *points as u16,
                    },
                }
            }

            pub fn division(&self) -> String {
                match self {
                    Tier::Iron(division)
                    | Tier::Bronze(division)
                    | Tier::Silver(division)
                    | Tier::Gold(division)
                    | Tier::Platinum(division)
                    | Tier::Diamond(division) => division.to_string(),
                    Tier::Master(points) | Tier::Grandmaster(points) | Tier::Challenger(points) => {
                        points.to_string()
                    }
                }
            }
        }

        impl ToString for Tier {
            fn to_string(&self) -> String {
                match self {
                    Tier::Iron(_) => "Iron",
                    Tier::Bronze(_) => "Bronze",
                    Tier::Silver(_) => "Silver",
                    Tier::Gold(_) => "Gold",
                    Tier::Platinum(_) => "Platinum",
                    Tier::Diamond(_) => "Diamond",
                    Tier::Master(_) => "Master",
                    Tier::Grandmaster(_) => "Grandmaster",
                    Tier::Challenger(_) => "Challenger",
                }
                .to_string()
            }
        }

        #[allow(dead_code)]
        #[derive(Debug, Copy, Clone)]
        pub enum Division {
            One(u8),
            Two(u8),
            Three(u8),
            Four(u8),
        }

        impl ToString for Division {
            fn to_string(&self) -> String {
                match self {
                    Division::One(_) => "1",
                    Division::Two(_) => "2",
                    Division::Three(_) => "3",
                    Division::Four(_) => "4",
                }
                .to_string()
            }
        }

        fn summoner_icon<'a>(icon: Option<image::Handle>, level: u16) -> Element<'a, Message> {
            let image: Element<Message> = if let Some(handle) = icon {
                image(handle).into()
            } else {
                vertical_space(96.0).into()
            };

            column![
                container(image)
                    .width(96.0)
                    .height(96.0)
                    .style(theme::summoner_icon_container()),
                container(bold(level).size(12))
                    .padding(4)
                    .style(theme::summoner_level_container())
                    .center_x(),
            ]
            .align_items(iced::Alignment::Center)
            .into()
        }

        #[derive(Debug, Clone)]
        pub enum Event {
            FetchSummonerIcon(u16),
        }

        pub struct Summoner {
            icon: u16,
            icon_image: Option<image::Handle>,
        }

        impl Summoner {
            pub fn new(icon: u16) -> Self {
                Summoner {
                    icon,
                    icon_image: None,
                }
            }

            pub fn set_icon_handle(&mut self, handle: image::Handle) {
                self.icon_image = Some(handle);
            }

            pub fn update(&mut self, message: Message) -> Option<Event> {
                match message {
                    Message::Update => Some(Event::FetchSummonerIcon(self.icon)),
                }
            }

            pub fn view(&self) -> Element<Message> {
                let summoner_level = 466;
                let icon = summoner_icon(self.icon_image.clone(), summoner_level);
                let past_ranks = {
                    let ranks = [
                        (12, Tier::Iron(Division::One(0))),
                        (11, Tier::Bronze(Division::Four(100))),
                        (10, Tier::Silver(Division::One(0))),
                        (9, Tier::Gold(Division::Four(100))),
                        (8, Tier::Platinum(Division::One(0))),
                        (7, Tier::Diamond(Division::Four(100))),
                        (6, Tier::Master(150)),
                        (5, Tier::Grandmaster(600)),
                        (4, Tier::Challenger(2000)),
                    ];

                    row(ranks
                        .into_iter()
                        .map(|(season, rank)| {
                            let division = rank.division();
                            let tier = rank.to_string();

                            container(
                                row![
                                    container(bold(format!("S{season}")).size(10))
                                        .padding([0, 2, 2, 2])
                                        .style(theme::past_rank_badge_container()),
                                    container(
                                        text!("{tier} {division}")
                                            .size(10)
                                            .style(theme::tier_color(rank))
                                    )
                                    .padding([0, 2, 2, 2]),
                                ]
                                .align_items(iced::Alignment::Center)
                                .spacing(2),
                            )
                            .style(theme::past_rank_container())
                            .into()
                        })
                        .collect())
                    .spacing(2)
                };

                let name = text("SynxTrak")
                    .size(24)
                    .vertical_alignment(alignment::Vertical::Center);

                let rank = 241768;
                let ladder_rank = rank
                    .to_string()
                    .as_bytes()
                    .rchunks(3)
                    .rev()
                    .map(std::str::from_utf8)
                    .collect::<Result<Vec<&str>, _>>()
                    .unwrap()
                    .join(",");
                let rank_percentage = 24.7;
                let ladder = row![
                    text("Ladder rank").size(12).style(theme::gray_text()),
                    text!("{ladder_rank}").size(12),
                    text!("(top {rank_percentage}%)")
                        .size(12)
                        .style(theme::gray_text()),
                ]
                .spacing(4);

                let update_button = button("Update")
                    .style(theme::update_button())
                    .on_press(Message::Update);

                container(
                    column![
                        past_ranks,
                        row![
                            icon,
                            column![
                                column![name, ladder],
                                container(update_button)
                                    .height(48)
                                    .align_y(alignment::Vertical::Bottom)
                            ]
                            .spacing(1)
                        ]
                        .spacing(16)
                    ]
                    .spacing(8)
                    .width(Length::Fill)
                    .max_width(920)
                    .padding([8, 0, 8, 0]),
                )
                .center_x()
                .width(Length::Fill)
                .style(theme::timeline_container())
                .into()
            }
        }
    }

    pub mod ranked_overview {
        use iced::{
            widget::{
                button, column, container, horizontal_space, image, progress_bar, row, text,
                vertical_space,
            },
            Alignment, Element, Length,
        };

        use crate::{chevron_down_icon, theme};

        use super::{
            bold, large_icon, medium_icon,
            summoner::{Division, Tier},
            Queue,
        };

        fn ranked_container<'a>(
            queue: Queue,
            tier: Tier,
            wins: u16,
            losses: u16,
            handle: image::Handle,
        ) -> Element<'a, Message> {
            let left_bar = container(horizontal_space(2))
                .style(theme::left_bar_container())
                .height(18);

            let chevron_down = image(chevron_down_icon()).width(10.0).height(10.0);

            let size = match queue {
                Queue::RankedSolo => 100.0,
                Queue::RankedFlex => 80.0,
            };
            let emblem_size = match queue {
                Queue::RankedSolo => match tier {
                    Tier::Challenger(_) | Tier::Grandmaster(_) | Tier::Master(_) => 100.0,
                    Tier::Diamond(_) => 90.0,
                    Tier::Platinum(_) | Tier::Gold(_) | Tier::Silver(_) => 80.0,
                    Tier::Bronze(_) | Tier::Iron(_) => 70.0,
                },
                Queue::RankedFlex => match tier {
                    Tier::Challenger(_) | Tier::Grandmaster(_) | Tier::Master(_) => 80.0,
                    Tier::Diamond(_) => 70.0,
                    Tier::Platinum(_) | Tier::Gold(_) | Tier::Silver(_) => 60.0,
                    Tier::Bronze(_) | Tier::Iron(_) => 50.0,
                },
            };
            let lp = tier.points();
            let tier = match tier {
                Tier::Challenger(_) | Tier::Grandmaster(_) | Tier::Master(_) => tier.to_string(),
                _ => format!("{} {}", tier.to_string(), tier.division()),
            };

            let win_rate = ((wins as f32 / (wins + losses) as f32) * 100.0).ceil();

            container(column![
                row![
                    left_bar,
                    horizontal_space(4),
                    bold(queue.to_string()).size(14),
                    horizontal_space(Length::Fill),
                    button(chevron_down)
                        .style(theme::expand_button())
                        .padding(4)
                        .on_press(Message::Expand),
                ]
                .padding([12, 12, 0, 12])
                .spacing(2)
                .align_items(Alignment::Center),
                row![
                    container(image(handle).width(emblem_size).height(emblem_size))
                        .width(size)
                        .height(size)
                        .center_x()
                        .center_y(),
                    column![
                        row![
                            bold(tier).size(16),
                            text("·").style(theme::sub_text()).size(16),
                            text(format!("{lp} LP")).style(theme::sub_text()).size(12)
                        ]
                        .align_items(Alignment::Center)
                        .spacing(4),
                        row![
                            text(format!("{wins}W {losses}L"))
                                .style(theme::sub_text())
                                .size(12),
                            text("·").style(theme::sub_text()),
                            bold(format!("{win_rate:.0}%"))
                                .style(theme::blue_text())
                                .size(12)
                        ]
                        .align_items(Alignment::Center)
                        .spacing(4),
                        progress_bar(0.0..=100.0, win_rate)
                            .width(120)
                            .height(4)
                            .style(theme::ratio_bar()),
                    ]
                    .spacing(2)
                ]
                .padding([0, 18, 0, 18])
                .spacing(16)
                .align_items(Alignment::Center),
            ])
            .style(theme::dark_container())
            .width(280)
            .into()
        }

        #[derive(Debug, Clone)]
        pub enum Message {
            Expand,
        }

        pub struct RankedOverview {
            ranked_solo_image: image::Handle,
            ranked_flex_image: image::Handle,
        }

        impl RankedOverview {
            pub fn new(assets: &crate::Assets) -> RankedOverview {
                RankedOverview {
                    ranked_solo_image: assets.emblems.get("emblem-challenger.png").unwrap().clone(),
                    ranked_flex_image: assets.emblems.get("emblem-iron.png").unwrap().clone(),
                }
            }

            pub fn update(&mut self, _message: Message) {}

            pub fn view(&self) -> Element<Message> {
                column![
                    ranked_container(
                        Queue::RankedSolo,
                        Tier::Challenger(650),
                        295,
                        208,
                        self.ranked_solo_image.clone()
                    ),
                    ranked_container(
                        Queue::RankedFlex,
                        Tier::Iron(Division::Four(39)),
                        21,
                        13,
                        self.ranked_flex_image.clone()
                    ),
                ]
                .spacing(4)
                .align_items(Alignment::Center)
                .into()
            }
        }
    }

    pub mod timeline {
        use crate::load_champion_icon;

        use self::summary::Summary;

        use super::game::{self, Game};
        use super::{theme, Role};
        use iced::widget::{column, container, scrollable};
        use iced::{Alignment, Element, Length};

        #[derive(Debug, Clone, Copy)]
        pub enum Message {
            Game(usize, game::Message),
        }

        #[derive(Debug, Clone)]
        pub struct Timeline {
            summary: Summary,
            games: Vec<Game>,
        }

        impl Timeline {
            pub fn new(assets: &crate::Assets) -> Self {
                let champions = vec![
                    summary::Champion {
                        handle: load_champion_icon(assets, "TwistedFate"),
                        wins: 2,
                        losses: 1,
                        kda: 1.15,
                        lane: Role::Mid.icon(),
                    },
                    summary::Champion {
                        handle: load_champion_icon(assets, "Orianna"),
                        wins: 3,
                        losses: 0,
                        kda: 2.0,
                        lane: Role::Bottom.icon(),
                    },
                    summary::Champion {
                        handle: load_champion_icon(assets, "Annie"),
                        wins: 2,
                        losses: 2,
                        kda: 3.0,
                        lane: Role::Support.icon(),
                    },
                    summary::Champion {
                        handle: load_champion_icon(assets, "Sion"),
                        wins: 0,
                        losses: 3,
                        kda: 0.5,
                        lane: Role::Top.icon(),
                    },
                ];

                Timeline {
                    summary: Summary::new(champions),
                    games: (0..5)
                        .into_iter()
                        .map(|_| {
                            [
                                Game::new(true, assets, "Annie"),
                                Game::new(false, assets, "Sion"),
                                Game::new(true, assets, "Darius"),
                                Game::new(false, assets, "KSante"),
                                Game::new(false, assets, "MonkeyKing"),
                            ]
                        })
                        .flatten()
                        .collect(),
                }
            }

            pub fn update(&mut self, message: Message) {
                match message {
                    Message::Game(index, message) => unsafe {
                        self.games.get_unchecked_mut(index).update(message);
                    },
                }
            }

            pub fn view(&self) -> Element<Message> {
                let games = self
                    .games
                    .iter()
                    .enumerate()
                    .map(|(i, game)| game.view().map(move |message| Message::Game(i, message)))
                    .collect();

                let content = column(games)
                    .width(Length::Fill)
                    .padding([0, 12, 0, 0])
                    .spacing(4)
                    .align_items(Alignment::Center);

                let summary = self.summary.view();
                let timeline = column![
                    summary,
                    scrollable(content)
                        .style(theme::scrollable())
                        .width(Length::Fill)
                        .height(Length::FillPortion(9))
                ]
                .max_width(640)
                .align_items(Alignment::Center)
                .spacing(4);

                container(timeline)
                    .style(theme::timeline_container())
                    .into()
            }
        }

        pub mod summary {
            use super::theme;
            use super::Message;
            use crate::text;
            use crate::widget;
            use crate::widget::medium_large_icon;
            use crate::widget::very_small_icon;
            use crate::widget::Role;
            use iced::alignment;
            use iced::widget::image;
            use iced::widget::image::Handle;
            use iced::widget::{column, container, horizontal_rule, progress_bar, row, text};
            use iced::{Alignment, Element};

            trait Fit {
                fn fit(self, size: u16) -> Self;
            }

            impl<'a> Fit for iced::widget::Text<'a> {
                fn fit(self, size: u16) -> iced::widget::Text<'a> {
                    self.size(size)
                        .line_height(1.1)
                        .vertical_alignment(alignment::Vertical::Center)
                }
            }

            #[derive(Debug, Clone)]
            pub struct Champion {
                pub handle: Handle,
                pub lane: Handle,
                pub wins: i16,
                pub losses: i16,
                pub kda: f32,
            }

            #[derive(Debug, Clone)]
            pub struct Summary {
                wins: i8,
                losses: i8,
                ratio: f32,
                kill_ratio: f32,
                death_ratio: f32,
                assist_ratio: f32,

                champions: Vec<Champion>,
            }

            impl Summary {
                pub fn new(champions: Vec<Champion>) -> Summary {
                    let wins = 6;
                    let losses = 4;
                    let ratio = (wins as f32 / (wins + losses) as f32) * 100.0;
                    let kill_ratio = 2.7;
                    let death_ratio = 6.7;
                    let assist_ratio = 7.0;

                    Summary {
                        wins,
                        losses,
                        ratio,
                        kill_ratio,
                        death_ratio,
                        assist_ratio,
                        champions,
                    }
                }

                pub fn view(&self) -> Element<Message> {
                    let total = self.wins + self.losses;
                    let is_positive_ratio = self.ratio > 50.0;

                    let title_bar = row![
                        widget::bold("Recent summary").size(12),
                        text!("last {total} games")
                            .style(theme::gray_text())
                            .size(10)
                    ]
                    .padding([2, 6, 0, 6])
                    .align_items(Alignment::Center)
                    .spacing(4);

                    let summary_ratio = {
                        let ratio_text = row![
                            row![
                                row![
                                    text!("{}", self.wins).fit(12),
                                    text("W").fit(12).style(theme::gray_text())
                                ]
                                .spacing(1),
                                row![
                                    text!("{}", self.losses).fit(12),
                                    text("L").fit(12).style(theme::gray_text())
                                ]
                            ]
                            .spacing(4),
                            text("·").fit(18).style(theme::sub_text()),
                            text!("{:.1}%", self.ratio)
                                .fit(12)
                                .style(theme::win_color(is_positive_ratio)),
                        ]
                        .align_items(Alignment::Center)
                        .spacing(4);

                        let ratio_bar = progress_bar(0.0..=100.0, self.ratio)
                            .width(80.0)
                            .height(4.0)
                            .style(theme::ratio_bar());

                        column![
                            text("Winrate").fit(10).style(theme::gray_text()),
                            ratio_text,
                            ratio_bar,
                        ]
                        .spacing(4)
                    };

                    let summary_lane = {
                        let lane_icon = image(Role::Mid.icon())
                            .width(24.0)
                            .height(24.0)
                            .content_fit(iced::ContentFit::Fill);

                        let lane_info = column![
                            row![
                                row![
                                    row![
                                        text!("{}", self.wins).fit(12),
                                        text("W").fit(12).style(theme::gray_text())
                                    ]
                                    .spacing(1),
                                    row![
                                        text!("{}", self.losses).fit(12),
                                        text("L").fit(12).style(theme::gray_text())
                                    ]
                                ]
                                .spacing(4),
                                text("·").fit(18).style(theme::sub_text()),
                                text!("{:.1}%", self.ratio)
                                    .fit(12)
                                    .style(theme::win_color(is_positive_ratio)),
                            ]
                            .align_items(Alignment::Center)
                            .spacing(4),
                            row![
                                text!("{:.1}", self.kill_ratio).size(10),
                                text("/").size(10).style(theme::gray_text()),
                                text!("{:.1}", self.death_ratio).size(10),
                                text("/").size(10).style(theme::gray_text()),
                                text!("{:.1}", self.assist_ratio).size(10),
                                row![
                                    text("(").size(10).style(theme::red_text()),
                                    text!(
                                        "{:.1} KDA",
                                        self.death_ratio + self.assist_ratio / self.kill_ratio
                                    )
                                    .size(10)
                                    .style(theme::red_text()),
                                    text(")").size(10).style(theme::red_text())
                                ],
                            ]
                            .spacing(2)
                            .align_items(Alignment::Start),
                        ];

                        column![
                            text("Lane").size(10).height(13).style(theme::gray_text()),
                            row![lane_icon, lane_info]
                                .align_items(Alignment::Center)
                                .spacing(4)
                        ]
                        .spacing(2)
                    };

                    let summary_champions = {
                        let content: Vec<Element<Message>> = self
                            .champions
                            .iter()
                            .map(|champion| {
                                let icon = iced::widget::image(champion.handle.clone())
                                    .width(24.0)
                                    .height(24.0)
                                    .content_fit(iced::ContentFit::Fill);
                                let winrate = champion.wins as f32 * 100.0
                                    / (champion.wins + champion.losses) as f32;

                                row![
                                    icon,
                                    // TODO: fix strange alignment between bottom and top text
                                    column![
                                        row![
                                            text!("{:.1}%", winrate)
                                                .size(10)
                                                .style(theme::win_color(winrate > 50.0)),
                                            text!("({}W {}L)", champion.wins, champion.losses)
                                                .size(10)
                                                .style(theme::gray_text())
                                        ]
                                        .align_items(Alignment::Center)
                                        .spacing(2),
                                        row![
                                            image(champion.lane.clone()).width(12.0).height(12.0),
                                            text!("{:.2} KDA", champion.kda)
                                                .size(10)
                                                .style(theme::gray_text())
                                        ]
                                        .spacing(2)
                                        .align_items(Alignment::Center),
                                    ]
                                ]
                                .align_items(Alignment::Center)
                                .spacing(4)
                                .into()
                            })
                            .collect();

                        column![
                            text("Champions").size(10).style(theme::gray_text()),
                            row(content).spacing(8).align_items(Alignment::Center)
                        ]
                        .spacing(8)
                    };

                    let body = container(
                        row![summary_ratio, summary_lane, summary_champions]
                            .spacing(16)
                            .align_items(Alignment::Start),
                    )
                    .padding(8)
                    .center_y();

                    let content = column![
                        title_bar,
                        container(horizontal_rule(2).style(theme::rule(theme::gray_text())))
                            .width(iced::Length::Fill)
                            .padding([0, 4, 0, 4]),
                        body
                    ];

                    container(content)
                        .width(iced::Length::Fill)
                        .style(theme::dark_container())
                        .into()
                }
            }
        }
    }

    pub mod game {
        use super::*;
        use crate::chevron_down_icon;
        use crate::chevron_up_icon;
        use crate::load_champion_icon;
        use crate::load_item_icon;
        use crate::load_runes_icon;
        use crate::load_summoner_spell_icon;
        use crate::theme;
        use crate::widget;
        use iced::widget::image;
        use iced::widget::{button, column, container, row, text, Space};
        use iced::{alignment, Alignment, Element, Length};

        fn champion_icon<'a>(handle: image::Handle) -> Element<'a, Message> {
            let icon = iced::widget::image(handle)
                .width(48.0)
                .height(48.0)
                .content_fit(iced::ContentFit::Fill);

            container(icon).width(48.0).height(48.0).into()
        }

        fn summoner_spell_icon<'a>(handle: image::Handle) -> Element<'a, Message> {
            let icon = iced::widget::image(handle)
                .width(22.0)
                .height(22.0)
                .content_fit(iced::ContentFit::Fill);

            container(icon).width(22.0).height(22.0).into()
        }

        fn summoner_rune_icon<'a>(handle: image::Handle) -> Element<'a, Message> {
            let icon = iced::widget::image(handle)
                .width(30.0)
                .height(30.0)
                .content_fit(iced::ContentFit::Cover);

            container(icon).width(22.0).height(22.0).into()
        }

        fn summoner_rune2_icon<'a>(handle: image::Handle) -> Element<'a, Message> {
            let icon = iced::widget::image(handle)
                .width(16.0)
                .height(16.0)
                .content_fit(iced::ContentFit::Fill);

            container(icon)
                .center_x()
                .center_y()
                .width(22.0)
                .height(22.0)
                .into()
        }

        fn item_icon<'a>(handle: image::Handle) -> Element<'a, Message> {
            let icon = iced::widget::image(handle)
                .width(22.0)
                .height(22.0)
                .content_fit(iced::ContentFit::Fill);

            container(icon).width(22.0).height(22.0).into()
        }

        #[derive(Debug, Clone)]
        pub struct Game {
            win: bool,
            queue: Queue,
            time: Time,
            duration: Duration,
            role: Option<Role>,
            champion_image: image::Handle,
            summoner_spell_images: [image::Handle; 2],
            runes_images: [image::Handle; 2],
            item_images: [image::Handle; 7],
            summoner_icons: [image::Handle; 10],
            player_kills: u16,
            player_deaths: u16,
            player_assists: u16,
            player_creep_score: u16,
            player_vision_score: u16,
            summoners: Vec<Summoner>,

            is_expanded: bool,
        }

        #[derive(Debug, Clone, Copy)]
        pub enum Message {
            ExpandPressed,
        }

        impl Game {
            pub fn new(win: bool, assets: &crate::Assets, champion: &'static str) -> Self {
                let champion_image = load_champion_icon(assets, champion);
                let summoner_spell_images = [
                    load_summoner_spell_icon(assets, "SummonerDot"),
                    load_summoner_spell_icon(assets, "SummonerFlash"),
                ];
                let runes_images = [
                    load_runes_icon(assets, "Conqueror"),
                    load_runes_icon(assets, "Resolve"),
                ];
                let item_images = [
                    load_item_icon(assets, "1001"),
                    load_item_icon(assets, "6630"),
                    load_item_icon(assets, "4401"),
                    load_item_icon(assets, "3143"),
                    load_item_icon(assets, "3742"),
                    load_item_icon(assets, "6333"),
                    load_item_icon(assets, "3364"),
                ];

                let summoner_icons = [
                    load_champion_icon(assets, champion),
                    load_champion_icon(assets, "Annie"),
                    load_champion_icon(assets, "Xerath"),
                    load_champion_icon(assets, "Sion"),
                    load_champion_icon(assets, "Darius"),
                    load_champion_icon(assets, "KSante"),
                    load_champion_icon(assets, "MonkeyKing"),
                    load_champion_icon(assets, "TwistedFate"),
                    load_champion_icon(assets, "Orianna"),
                    load_champion_icon(assets, "Jhin"),
                ];

                Game {
                    win,
                    queue: Queue::RankedFlex,
                    time: Time(
                        time::OffsetDateTime::now_utc().saturating_sub(time::Duration::days(1)),
                    ),
                    duration: Duration(
                        time::Duration::minutes(28).saturating_add(time::Duration::seconds(33)),
                    ),
                    role: Some(Role::Mid),
                    champion_image,
                    summoner_spell_images,
                    runes_images,
                    item_images,
                    summoner_icons,
                    player_kills: 1,
                    player_deaths: 6,
                    player_assists: 12,
                    player_creep_score: 151,
                    player_vision_score: 18,
                    summoners: (0..10)
                        .map(|i| Summoner(format!("Summoner {}", i)))
                        .collect(),

                    is_expanded: false,
                }
            }
            pub fn update(&mut self, message: Message) {
                match message {
                    Message::ExpandPressed => self.is_expanded = !self.is_expanded,
                }
            }

            pub fn view(&self) -> Element<Message> {
                let match_stats = {
                    // TODO: track and display points gained/lost
                    // let points_icon: Element<Message> = small_icon().into();
                    // let result_points = row![points_icon, text("31 LP").size(16)]
                    //     .spacing(2)
                    //     .align_items(Alignment::Center);

                    let role: Element<_> = if let Some(role) = &self.role {
                        column![
                            row![
                                image(role.icon()).width(12.0).height(12.0),
                                text(role.to_string()).style(theme::sub_text()).size(10),
                            ]
                            .align_items(Alignment::Center)
                            .spacing(2),
                            text("28:33").size(10).style(theme::sub_text()),
                        ]
                        .padding([4, 0, 0, 0])
                        .into()
                    } else {
                        Space::new(0, 0).into()
                    };

                    column![
                        column![
                            widget::bold(formatting::win(self.win))
                                .style(if self.win {
                                    theme::blue_text()
                                } else {
                                    theme::red_text()
                                })
                                .size(16),
                            text(self.queue.to_string()).size(12),
                            text(self.time.to_string())
                                .style(theme::sub_text())
                                .size(10),
                        ],
                        role
                    ]
                    .align_items(Alignment::Start)
                    .spacing(2)
                    .padding(4)
                };

                let champion_info = {
                    let champion_icon = champion_icon(self.champion_image.clone());

                    let champion_spells = row![
                        summoner_spell_icon(self.summoner_spell_images[0].clone()),
                        summoner_spell_icon(self.summoner_spell_images[1].clone())
                    ]
                    .spacing(2);

                    let champion_runes = row![
                        summoner_rune_icon(self.runes_images[0].clone()),
                        summoner_rune2_icon(self.runes_images[1].clone())
                    ]
                    .spacing(2);

                    row![
                        champion_icon,
                        column![champion_spells, champion_runes].spacing(2),
                    ]
                    .spacing(2)
                };

                let player_stats = {
                    let kda = row![
                        text(self.player_kills).size(12),
                        text("/").style(theme::gray_text()).size(12),
                        text(self.player_deaths).style(theme::red_text()).size(12),
                        text("/").style(theme::gray_text()).size(12),
                        text(self.player_assists).size(12)
                    ]
                    .align_items(Alignment::Center)
                    .spacing(3);

                    let other_stats = column![
                        row![text(formatting::kda(
                            self.player_kills,
                            self.player_deaths,
                            self.player_assists
                        ))
                        .size(10)
                        .style(theme::sub_text())]
                        .spacing(4)
                        .align_items(Alignment::Center),
                        row![text(formatting::creep_score(
                            self.player_creep_score,
                            self.duration.0.whole_minutes() as u16
                        ))
                        .size(10)
                        .style(theme::sub_text())]
                        .spacing(4)
                        .align_items(Alignment::Center),
                        row![text(formatting::vision_score(self.player_vision_score))
                            .size(10)
                            .style(theme::sub_text())]
                        .spacing(4)
                        .align_items(Alignment::Center),
                    ]
                    .align_items(Alignment::Center);

                    column![kda, other_stats,].align_items(Alignment::Center)
                };

                let player_items = {
                    row![
                        column![
                            item_icon(self.item_images[0].clone()),
                            item_icon(self.item_images[1].clone())
                        ]
                        .spacing(2),
                        column![
                            item_icon(self.item_images[2].clone()),
                            item_icon(self.item_images[3].clone())
                        ]
                        .spacing(2),
                        column![
                            item_icon(self.item_images[4].clone()),
                            item_icon(self.item_images[5].clone())
                        ]
                        .spacing(2),
                        item_icon(self.item_images[6].clone()),
                    ]
                    .spacing(2)
                };

                let player_i = 0;
                let mut left_players: Vec<Element<_>> = self
                    .summoners
                    .iter()
                    .enumerate()
                    .map(|(i, summoner)| {
                        let summoner_icon = image(self.summoner_icons[i].clone())
                            .width(14.0)
                            .height(14.0);
                        let summoner_name = if player_i == i {
                            bold(summoner.to_string()).size(8.0)
                        } else {
                            small_text(summoner.to_string())
                        };

                        row![summoner_icon, summoner_name]
                            .align_items(Alignment::Center)
                            .spacing(4)
                            .into()
                    })
                    .collect();

                let right_players = left_players.split_off(5);

                let other_players = {
                    row![
                        column(left_players).spacing(2),
                        column(right_players).spacing(2),
                    ]
                    .spacing(8)
                };

                let chevron_icon = if self.is_expanded {
                    chevron_up_icon()
                } else {
                    chevron_down_icon()
                };

                let expand_content = container(image(chevron_icon).width(8.0).height(8.0))
                    .center_x()
                    .align_y(alignment::Vertical::Bottom)
                    .height(Length::Fill)
                    .width(24)
                    .padding(2);

                let expand_button = button(expand_content)
                    .height(Length::Fill)
                    .on_press(Message::ExpandPressed)
                    .style(theme::expander_button(self.is_expanded));

                let overview = container(row![
                    row![
                        match_stats,
                        champion_info,
                        player_stats,
                        player_items,
                        other_players,
                    ]
                    .width(Length::Fill)
                    .spacing(32)
                    .padding(4)
                    .align_items(Alignment::Center),
                    expand_button.padding(0),
                ])
                .max_height(100.0);

                let game = if self.is_expanded {
                    let match_details = container(Space::new(0.0, 400.0));

                    row![
                        left_border(self.win).max_height(600.0),
                        column![overview, match_details,]
                    ]
                } else {
                    row![left_border(self.win).max_height(100.0), column![overview]]
                };

                container(game)
                    .width(Length::Fill)
                    .style(theme::dark_container())
                    .into()
            }
        }
    }

    #[macro_export]
    macro_rules! text {
        ($($arg:tt)*) => {
            iced::widget::Text::new(format!($($arg)*))
        }
    }

    fn bold<'a>(text: impl ToString) -> iced::widget::Text<'a> {
        iced::widget::Text::new(text.to_string()).font(iced::Font {
            weight: iced::font::Weight::Semibold,
            ..Default::default()
        })
    }

    fn left_border<'a, Message: 'a>(win: bool) -> iced::widget::Container<'a, Message> {
        container(Space::new(6.0, 0.0))
            .style(theme::left_border_container(win))
            .height(Length::Fill)
    }

    fn small_text<'a>(text: impl ToString) -> iced::widget::Text<'a> {
        iced::widget::Text::new(text.to_string())
            .style(theme::sub_text())
            .size(8.0)
    }

    /// size 8
    fn very_small_icon<'a, Message: 'a>() -> iced::widget::Container<'a, Message> {
        container(Space::new(8.0, 8.0))
            .style(theme::icon_container())
            .max_width(8.0)
            .max_height(8.0)
    }

    /// size 10
    fn small_icon<'a, Message: 'a>() -> iced::widget::Container<'a, Message> {
        container(Space::new(10.0, 10.0))
            .style(theme::icon_container())
            .max_width(10.0)
            .max_height(10.0)
    }

    /// size 12
    fn medium_icon<'a, Message: 'a>() -> iced::widget::Container<'a, Message> {
        container(Space::new(12.0, 12.0))
            .style(theme::icon_container())
            .max_width(12.0)
            .max_height(12.0)
    }

    /// size 18
    fn medium_large_icon<'a, Message: 'a>() -> iced::widget::Container<'a, Message> {
        container(Space::new(18.0, 18.0))
            .style(theme::icon_container())
            .max_width(18.0)
            .max_height(18.0)
    }

    /// size 48
    fn large_icon<'a, Message: 'a>() -> iced::widget::Container<'a, Message> {
        container(Space::new(48.0, 48.0))
            .style(theme::icon_container())
            .max_width(48.0)
            .max_height(48.0)
    }
}

mod theme {
    use iced::theme;
    use iced::widget;
    use iced::Background;
    use iced::Color;

    pub enum Container {
        Dark,
        Icon,
        LeftBorder(bool),
        Timeline,
        PastRank,
        PastRankBadge,
        SummonerIcon,
        SummonerLevel,
        SearchBar,
        LeftBar,
    }

    pub const DARKER_BACKGROUND: Color = Color::from_rgb(0.05, 0.05, 0.05);
    pub const DARK_BACKGROUND: Color = Color::from_rgb(0.1, 0.1, 0.1);
    pub const LIGHTER_BACKGROUND: Color = Color::from_rgb(0.2, 0.2, 0.2);
    pub const LIGHT_BACKGROUND: Color = Color::from_rgb(0.95, 0.95, 0.95);

    pub const RED: Color = Color::from_rgb(1.0, 0.34, 0.2);
    pub const BLUE: Color = Color::from_rgb(0.0, 0.58, 1.0);
    pub const GOLD: Color = Color::from_rgb(205.0 / 255.0, 136.0 / 255.0, 55.0 / 255.0);

    pub fn search_bar_text_input() -> theme::TextInput {
        theme::TextInput::Custom(Box::new(TextInput::SearchBar))
    }

    pub fn left_bar_container() -> theme::Container {
        theme::Container::Custom(Box::new(Container::LeftBar))
    }

    pub fn search_bar_container() -> theme::Container {
        theme::Container::Custom(Box::new(Container::SearchBar))
    }

    pub fn timeline_container() -> theme::Container {
        theme::Container::Custom(Box::new(Container::Timeline))
    }

    pub fn past_rank_container() -> theme::Container {
        theme::Container::Custom(Box::new(Container::PastRank))
    }

    pub fn summoner_icon_container() -> theme::Container {
        theme::Container::Custom(Box::new(Container::SummonerIcon))
    }

    pub fn summoner_level_container() -> theme::Container {
        theme::Container::Custom(Box::new(Container::SummonerLevel))
    }

    pub fn dark_container() -> theme::Container {
        theme::Container::Custom(Box::new(Container::Dark))
    }

    pub fn icon_container() -> theme::Container {
        theme::Container::Custom(Box::new(Container::Icon))
    }

    pub fn left_border_container(win: bool) -> theme::Container {
        theme::Container::Custom(Box::new(Container::LeftBorder(win)))
    }

    pub fn past_rank_badge_container() -> theme::Container {
        theme::Container::Custom(Box::new(Container::PastRankBadge))
    }

    pub fn ratio_bar() -> theme::ProgressBar {
        theme::ProgressBar::Custom(Box::new(RatioBar))
    }

    pub fn region_button() -> theme::Button {
        theme::Button::Custom(Box::new(Button::Region))
    }

    pub fn expand_button() -> theme::Button {
        theme::Button::Custom(Box::new(Button::Expand))
    }

    pub fn tier_color(tier: crate::summoner::Tier) -> Color {
        match tier {
            crate::widget::summoner::Tier::Iron(_) => Color::from_rgb8(87, 77, 79),
            crate::widget::summoner::Tier::Bronze(_) => Color::from_rgb8(140, 82, 58),
            crate::widget::summoner::Tier::Silver(_) => Color::from_rgb8(128, 152, 157),
            crate::widget::summoner::Tier::Gold(_) => Color::from_rgb8(205, 136, 55),
            crate::widget::summoner::Tier::Platinum(_) => Color::from_rgb8(78, 153, 150),
            crate::widget::summoner::Tier::Diamond(_) => Color::from_rgb8(87, 107, 206),
            crate::widget::summoner::Tier::Master(_) => Color::from_rgb8(157, 72, 224),
            crate::widget::summoner::Tier::Grandmaster(_) => Color::from_rgb8(205, 69, 69),
            crate::widget::summoner::Tier::Challenger(_) => Color::from_rgb8(244, 200, 116),
        }
    }

    pub fn win_color(win: bool) -> Color {
        if win {
            BLUE
        } else {
            RED
        }
    }

    pub fn gray_text() -> Color {
        Color::from_rgb(0.5, 0.5, 0.5)
    }

    pub fn red_text() -> Color {
        RED
    }

    pub fn sub_text() -> Color {
        Color::from_rgb(0.8, 0.8, 0.8)
    }

    pub fn blue_text() -> Color {
        BLUE
    }

    pub fn update_button() -> theme::Button {
        theme::Button::Custom(Box::new(Button::Update))
    }

    pub fn scrollable() -> theme::Scrollable {
        theme::Scrollable::Custom(Box::new(Scrollable))
    }

    impl widget::container::StyleSheet for Container {
        type Style = iced::Theme;

        fn appearance(&self, _theme: &iced::Theme) -> widget::container::Appearance {
            let background = match self {
                Container::Timeline => Background::Color(DARKER_BACKGROUND),
                Container::Dark => Background::Color(DARK_BACKGROUND),
                Container::PastRankBadge => Background::Color(LIGHTER_BACKGROUND),
                Container::PastRank => Background::Color(Color::from_rgb(0.1, 0.1, 0.1)),
                Container::Icon => Background::Color(LIGHT_BACKGROUND),
                Container::LeftBorder(win) => Background::Color(win_color(*win)),
                Container::SummonerIcon => Background::Color(LIGHT_BACKGROUND), // todo: switch to image
                Container::SummonerLevel => Background::Color(DARK_BACKGROUND),
                Container::SearchBar => Background::Color(LIGHTER_BACKGROUND),
                Container::LeftBar => Background::Color(BLUE),
            };

            let text_color = match self {
                Container::Dark
                | Container::LeftBorder(_)
                | Container::Timeline
                | Container::PastRank
                | Container::PastRankBadge
                | Container::SummonerIcon
                | Container::SummonerLevel
                | Container::LeftBar => Color::WHITE,
                Container::Icon => Color::BLACK,
                Container::SearchBar => sub_text(),
            };

            let border_radius = match self {
                Container::Dark => 4.0.into(),
                Container::PastRank => 2.0.into(),
                Container::SummonerLevel => 2.0.into(),
                Container::SummonerIcon => [0.0, 2.0, 2.0, 2.0].into(),
                Container::Timeline | Container::Icon => 0.0.into(),
                Container::LeftBorder(_) => [4.0, 0.0, 0.0, 4.0].into(),
                Container::PastRankBadge => [2.0, 0.0, 0.0, 2.0].into(),
                Container::SearchBar | Container::LeftBar => 0.0.into(),
            };

            let border_color = match self {
                Container::Dark
                | Container::PastRank
                | Container::PastRankBadge
                | Container::Timeline
                | Container::LeftBorder(_)
                | Container::Icon => Color::TRANSPARENT,
                Container::SummonerIcon | Container::SummonerLevel => GOLD,
                Container::SearchBar => Color::TRANSPARENT,
                Container::LeftBar => Color::TRANSPARENT,
            };

            let border_width = match self {
                Container::Dark
                | Container::PastRank
                | Container::PastRankBadge
                | Container::Timeline
                | Container::LeftBorder(_)
                | Container::Icon
                | Container::SearchBar => 0.0,
                Container::SummonerIcon => 2.0,
                Container::SummonerLevel => 2.0,
                Container::LeftBar => 0.0,
            };

            widget::container::Appearance {
                background: Some(background),
                text_color: Some(text_color),
                border_radius,
                border_color,
                border_width,
                ..Default::default()
            }
        }
    }

    pub fn expander_button(toggled: bool) -> theme::Button {
        theme::Button::custom(Button::Expander(toggled))
    }

    pub enum Button {
        Expander(bool),
        Update,
        Region,
        Expand,
    }

    impl widget::button::StyleSheet for Button {
        type Style = iced::Theme;

        fn active(&self, _theme: &iced::Theme) -> widget::button::Appearance {
            let background_color = match self {
                Button::Expander(false) => Color::from_rgba(0.2, 0.2, 0.2, 0.6),
                Button::Expander(true) => Color::TRANSPARENT,
                Button::Update => BLUE,
                Button::Region => GOLD,
                Button::Expand => LIGHTER_BACKGROUND,
            };

            let border_radius = match self {
                Button::Expander(true) => [0.0, 4.0, 0.0, 0.0].into(),
                Button::Expander(false) => [0.0, 4.0, 4.0, 0.0].into(),
                Button::Update => 2.0.into(),
                Button::Region => 0.0.into(),
                Button::Expand => 2.0.into(),
            };

            let text_color = match self {
                Button::Region | Button::Expander(_) | Button::Update | Button::Expand => {
                    Color::WHITE
                }
            };

            widget::button::Appearance {
                background: Some(iced::Background::Color(background_color)),
                border_radius,
                text_color,
                ..Default::default()
            }
        }

        fn hovered(&self, _theme: &iced::Theme) -> widget::button::Appearance {
            let background_color = match self {
                Button::Update => Color::from_rgba(0.0, 0.58, 1.0, 0.7),
                Button::Expander(_) => Color::from_rgba(0.4, 0.4, 0.4, 0.9),
                Button::Region => Color::from_rgba(205.0 / 255.0, 136.0 / 255.0, 55.0 / 255.0, 0.7),
                Button::Expand => Color::from_rgba(0.4, 0.4, 0.4, 0.9),
            };

            widget::button::Appearance {
                background: Some(iced::Background::Color(background_color)),
                ..self.active(_theme)
            }
        }
    }

    struct RatioBar;

    impl widget::progress_bar::StyleSheet for RatioBar {
        type Style = iced::Theme;

        fn appearance(&self, _theme: &iced::Theme) -> widget::progress_bar::Appearance {
            widget::progress_bar::Appearance {
                background: Background::Color(RED),
                bar: Background::Color(BLUE),
                border_radius: 0.0.into(),
            }
        }
    }

    pub fn rule(color: Color) -> theme::Rule {
        theme::Rule::Custom(Box::new(Rule(color)))
    }

    struct Rule(Color);

    impl widget::rule::StyleSheet for Rule {
        type Style = iced::Theme;

        fn appearance(&self, _theme: &iced::Theme) -> widget::rule::Appearance {
            widget::rule::Appearance {
                color: self.0,
                width: 1,
                radius: 0.0.into(),
                fill_mode: widget::rule::FillMode::Full,
            }
        }
    }

    struct Scrollable;

    impl widget::scrollable::StyleSheet for Scrollable {
        type Style = iced::Theme;

        fn active(&self, _theme: &iced::Theme) -> widget::scrollable::Scrollbar {
            widget::scrollable::Scrollbar {
                background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.1))),
                scroller: widget::scrollable::Scroller {
                    color: Color::from_rgb(0.2, 0.2, 0.2),
                    border_radius: 2.0.into(),
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                },
                border_radius: 2.0.into(),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            }
        }

        fn hovered(
            &self,
            style: &Self::Style,
            is_mouse_over_scrollbar: bool,
        ) -> widget::scrollable::Scrollbar {
            if is_mouse_over_scrollbar {
                widget::scrollable::Scrollbar {
                    background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.1))),
                    border_radius: 2.0.into(),
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                    scroller: widget::scrollable::Scroller {
                        color: Color::from_rgba(0.4, 0.4, 0.4, 0.9),
                        border_radius: 2.0.into(),
                        border_width: 0.0,
                        border_color: Color::TRANSPARENT,
                    },
                }
            } else {
                self.active(style)
            }
        }
    }

    pub enum TextInput {
        SearchBar,
    }

    impl widget::text_input::StyleSheet for TextInput {
        type Style = iced::Theme;

        fn active(&self, _style: &Self::Style) -> widget::text_input::Appearance {
            widget::text_input::Appearance {
                background: Background::Color(Color::TRANSPARENT),
                border_radius: 0.0.into(),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                icon_color: Color::TRANSPARENT,
            }
        }

        fn hovered(&self, style: &Self::Style) -> widget::text_input::Appearance {
            widget::text_input::Appearance {
                background: Background::Color(Color::from_rgba(0.2, 0.2, 0.2, 0.7)),
                ..self.active(style)
            }
        }

        fn focused(&self, style: &Self::Style) -> widget::text_input::Appearance {
            self.hovered(style)
        }

        fn placeholder_color(&self, _style: &Self::Style) -> Color {
            sub_text()
        }

        fn value_color(&self, _style: &Self::Style) -> Color {
            Color::WHITE
        }

        fn selection_color(&self, _style: &Self::Style) -> Color {
            BLUE
        }

        fn disabled(&self, style: &Self::Style) -> widget::text_input::Appearance {
            widget::text_input::Appearance {
                background: Background::Color(DARKER_BACKGROUND),
                ..self.active(style)
            }
        }

        fn disabled_color(&self, _style: &Self::Style) -> Color {
            sub_text()
        }
    }
}
