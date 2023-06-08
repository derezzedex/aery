use iced::{Application, Command, Element, Settings};
use widget::timeline::{self, Timeline};

pub fn main() -> iced::Result {
    Aery::run(Settings {
        antialiasing: true,
        ..Default::default()
    })
}

struct Aery {
    timeline: Timeline,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Timeline(timeline::Message),
}

impl Application for Aery {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                timeline: Timeline::new(),
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
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        self.timeline.view().map(Message::Timeline)
    }
}

mod widget {
    use crate::theme;
    use iced::widget::{container, Space};
    use iced::Length;

    #[derive(Debug, Clone)]
    enum Queue {
        RankedFlex,
    }

    impl ToString for Queue {
        fn to_string(&self) -> String {
            match self {
                Queue::RankedFlex => "Ranked Flex",
            }
            .to_string()
        }
    }

    #[derive(Debug, Clone)]
    enum Role {
        Mid,
    }

    impl ToString for Role {
        fn to_string(&self) -> String {
            match self {
                Role::Mid => "Mid",
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
                format!("{} seconds ago", seconds)
            } else if minutes < 60 {
                format!(
                    "{} minute{} ago",
                    minutes,
                    if minutes == 1 { "" } else { "s" }
                )
            } else if hours < 24 {
                format!("{} hour{} ago", hours, if minutes == 1 { "" } else { "s" })
            } else if days < 7 {
                format!("{} day{} ago", days, if minutes == 1 { "" } else { "s" })
            } else if weeks < 4 {
                format!("{} week{} ago", weeks, if minutes == 1 { "" } else { "s" })
            } else if months < 12 {
                format!(
                    "{} month{} ago",
                    months,
                    if minutes == 1 { "" } else { "s" }
                )
            } else {
                format!("{} years ago", years)
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
    struct Champion(u16);

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

    pub mod timeline {
        use self::summary::Summary;

        use super::game::{self, Game};
        use super::theme;
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
            pub fn new() -> Self {
                Timeline {
                    summary: Summary::new(),
                    games: (0..5)
                        .into_iter()
                        .map(|_| [Game::new(true), Game::new(false)])
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
                        .width(Length::Fill)
                        .height(Length::FillPortion(9))
                ]
                .max_width(640)
                .align_items(Alignment::Center)
                .spacing(4);

                let el: Element<_> = container(timeline)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .style(theme::timeline_container())
                    .into();

                el
            }
        }

        pub mod summary {
            use super::theme;
            use super::Message;
            use crate::text;
            use crate::widget::medium_large_icon;
            use crate::widget::very_small_icon;
            use iced::alignment;
            use iced::widget::Space;
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
            pub struct Summary;

            impl Summary {
                pub fn new() -> Summary {
                    Summary
                }

                pub fn view(&self) -> Element<Message> {
                    let wins = 6;
                    let losses = 4;
                    let ratio = (wins as f32 / (wins + losses) as f32) * 100.0;
                    let is_positive_ratio = ratio > 50.0;
                    let kill_ratio = 2.7;
                    let death_ratio = 6.7;
                    let assist_ratio = 7.0;

                    let title_bar = row![
                        text("RECENT SUMMARY").size(12),
                        text!("last {total} games", total = wins + losses)
                            .style(theme::gray_text())
                            .size(10)
                    ]
                    .padding(6)
                    .align_items(Alignment::Center)
                    .spacing(4);

                    let summary_ratio = {
                        let ratio_text = row![
                            row![
                                row![
                                    text!("{wins}").fit(12),
                                    text("W").fit(12).style(theme::gray_text())
                                ]
                                .spacing(1),
                                row![
                                    text!("{losses}").fit(12),
                                    text("L").fit(12).style(theme::gray_text())
                                ]
                            ]
                            .spacing(4),
                            text("·").fit(18).style(theme::sub_text()),
                            text!("{ratio:.1}%")
                                .fit(12)
                                .style(theme::win_color(is_positive_ratio)),
                        ]
                        .align_items(Alignment::Center)
                        .spacing(4);

                        let ratio_bar = progress_bar(0.0..=100.0, ratio)
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
                        let lane_icon = medium_large_icon();

                        let lane_info = column![
                            row![
                                row![
                                    row![
                                        text!("{wins}").fit(12),
                                        text("W").fit(12).style(theme::gray_text())
                                    ]
                                    .spacing(1),
                                    row![
                                        text!("{losses}").fit(12),
                                        text("L").fit(12).style(theme::gray_text())
                                    ]
                                ]
                                .spacing(4),
                                text("·").fit(18).style(theme::sub_text()),
                                text!("{ratio:.1}%")
                                    .fit(12)
                                    .style(theme::win_color(is_positive_ratio)),
                            ]
                            .align_items(Alignment::Center)
                            .spacing(4),
                            row![
                                text!("{kill_ratio:.1}").size(10),
                                text("/").size(10).style(theme::gray_text()),
                                text!("{death_ratio:.1}").size(10),
                                text("/").size(10).style(theme::gray_text()),
                                text!("{assist_ratio:.1}").size(10),
                                row![
                                    text("(").size(10).style(theme::red_text()),
                                    text!("{:.1} KDA", death_ratio + assist_ratio / kill_ratio)
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
                        // TODO: change to champion id
                        let champions = vec![
                            ("Twisted Fate", 2, 1, 1.15),
                            ("Orianna", 3, 0, 2.0),
                            ("Annie", 2, 2, 3.0),
                            ("Sion", 0, 3, 0.5),
                        ];

                        let content: Vec<Element<Message>> = champions
                            .into_iter()
                            .map(|(_name, wins, losses, kda)| {
                                let icon = container(Space::new(24.0, 24.0))
                                    .style(theme::icon_container())
                                    .max_width(24.0)
                                    .max_height(24.0);
                                let winrate = wins as f32 * 100.0 / (wins + losses) as f32;

                                row![
                                    icon,
                                    // TODO: fix strange alignment between bottom and top text
                                    column![
                                        row![
                                            text!("{:.1}%", winrate)
                                                .size(10)
                                                .style(theme::win_color(winrate > 50.0)),
                                            text!("({wins}W {losses}L)")
                                                .size(10)
                                                .style(theme::gray_text())
                                        ]
                                        .align_items(Alignment::Center)
                                        .spacing(2),
                                        row![
                                            very_small_icon(),
                                            text!("{:.2} KDA", kda)
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

                    let content = column![title_bar, horizontal_rule(2), body];

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
        use crate::theme;
        use iced::widget::{button, column, container, row, text, Space};
        use iced::{alignment, Alignment, Element, Length};

        #[derive(Debug, Clone)]
        pub struct Game {
            win: bool,
            queue: Queue,
            time: Time,
            duration: Duration,
            role: Option<Role>,
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
            pub fn new(win: bool) -> Self {
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
                                very_small_icon(),
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
                            text(formatting::win(self.win))
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
                    let champion_icon = large_icon();

                    let champion_spells = row![medium_icon(), medium_icon(),].spacing(2);

                    let champion_runes = row![medium_icon(), medium_icon(),].spacing(2);

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
                        row![
                            very_small_icon(),
                            text(formatting::kda(
                                self.player_kills,
                                self.player_deaths,
                                self.player_assists
                            ))
                            .size(10)
                            .style(theme::sub_text())
                        ]
                        .spacing(4)
                        .align_items(Alignment::Center),
                        row![
                            very_small_icon(),
                            text(formatting::creep_score(
                                self.player_creep_score,
                                self.duration.0.whole_minutes() as u16
                            ))
                            .size(10)
                            .style(theme::sub_text())
                        ]
                        .spacing(4)
                        .align_items(Alignment::Center),
                        row![
                            very_small_icon(),
                            text(formatting::vision_score(self.player_vision_score))
                                .size(10)
                                .style(theme::sub_text())
                        ]
                        .spacing(4)
                        .align_items(Alignment::Center),
                    ]
                    .align_items(Alignment::Start);

                    column![kda, other_stats,].align_items(Alignment::Center)
                };

                let player_items = {
                    row![
                        column![medium_large_icon(), medium_large_icon()].spacing(2),
                        column![medium_large_icon(), medium_large_icon()].spacing(2),
                        column![medium_large_icon(), medium_large_icon()].spacing(2),
                        medium_large_icon(),
                    ]
                    .spacing(2)
                };

                let mut left_players: Vec<Element<_>> = self
                    .summoners
                    .iter()
                    .map(|summoner| {
                        let summoner_icon = small_icon();
                        let summoner_name = small_text(summoner.to_string());

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

                let expand_content = container(small_icon())
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

    fn very_small_icon<'a, Message: 'a>() -> iced::widget::Container<'a, Message> {
        container(Space::new(8.0, 8.0))
            .style(theme::icon_container())
            .max_width(8.0)
            .max_height(8.0)
    }

    fn small_icon<'a, Message: 'a>() -> iced::widget::Container<'a, Message> {
        container(Space::new(10.0, 10.0))
            .style(theme::icon_container())
            .max_width(10.0)
            .max_height(10.0)
    }

    fn medium_icon<'a, Message: 'a>() -> iced::widget::Container<'a, Message> {
        container(Space::new(12.0, 12.0))
            .style(theme::icon_container())
            .max_width(12.0)
            .max_height(12.0)
    }

    fn medium_large_icon<'a, Message: 'a>() -> iced::widget::Container<'a, Message> {
        container(Space::new(18.0, 18.0))
            .style(theme::icon_container())
            .max_width(18.0)
            .max_height(18.0)
    }

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
    }

    pub const DARKER_BACKGROUND: Color = Color::from_rgb(0.05, 0.05, 0.05);
    pub const DARK_BACKGROUND: Color = Color::from_rgb(0.1, 0.1, 0.1);
    pub const LIGHT_BACKGROUND: Color = Color::from_rgb(0.95, 0.95, 0.95);

    pub const RED: Color = Color::from_rgb(1.0, 0.34, 0.2);
    pub const BLUE: Color = Color::from_rgb(0.0, 0.58, 1.0);

    pub fn timeline_container() -> theme::Container {
        theme::Container::Custom(Box::new(Container::Timeline))
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

    pub fn ratio_bar() -> theme::ProgressBar {
        theme::ProgressBar::Custom(Box::new(RatioBar))
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

    impl widget::container::StyleSheet for Container {
        type Style = iced::Theme;

        fn appearance(&self, _theme: &iced::Theme) -> widget::container::Appearance {
            let background_color = match self {
                Container::Timeline => DARKER_BACKGROUND,
                Container::Dark => DARK_BACKGROUND,
                Container::Icon => LIGHT_BACKGROUND,
                Container::LeftBorder(win) => win_color(*win),
            };

            let text_color = match self {
                Container::Dark | Container::LeftBorder(_) | Container::Timeline => Color::WHITE,
                Container::Icon => Color::BLACK,
            };

            let border_radius = match self {
                Container::Dark => 4.0.into(),
                Container::Timeline | Container::Icon => 0.0.into(),
                Container::LeftBorder(_) => [4.0, 0.0, 0.0, 4.0].into(),
            };

            widget::container::Appearance {
                background: Some(Background::Color(background_color)),
                text_color: Some(text_color),
                border_radius,
                ..Default::default()
            }
        }
    }

    pub fn expander_button(toggled: bool) -> theme::Button {
        theme::Button::custom(Button::Expander(toggled))
    }

    pub enum Button {
        Expander(bool),
    }

    impl widget::button::StyleSheet for Button {
        type Style = iced::Theme;

        fn active(&self, _theme: &iced::Theme) -> widget::button::Appearance {
            let background_color = match self {
                Button::Expander(false) => Color::from_rgba(0.2, 0.2, 0.2, 0.6),
                Button::Expander(true) => Color::TRANSPARENT,
            };

            let border_radius = match self {
                Button::Expander(true) => [0.0, 4.0, 0.0, 0.0].into(),
                Button::Expander(false) => [0.0, 4.0, 4.0, 0.0].into(),
            };

            widget::button::Appearance {
                background: Some(iced::Background::Color(background_color)),
                border_radius,
                ..Default::default()
            }
        }

        fn hovered(&self, _theme: &iced::Theme) -> widget::button::Appearance {
            let background_color = Color::from_rgba(0.4, 0.4, 0.4, 0.9);

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
                border_radius: 2.0.into(),
            }
        }
    }
}
