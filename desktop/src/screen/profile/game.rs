use crate::assets::load_champion_icon;
use crate::assets::load_item_icon;
use crate::assets::load_runes_icon;
use crate::assets::load_summoner_spell_icon;
use crate::component::*;
use crate::core;
use crate::core::game;
use crate::core::summoner;
use crate::core::{Duration, Time};
use crate::theme;
use crate::theme::chevron_down_icon;
use crate::theme::chevron_up_icon;
use crate::widget;
use iced::widget::horizontal_space;
use iced::widget::image;
use iced::widget::progress_bar;
use iced::widget::svg;
use iced::widget::vertical_space;
use iced::widget::{button, column, container, row, text, Space};
use iced::{alignment, Alignment, Element, Length};
use itertools::Itertools;

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

fn item_icon<'a>(handle: Option<image::Handle>) -> Element<'a, Message> {
    let icon: Element<_> = if let Some(handle) = handle {
        iced::widget::image(handle)
            .width(28.0)
            .height(28.0)
            .content_fit(iced::ContentFit::Fill)
            .into()
    } else {
        container(iced::widget::Space::new(28.0, 28.0))
            .style(theme::search_bar_container())
            .into()
    };

    container(icon).width(28.0).height(28.0).into()
}

#[derive(Debug, Clone)]
pub struct PlayerAssets {
    champion_image: image::Handle,
    summoner_spell_images: [image::Handle; 2],
    runes_images: [image::Handle; 2],
    item_images: [Option<image::Handle>; 6],
    trinket_image: Option<image::Handle>,
}

