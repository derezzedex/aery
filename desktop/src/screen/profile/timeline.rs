use super::game::{self, Game};
use crate::profile;
use crate::theme;

use iced::padding;
use iced::widget::{column, container, scrollable, text};
use iced::{Alignment, Element, Length};

use summary::Summary;

#[derive(Debug, Clone)]
pub enum Message {
    Game(usize, game::Message),
}

#[derive(Debug, Clone)]
pub struct Timeline {
    summary: Summary,
    games: Vec<Game>,
}

impl Timeline {
    pub fn from_profile(
        assets: &crate::assets::Assets,
        profile: &profile::Data,
        queue_filter: &profile::QueueFilter,
    ) -> Self {
        let games = profile
            .games
            .iter()
            .filter(|game| *queue_filter == game.queue())
            .collect::<Vec<_>>();

        let summary = Summary::from_games(assets, &profile.summoner, &games);

        let games = games
            .iter()
            .map(|game| Game::from_summoner_game(assets, &profile.summoner, game))
            .collect();

        Timeline { summary, games }
    }

    pub fn update(&mut self, message: Message) -> Option<game::Event> {
        match message {
            Message::Game(index, message) => unsafe {
                self.games.get_unchecked_mut(index).update(message)
            },
        }
    }

    pub fn view(&self) -> Element<Message> {
        let games = self
            .games
            .iter()
            .enumerate()
            .map(|(i, game)| game.view().map(move |message| Message::Game(i, message)))
            .collect::<Vec<_>>();

        if games.is_empty() {
            return container(text("No games found...").size(20))
                .padding(8)
                .height(Length::Fill)
                .center_x(682)
                .into();
        }

        let content = column(games)
            .width(Length::Fill)
            .clip(true)
            .padding(padding::right(12))
            .spacing(4)
            .align_x(Alignment::Center);

        let summary = self.summary.view();
        let timeline = column![
            container(summary),
            scrollable(content)
                .style(theme::scrollable)
                .width(Length::Fill)
                .height(Length::FillPortion(10))
        ]
        .max_width(680)
        .align_x(Alignment::Center)
        .spacing(4);

        container(timeline)
            .width(Length::Shrink)
            .style(theme::timeline)
            .into()
    }
}

pub mod summary {
    use super::theme;
    use super::Message;
    use crate::core;
    use crate::core::game::Role;
    use crate::theme::icon;

    use iced::alignment;
    use iced::padding;
    use iced::widget::image::Handle;
    use iced::widget::vertical_space;
    use iced::widget::{
        column, container, horizontal_rule, horizontal_space, progress_bar, row, text,
    };
    use iced::{Alignment, Element};
    use itertools::Itertools;

    trait Fit {
        fn fit(self, size: u32) -> Self;
    }

