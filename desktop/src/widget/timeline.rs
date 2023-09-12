use crate::assets::load_champion_icon;

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
    pub fn new(assets: &crate::assets::Assets) -> Self {
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
                        let winrate =
                            champion.wins as f32 * 100.0 / (champion.wins + champion.losses) as f32;

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