impl PlayerAssets {
    fn from_participant(assets: &crate::Assets, participant: &game::Player) -> Self {
        let champion_image = load_champion_icon(assets, participant.champion);

        let summoner_spell_images = [
            load_summoner_spell_icon(assets, participant.summoner_spells.first()),
            load_summoner_spell_icon(assets, participant.summoner_spells.second()),
        ];
        let runes_images = [
            load_runes_icon(assets, participant.rune_page.primary.keystone()),
            load_runes_icon(assets, participant.rune_page.secondary.keystone()),
        ];

        let item_images = participant
            .inventory
            .into_iter()
            .map(|item| item.map(|item| load_item_icon(assets, item)))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let trinket_image = match participant.trinket {
            core::Trinket(0) => None,
            trinket => Some(load_item_icon(assets, trinket.into())),
        };

        Self {
            champion_image,
            summoner_spell_images,
            runes_images,
            item_images,
            trinket_image,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    assets: PlayerAssets,
    info: game::Player,
}

impl Player {
    fn dummy_with_puuid(assets: &crate::Assets, champion: core::Champion, puuid: String) -> Self {
        let dummy = Self::dummy(assets, champion);
        Self {
            info: game::Player {
                puuid,
                ..dummy.info
            },
            ..dummy
        }
    }

    fn dummy(assets: &crate::Assets, champion: core::Champion) -> Self {
        let info = game::Player {
            puuid: String::from("dummy"),
            name: String::from("dummy"),
            riot_id: summoner::RiotId {
                name: None,
                tagline: String::from("dummy"),
            },
            team: core::Team::BLUE,
            result: game::Result::Victory,
            role: core::Role::Mid,
            inventory: core::Inventory([
                Some(core::Item::new(1001)),
                Some(core::Item::new(6630)),
                Some(core::Item::new(4401)),
                Some(core::Item::new(3143)),
                Some(core::Item::new(3742)),
                Some(core::Item::new(6333)),
            ]),
            trinket: core::Trinket(3364),
            champion,
            summoner_spells: core::SummonerSpells::from([
                core::SummonerSpell::new(14),
                core::SummonerSpell::new(4),
            ]),
            rune_page: game::rune::Page {
                primary: game::rune::Primary {
                    keystone: game::rune::Rune(8010),
                },
                secondary: game::rune::Secondary {
                    lesser: [game::rune::Rune::new(8400); 2],
                },
            },
            stats: game::player::Stats {
                level: 5,
                kills: 1,
                deaths: 6,
                assists: 12,
                creep_score: 151,
                monster_score: 10,
                vision_score: 18,
                damage_dealt: 12456,
                damage_taken: 20520,
                gold: 13521,
                control_wards: 5,
                wards_placed: 10,
                wards_removed: 3,
            },
        };

        let assets = PlayerAssets::from_participant(assets, &info);

        Self { assets, info }
    }

    pub fn from_participant(assets: &crate::Assets, participant: &game::Player) -> Self {
        let assets = PlayerAssets::from_participant(assets, participant);

        Self {
            assets,
            info: participant.clone(),
        }
    }
}

#[derive(Debug, Clone)]
struct Team {
    id: core::Team,
    result: game::Result,
    players: Vec<Player>,
}

#[derive(Debug, Clone)]
pub struct Game {
    result: game::Result,
    queue: game::Queue,
    time: Time,
    duration: Duration,
    player: Player,
    teams: Vec<Team>,

    is_expanded: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    ExpandPressed,
}

impl Game {
    pub fn from_summoner_game(
        assets: &crate::assets::Assets,
        summoner: &core::Summoner,
        game: &core::Game,
    ) -> Self {
        let participants = game.participants();
        let player = participants
            .iter()
            .find(|p| p.puuid == summoner.puuid())
            .cloned()
            .unwrap();

        let result = player.result;

        let player = Player::from_participant(assets, &player);

        let teams = participants
            .iter()
            .into_grouping_map_by(|p| p.team)
            .fold(Vec::new(), |mut players, _team, participant| {
                players.push(Player::from_participant(assets, participant));
                players
            })
            .into_iter()
            .map(|(id, players)| Team {
                id,
                result: players
                    .first()
                    .map(|p| p.info.result)
                    .unwrap_or(game::Result::Defeat), // TODO: fix this
                players,
            })
            .collect();

        Game {
            result,
            queue: game.queue(),
            time: game.created_at(),
            duration: game.duration(),
            player,
            teams,

            is_expanded: false,
        }
    }

    pub fn new(win: bool, assets: &crate::assets::Assets, champion: core::Champion) -> Self {
        let puuid = String::from("player");
        let player = Player::dummy_with_puuid(assets, champion, puuid);
        let dummy = |id| Player::dummy(assets, core::Champion::new(id));

        let teams = vec![
            Team {
                id: core::Team::BLUE,
                result: win.into(),
                players: vec![player.clone(), dummy(1), dummy(101), dummy(14), dummy(122)],
            },
            Team {
                id: core::Team::RED,
                result: (!win).into(),
                players: vec![dummy(897), dummy(62), dummy(4), dummy(61), dummy(202)],
            },
        ];

        Game {
            result: if win {
                game::Result::Victory
            } else {
                game::Result::Defeat
            },
            queue: game::Queue::RankedFlex,
            time: Time(time::OffsetDateTime::now_utc().saturating_sub(time::Duration::days(1))),
            duration: Duration(
                time::Duration::minutes(28).saturating_add(time::Duration::seconds(33)),
            ),
            player,
            teams,

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

            let role: Element<_> = if let Some(role) = Role::try_from(self.player.info.role).ok() {
                column![
                    row![
                        image(role.icon()).width(12.0).height(12.0),
                        text(role.to_string()).style(theme::sub_text()).size(10),
                    ]
                    .align_items(Alignment::Center)
                    .spacing(4),
                    row![
                        svg(theme::clock_icon()).width(12.0).height(12.0),
                        container(
                            text(self.duration.to_string())
                                .size(10)
                                .style(theme::sub_text())
                        ),
                    ]
                    .align_items(Alignment::Center)
                    .spacing(4),
                ]
                .spacing(2)
                .into()
            } else {
                Space::new(0, 0).into()
            };

            column![
                widget::bold(formatting::win(self.result))
                    .style(theme::win_color(self.result))
                    .size(18),
                column![
                    text(self.queue.to_string()).size(11),
                    container(
                        text(self.time.to_string())
                            .style(theme::sub_text())
                            .size(10)
                    ),
                ],
                vertical_space().height(Length::Fill),
                role,
            ]
            .align_items(Alignment::Start)
            .spacing(2)
            .padding(2)
        };

        let champion_info = {
            let champion_icon = champion_icon(self.player.assets.champion_image.clone());
            let champion_level = container(widget::bold(self.player.info.stats.level).size(10))
                .padding([2, 4, 0, 4])
                .style(theme::summoner_level_container());

            // TODO: fix `champion_level` overlay not being clipped on the `scrollable`
            let champion = Modal::new(champion_icon, champion_level)
                .horizontal_alignment(Alignment::Start)
                .vertical_alignment(Alignment::End);

            let champion_spells = row![
                summoner_spell_icon(self.player.assets.summoner_spell_images[0].clone()),
                summoner_spell_icon(self.player.assets.summoner_spell_images[1].clone())
            ]
            .spacing(2);

            let champion_runes = row![
                summoner_rune_icon(self.player.assets.runes_images[0].clone()),
                summoner_rune2_icon(self.player.assets.runes_images[1].clone())
            ]
            .spacing(2);

            row![
                champion,
                column![champion_spells, champion_runes].spacing(2),
            ]
            .spacing(2)
        };

        let player_stats = {
            let kda = row![
                text(self.player.info.stats.kills).size(15),
                text("/").style(theme::gray_text()).size(15),
                text(self.player.info.stats.deaths)
                    .style(theme::red_text())
                    .size(15),
                text("/").style(theme::gray_text()).size(15),
                text(self.player.info.stats.assists).size(15)
            ]
            .align_items(Alignment::Center)
            .spacing(2);

            let other_stats = column![
                row![text(formatting::kda(
                    self.player.info.stats.kills,
                    self.player.info.stats.deaths,
                    self.player.info.stats.assists
                ))
                .size(10)
                .style(theme::sub_text())]
                .spacing(4)
                .align_items(Alignment::Center),
                row![text(formatting::creep_score(
                    self.player.info.stats.creep_score,
                    self.duration.0.whole_minutes() as u32
                ))
                .size(10)
                .style(theme::sub_text())]
                .spacing(4)
                .align_items(Alignment::Center),
                row![text(formatting::vision_score(
                    self.player.info.stats.vision_score
                ))
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
                    item_icon(self.player.assets.item_images[0].clone()),
                    item_icon(self.player.assets.item_images[1].clone())
                ]
                .spacing(2),
                column![
                    item_icon(self.player.assets.item_images[2].clone()),
                    item_icon(self.player.assets.item_images[3].clone())
                ]
                .spacing(2),
                column![
                    item_icon(self.player.assets.item_images[4].clone()),
                    item_icon(self.player.assets.item_images[5].clone())
                ]
                .spacing(2),
                item_icon(self.player.assets.trinket_image.clone()),
            ]
            .spacing(2)
        };

        let player_name_view = |player: &Player| {
            let name = truncate(player.info.name.to_string(), 10);
            let summoner_icon = image(player.assets.champion_image.clone())
                .width(16.0)
                .height(16.0);
            let summoner_name = if self.player.info.puuid == player.info.puuid {
                text(name).size(8.0).line_height(iced::Pixels(12.0))
            } else {
                widget::small_text(name)
                    .size(8.0)
                    .line_height(iced::Pixels(12.0))
            };

            row![summoner_icon, summoner_name]
                .align_items(Alignment::Center)
                .spacing(4)
                .into()
        };

        let blue_team = self
            .teams
            .first()
            .map(|team| team.players.iter().map(player_name_view))
            .unwrap();
        let red_team = self
            .teams
            .last()
            .map(|team| team.players.iter().map(player_name_view))
            .unwrap();

        let teams = row![column(blue_team).spacing(2), column(red_team).spacing(2),].spacing(8);

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
            .height(Length::Shrink)
            .on_press(Message::ExpandPressed)
            .style(theme::expander_button(self.is_expanded));

        let overview = container(row![
            row![
                match_stats,
                horizontal_space().width(Length::Fill),
                champion_info,
                horizontal_space().width(Length::Fill),
                player_stats,
                horizontal_space().width(Length::Fill),
                player_items,
                horizontal_space().width(Length::Fill),
                teams,
                horizontal_space().width(Length::Fill),
            ]
            .width(Length::Fill)
            .padding(4)
            .align_items(Alignment::Center),
            expand_button.padding(0),
        ])
        .max_height(100.0);

        let game = if self.is_expanded {
            let max_damage_dealt = self
                .teams
                .iter()
                .flat_map(|t| t.players.iter().map(|p| p.info.stats.damage_dealt))
                .max()
                .unwrap();
            let max_damage_taken = self
                .teams
                .iter()
                .flat_map(|t| t.players.iter().map(|p| p.info.stats.damage_taken))
                .max()
                .unwrap();
            let teams = self.teams.iter().map(|t| {
                team(
                    t,
                    &self.player,
                    max_damage_dealt,
                    max_damage_taken,
                    self.duration,
                )
            });

            let match_details = container(column(teams));

            container(row![
                widget::left_border(self.result),
                column![overview.height(Length::Shrink), match_details,]
            ])
            .max_height(600.0)
        } else {
            container(row![
                widget::left_border(self.result).max_height(100.0),
                overview,
            ])
            .max_height(100.0)
        };

        container(game)
            .width(Length::Fill)
            .height(Length::Shrink)
            .style(theme::dark_container())
            .into()
    }
}

fn team<'a>(
    team: &Team,
    summoner: &Player,
    max_damage_dealt: u32,
    max_damage_taken: u32,
    game_duration: Duration,
) -> Element<'a, Message> {
    // TODO: align header with content
    let header = row![
        row![
            widget::bold(formatting::win(team.result))
                .style(theme::win_color(team.result))
                .size(12),
            text(format!("({})", formatting::team(team.id)))
                .size(10)
                .style(theme::sub_text()),
        ]
        .spacing(4)
        .align_items(Alignment::Center),
        horizontal_space().width(Length::Fill),
        small_text("KDA"),
        horizontal_space().width(Length::Fill),
        small_text("Damage"),
        horizontal_space().width(Length::Fill),
        small_text("Vision"),
        horizontal_space().width(Length::Fill),
        small_text("CS"),
        horizontal_space().width(Length::Fill),
        small_text("Items"),
        horizontal_space().width(Length::Fill),
    ]
    .padding(4);