    impl<'a> Fit for iced::widget::Text<'a> {
        fn fit(self, size: u32) -> iced::widget::Text<'a> {
            self.size(size)
                .line_height(1.1)
                .align_y(alignment::Vertical::Center)
        }
    }

    fn kda(kills: usize, deaths: usize, assists: usize, total: usize) -> String {
        if deaths == 0 {
            return String::from("Perfect KDA");
        }

        format!(
            "{:.1} KDA",
            ((kills as f32 + assists as f32) / deaths as f32) / total as f32,
        )
    }

    #[derive(Debug, Clone)]
    pub struct Champion {
        pub handle: Handle,
        pub lane: Role,
        pub wins: usize,
        pub losses: usize,
        pub kills: usize,
        pub deaths: usize,
        pub assists: usize,
    }

    #[derive(Debug, Clone, Default)]
    pub struct RoleStats {
        wins: usize,
        losses: usize,
        kills: usize,
        deaths: usize,
        assists: usize,
    }

    #[derive(Debug, Clone)]
    pub struct Summary {
        total: usize,
        wins: usize,
        losses: usize,

        role: Role,
        role_stats: RoleStats,

        champions: Vec<Champion>,
    }

    impl Summary {
        pub fn from_games(
            assets: &crate::Assets,
            player: &core::Summoner,
            games: &[&core::Game],
        ) -> Summary {
            let games = games
                .iter()
                .map(|game| game.participant(player.puuid()).unwrap())
                .collect_vec();

            let total = games.iter().count();
            let wins = games.iter().filter(|game| game.result.won()).count();
            let losses = games.iter().filter(|game| game.result.lost()).count();

            let (role, role_stats) = games
                .iter()
                .filter(|p| p.role.is_some())
                .into_grouping_map_by(|p| p.role.unwrap())
                .fold(RoleStats::default(), |acc, _role, p| RoleStats {
                    wins: acc.wins + p.result.won() as usize,
                    losses: acc.losses + p.result.lost() as usize,
                    kills: acc.kills + p.stats.kills as usize,
                    deaths: acc.deaths + p.stats.deaths as usize,
                    assists: acc.assists + p.stats.assists as usize,
                })
                .into_iter()
                .max_by(|(_, a), (_, b)| a.wins.cmp(&b.wins))
                .unwrap_or((Role::Mid, RoleStats::default()));

            let champions = games
                .iter()
                .filter(|p| p.role.is_some())
                .into_grouping_map_by(|&p| (p.role.unwrap(), p.champion))
                .fold(RoleStats::default(), |acc, _, p| RoleStats {
                    wins: acc.wins + p.result.won() as usize,
                    losses: acc.losses + p.result.lost() as usize,
                    kills: acc.kills + p.stats.kills as usize,
                    deaths: acc.deaths + p.stats.deaths as usize,
                    assists: acc.assists + p.stats.assists as usize,
                })
                .into_iter()
                .sorted_unstable_by(|(_, a), (_, b)| b.wins.cmp(&a.wins))
                .take(4)
                .map(|((role, champion), stats)| Champion {
                    handle: assets.champion(&champion),
                    lane: role,
                    wins: stats.wins,
                    losses: stats.losses,
                    kills: stats.kills,
                    deaths: stats.deaths,
                    assists: stats.assists,
                })
                .collect_vec();

            Self {
                total,
                wins,
                losses,

                role,
                role_stats,

                champions,
            }
        }

        pub fn view(&self) -> Element<Message> {
            let played = self.wins + self.losses;
            let ratio = (self.wins as f32 / played as f32) * 100.0;
            let is_positive_ratio = self.wins > self.losses;

            let title_bar = row![
                container(text("Recent summary").font(theme::SEMIBOLD).size(11))
                    .padding(padding::top(3)),
                text!("last {} games", self.total)
                    .style(theme::text)
                    .size(11)
            ]
            .padding(padding::top(3).right(6).left(6))
            .align_y(Alignment::Center)
            .spacing(8);

            let summary_ratio = {
                let ratio_text = row![
                    row![
                        row![
                            text!("{}", self.wins).fit(12),
                            text("W").fit(12).style(theme::text)
                        ]
                        .spacing(1),
                        row![
                            text!("{}", self.losses).fit(12),
                            text("L").fit(12).style(theme::text)
                        ]
                    ]
                    .spacing(4),
                    text("·").fit(18),
                    text!("{:.1}%", ratio)
                        .fit(12)
                        .style(move |theme| text::Style {
                            color: Some(theme::win_color(theme, is_positive_ratio))
                        }),
                ]
                .align_y(Alignment::Center)
                .spacing(4);

                let ratio_bar = container(progress_bar(0.0..=100.0, ratio).style(theme::ratio_bar))
                    .width(80.0)
                    .height(4.0);

                column![
                    text("Winrate").fit(11).font(theme::SEMIBOLD),
                    vertical_space().height(2),
                    ratio_text,
                    ratio_bar,
                ]
            };

            let summary_lane = {
                let lane_icon = icon::role(self.role)
                    .width(24.0)
                    .height(24.0)
                    .content_fit(iced::ContentFit::Fill);

                let total = self.role_stats.wins + self.role_stats.losses;
                let lane_ratio = (self.role_stats.wins as f32 / total as f32) * 100.0;
                let kill_ratio = self.role_stats.kills as f32 / total as f32;
                let death_ratio = self.role_stats.deaths as f32 / total as f32;
                let assist_ratio = self.role_stats.assists as f32 / total as f32;

                let lane_info = column![
                    row![
                        row![
                            row![
                                text!("{}", self.role_stats.wins).fit(12),
                                text("W").fit(12).style(theme::text)
                            ]
                            .spacing(1),
                            row![
                                text!("{}", self.role_stats.losses).fit(12),
                                text("L").fit(12).style(theme::text)
                            ]
                        ]
                        .spacing(4),
                        text("·").fit(18).style(theme::text),
                        text!("{:.1}%", lane_ratio)
                            .fit(12)
                            .style(move |theme| text::Style {
                                color: Some(theme::win_color(theme, lane_ratio > 50.0))
                            }),
                    ]
                    .align_y(Alignment::Center)
                    .spacing(4),
                    row![
                        text!("{:.1}", kill_ratio).size(10),
                        text("/").size(10).style(theme::text),
                        text!("{:.1}", death_ratio).size(10),
                        text("/").size(10).style(theme::text),
                        text!("{:.1}", assist_ratio).size(10),
                        horizontal_space().width(2),
                        row![
                            text("(").size(10).style(theme::defeat),
                            text(kda(
                                self.role_stats.kills,
                                self.role_stats.deaths,
                                self.role_stats.assists,
                                total
                            ))
                            .size(10)
                            .style(theme::defeat),
                            text(")").size(10).style(theme::defeat)
                        ],
                    ]
                    .spacing(2)
                    .align_y(Alignment::Start),
                ];

                column![
                    text("Lane").size(11).font(theme::SEMIBOLD),
                    row![lane_icon, lane_info]
                        .align_y(Alignment::Center)
                        .spacing(4)
                ]
            };

            let summary_champions = {
                let content = self.champions.iter().map(|champion| {
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
                                    .style(move |theme| text::Style {
                                        color: Some(theme::win_color(theme, winrate > 50.0))
                                    }),
                                text!("({}W {}L)", champion.wins, champion.losses)
                                    .size(10)
                                    .style(theme::text)
                            ]
                            .align_y(Alignment::Center)
                            .spacing(2),
                            row![
                                icon::role(champion.lane).width(12.0).height(12.0),
                                container(
                                    text(kda(champion.kills, champion.deaths, champion.assists, 1))
                                        .size(10)
                                        .style(theme::text)
                                )
                                .padding(padding::top(2).left(2))
                            ]
                            .spacing(2)
                            .align_y(Alignment::Center),
                        ]
                    ]
                    .align_y(Alignment::Center)
                    .spacing(4)
                    .into()
                });

                column![
                    text("Champions").size(11).font(theme::SEMIBOLD),
                    row(content).spacing(4).align_y(Alignment::Center)
                ]
                .spacing(4)
            };

            let body = container(
                row![summary_ratio, summary_lane, summary_champions]
                    .spacing(16)
                    .align_y(Alignment::Start),
            )
            .padding(8);

            let content = column![
                title_bar,
                container(horizontal_rule(2).style(theme::rule))
                    .width(iced::Length::Fill)
                    .padding([0, 4]),
                body
            ];

            container(content)
                .width(iced::Length::Fill)
                .style(theme::dark)
                .into()
        }
    }
}