    let total_team_kills: u32 = team.players.iter().map(|p| p.info.stats.kills).sum();

    let player = |player: &Player| -> Element<_> {
        let is_player = player.info.puuid == summoner.info.puuid;

        let champion_icon = image(player.assets.champion_image.clone())
            .width(32.0)
            .height(32.0);
        let champion_level = container(widget::bold(player.info.stats.level).size(8))
            .padding([1, 2, 0, 2])
            .style(theme::summoner_level_container());

        // TODO: fix `champion_level` overlay not being clipped on the `scrollable`
        let champion = Modal::new(champion_icon, champion_level)
            .horizontal_alignment(Alignment::Start)
            .vertical_alignment(Alignment::End);

        let spell_and_runes = {
            let champion_spells = row![
                image(player.assets.summoner_spell_images[0].clone())
                    .width(16.0)
                    .height(16.0),
                image(player.assets.summoner_spell_images[1].clone())
                    .width(16.0)
                    .height(16.0)
            ]
            .spacing(2)
            .align_items(Alignment::Center);

            let champion_runes = row![
                image(player.assets.runes_images[0].clone())
                    .width(16.0)
                    .height(16.0),
                image(player.assets.runes_images[1].clone())
                    .width(16.0)
                    .height(16.0)
            ]
            .spacing(2)
            .align_items(Alignment::Center);
            column![champion_spells, champion_runes]
                .spacing(2)
                .align_items(Alignment::Center)
        };

        let name = if is_player {
            widget::bold(&player.info.name).size(12)
        } else {
            text(&player.info.name).size(12)
        };

        let kda = {
            let stats = player.info.stats;
            let kill_participation = stats.kills as f32 / total_team_kills as f32 * 100.0;

            column![
                smaller_text(format!(
                    "{}/{}/{} ({:.1}%)",
                    stats.kills, stats.deaths, stats.assists, kill_participation
                )),
                smaller_text(formatting::kda(stats.kills, stats.deaths, stats.assists)),
            ]
            .align_items(Alignment::Center)
        };

        let damage = {
            let damage_dealt = column![
                smaller_text(&player.info.stats.damage_dealt),
                progress_bar(
                    0.0..=max_damage_dealt as f32,
                    player.info.stats.damage_dealt as f32
                )
                .width(48.0)
                .height(6.0)
                .style(theme::fill_bar(theme::RED)),
            ]
            .align_items(Alignment::Center);

            let damage_taken = column![
                smaller_text(&player.info.stats.damage_taken),
                progress_bar(
                    0.0..=max_damage_taken as f32,
                    player.info.stats.damage_taken as f32
                )
                .width(48.0)
                .height(6.0)
                .style(theme::fill_bar(theme::LIGHT_BACKGROUND)),
            ]
            .align_items(Alignment::Center);

            row![damage_dealt, damage_taken,]
                .spacing(8)
                .align_items(Alignment::Center)
        };

        let wards = column![
            smaller_text(player.info.stats.control_wards),
            row![
                smaller_text(player.info.stats.wards_placed),
                smaller_text("/"),
                smaller_text(player.info.stats.wards_removed)
            ]
            .spacing(1)
            .align_items(Alignment::Center),
        ]
        .align_items(Alignment::Center);

        let cs = column![
            smaller_text(player.info.stats.creep_score),
            smaller_text(format!(
                "{:.1}/m",
                player.info.stats.creep_score as f32 / game_duration.0.whole_minutes() as f32,
            )),
        ]
        .align_items(Alignment::Center);

        let items = {
            let items = player.assets.item_images.iter().cloned().map(small_item);
            row(items).spacing(2).align_items(Alignment::Center)
        };

        let ward = small_item(player.assets.trinket_image.clone());

        let content = row![
            champion,
            horizontal_space().width(4),
            spell_and_runes,
            horizontal_space().width(8),
            name,
            horizontal_space().width(Length::Fill),
            kda,
            horizontal_space().width(16),
            damage,
            horizontal_space().width(16),
            wards,
            horizontal_space().width(16),
            cs,
            horizontal_space().width(16),
            items,
            horizontal_space().width(4),
            ward,
            horizontal_space().width(16),
        ]
        .width(Length::Fill)
        .align_items(Alignment::Center);

        container(content)
            .style(theme::team_player_container(team.result, is_player))
            .padding(4)
            .into()
    };

    let players = team.players.iter().map(player);

    let content = column![
        container(header.padding(4))
            .style(theme::team_header_container())
            .padding(1),
        column(players).padding(1).spacing(4),
    ];

    container(content)
        .style(theme::team_player_container(team.result, false))
        .into()
}

fn small_item<'a>(item: Option<image::Handle>) -> Element<'a, Message> {
    match item {
        Some(handle) => image(handle).width(20.0).height(20.0).into(),
        None => container(iced::widget::Space::new(20.0, 20.0))
            .style(theme::search_bar_container())
            .into(),
    }
}

fn small_text<'a>(content: impl ToString) -> Element<'a, Message> {
    text(content).size(12).style(theme::sub_text()).into()
}

fn smaller_text<'a>(content: impl ToString) -> Element<'a, Message> {
    text(content).size(10).style(theme::sub_text()).into()
}

fn truncate(string: String, max: usize) -> String {
    match string.char_indices().nth(max) {
        None => string,
        Some((idx, _)) => format!("{string:.idx$}…"),
    }
}
