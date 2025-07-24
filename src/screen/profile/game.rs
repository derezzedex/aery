use crate::core;
use crate::core::game;
use crate::core::game::item;
use crate::formatting;
use crate::theme;
use crate::theme::icon;
use crate::widget;
use aery_core::account;
use iced::widget::horizontal_space;
use iced::widget::image;
use iced::widget::progress_bar;
use iced::widget::stack;
use iced::widget::tooltip;
use iced::widget::vertical_space;
use iced::widget::{Space, button, column, container, row, text};
use iced::{Alignment, Element, Length, alignment};
use itertools::Itertools;

fn champion_icon<'a>(handle: image::Handle) -> Element<'a, Message> {
    let icon = iced::widget::image(handle)
        .width(48.0)
        .height(48.0)
        .content_fit(iced::ContentFit::Cover);

    container(icon).into()
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

    container(icon).center_x(22.0).center_y(22.0).into()
}

fn item_icon<'a>(handle: Option<image::Handle>) -> Element<'a, Message> {
    let icon: Element<'_, _> = if let Some(handle) = handle {
        iced::widget::image(handle)
            .width(28.0)
            .height(28.0)
            .content_fit(iced::ContentFit::Fill)
            .into()
    } else {
        container(iced::widget::Space::new(28.0, 28.0))
            .style(theme::team_header)
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
        let champion_image = assets.champion(&participant.champion);

        let summoner_spell_images = [
            assets.spell(&participant.summoner_spells.first()),
            assets.spell(&participant.summoner_spells.second()),
        ];
        let runes_images = [
            assets.rune(&participant.rune_page.primary.keystone.rune),
            assets.rune(&participant.rune_page.secondary.path.into()),
        ];

        let item_images = participant
            .inventory
            .into_iter()
            .map(|item| item.map(|item| assets.item(&item)))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let trinket_image = match participant.trinket {
            item::Trinket(0) => None,
            trinket => Some(assets.item(&trinket.into())),
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
    time: time::OffsetDateTime,
    duration: time::Duration,
    player: Player,
    teams: Vec<Team>,

    is_expanded: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    ExpandPressed,
    NamePressed(account::RiotId),
}

#[derive(Debug, Clone)]
pub enum Event {
    NamePressed(account::RiotId),
}

impl Game {
    pub fn from_summoner_game(
        assets: &crate::assets::Assets,
        summoner: &core::Summoner,
        game: &core::Game,
    ) -> Self {
        let player = game.player(summoner.puuid()).unwrap();
        let player = Player::from_participant(assets, player);

        let teams = game
            .players
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
            result: player.info.result,
            queue: game.queue,
            time: game.created_at_time(),
            duration: game.duration_time(),
            player,
            teams,

            is_expanded: false,
        }
    }

    pub fn queue(&self) -> game::Queue {
        self.queue
    }

    pub fn update(&mut self, message: Message) -> Option<Event> {
        match message {
            Message::ExpandPressed => self.is_expanded = !self.is_expanded,
            Message::NamePressed(riot_id) => return Some(Event::NamePressed(riot_id)),
        }

        None
    }

    pub fn view(&self) -> Element<'_, Message> {
        let now = time::OffsetDateTime::now_utc();

        let match_stats = {
            // TODO: track and display points gained/lost
            // let points_icon: Element<Message> = small_icon().into();
            // let result_points = row![points_icon, text("31 LP").size(16)]
            //     .spacing(2)
            //     .align_items(Alignment::Center);

            let role: Element<'_, _> = if let Some(role) = self.player.info.role {
                row![
                    icon::role(role).width(12.0).height(12.0),
                    text(formatting::role(role)).style(theme::text).size(10),
                ]
                .align_y(Alignment::Center)
                .spacing(4)
                .into()
            } else {
                Space::new(0, 0).into()
            };

            column![
                text(formatting::win(self.result))
                    .font(theme::BOLD)
                    .style(move |theme| text::Style {
                        color: Some(theme::win_color(theme, self.result))
                    })
                    .size(18),
                column![
                    text(self.queue.to_string()).size(11),
                    container(
                        text(formatting::time_since(now, self.time))
                            .style(theme::text)
                            .size(10)
                    ),
                ],
                vertical_space().height(Length::Fill),
                column![
                    role,
                    row![
                        icon::clock().width(12.0).height(12.0),
                        container(
                            text(formatting::duration(self.duration))
                                .size(10)
                                .style(theme::text)
                        ),
                    ]
                    .align_y(Alignment::Center)
                    .spacing(4),
                ]
                .spacing(2),
            ]
            .align_x(Alignment::Start)
            .spacing(2)
            .padding(2)
        };

        let champion_info = {
            let champion_icon = champion_icon(self.player.assets.champion_image.clone());
            let champion_level = container(
                text(self.player.info.stats.level)
                    .font(theme::EXTRA_BOLD)
                    .size(11),
            )
            .padding([1, 4])
            .style(theme::summoner_level);

            // TODO: fix `champion_level` overlay not being clipped on the `scrollable`
            let champion = stack![
                champion_icon,
                container(champion_level)
                    .align_bottom(Length::Fill)
                    .align_right(Length::Fill)
            ];

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
                text(self.player.info.stats.kills)
                    .font(theme::BOLD)
                    .size(14),
                text("/").style(theme::text).size(14),
                text(self.player.info.stats.deaths)
                    .font(theme::BOLD)
                    .style(theme::defeat)
                    .size(14),
                text("/").style(theme::text).size(14),
                text(self.player.info.stats.assists)
                    .font(theme::BOLD)
                    .size(14)
            ]
            .align_y(Alignment::Center)
            .spacing(2);

            let other_stats = column![
                row![
                    text(formatting::kda(
                        self.player.info.stats.kills,
                        self.player.info.stats.deaths,
                        self.player.info.stats.assists
                    ))
                    .size(11)
                    .style(theme::text)
                ]
                .spacing(4)
                .align_y(Alignment::Center),
                row![
                    text(formatting::creep_score(
                        self.player.info.stats.creep_score,
                        self.duration.whole_minutes() as u32
                    ))
                    .size(11)
                    .style(theme::text)
                ]
                .spacing(4)
                .align_y(Alignment::Center),
                row![
                    text(formatting::vision_score(
                        self.player.info.stats.vision_score
                    ))
                    .size(11)
                    .style(theme::text)
                ]
                .spacing(4)
                .align_y(Alignment::Center),
            ]
            .align_x(Alignment::Center);

            column![kda, other_stats,].align_x(Alignment::Center)
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
            let summoner_icon = image(player.assets.champion_image.clone())
                .width(16.0)
                .height(16.0);

            let summoner_name = player_name(
                &player.info.riot_id,
                8,
                self.player.info.puuid == player.info.puuid,
            );

            row![summoner_icon, summoner_name]
                .align_y(Alignment::Center)
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

        let teams = row![
            column(blue_team).spacing(2).width(Length::Fill),
            column(red_team).spacing(2).width(Length::Fill),
        ]
        .spacing(8);

        let chevron_icon = if self.is_expanded {
            icon::chevron_up()
        } else {
            icon::chevron_down()
        };

        let expand_content = container(chevron_icon.width(12.0).height(12.0))
            .center_x(24)
            .align_y(alignment::Vertical::Bottom)
            .height(Length::Fill)
            .padding(2);

        let expand_button = button(expand_content)
            .height(Length::Shrink)
            .on_press(Message::ExpandPressed)
            .style(|theme, status| theme::expander(theme, status, self.is_expanded));

        let match_info = row![
            match_stats.width(Length::FillPortion(2)),
            row![
                champion_info,
                horizontal_space().width(Length::Fill),
                player_stats,
                horizontal_space().width(Length::Fill),
                player_items,
                horizontal_space().width(Length::Fill),
            ]
            .align_y(Alignment::Center)
            .spacing(8)
            .width(Length::FillPortion(6)),
            container(teams).center_x(Length::FillPortion(3)),
        ]
        .spacing(8)
        .width(Length::Fill)
        .padding(4)
        .align_y(Alignment::Center);

        let overview = container(row![match_info, expand_button.padding(0),]).max_height(100.0);

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
            let teams = self
                .teams
                .iter()
                .sorted_by_key(|team| team.id != self.player.info.team)
                .map(|t| {
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
            .style(theme::dark)
            .into()
    }
}

fn team<'a>(
    team: &'a Team,
    summoner: &'a Player,
    max_damage_dealt: u32,
    max_damage_taken: u32,
    game_duration: time::Duration,
) -> Element<'a, Message> {
    let result = row![
        text(formatting::win(team.result))
            .font(theme::BOLD)
            .style(move |theme| text::Style {
                color: Some(theme::win_color(theme, team.result))
            })
            .size(12),
        text(format!("({})", formatting::team(team.id))).size(12),
    ]
    .spacing(6)
    .align_y(Alignment::Center);

    let column = |content: Element<'a, Message>, size| {
        column![
            container(content)
                .style(theme::team_header)
                .center_x(Length::Fill)
        ]
        .align_x(Alignment::Center)
        .width(Length::FillPortion(size))
    };

    let columns = vec![
        column![
            container(result)
                .padding(4)
                .style(theme::team_header)
                .align_left(Length::Fill)
        ]
        .width(Length::FillPortion(3)),
        column(header("KDA"), 2),
        column(header("Damage"), 2),
        column(header("Vision"), 1),
        column(header("CS"), 1),
        column(header("Items"), 3),
    ];

    let total_team_kills = team.players.iter().map(|p| p.info.stats.kills).sum();

    let team = team
        .players
        .iter()
        .map(|p| {
            player(
                p,
                summoner,
                max_damage_dealt,
                max_damage_taken,
                total_team_kills,
                game_duration,
            )
        })
        .fold(columns, |columns, player| {
            columns
                .into_iter()
                .zip(player)
                .map(|(col, item)| col.push(item))
                .collect()
        });

    let content = row(team.into_iter().map(Element::from));

    container(content)
        .style(|theme| theme::team_player(theme, false))
        .into()
}

fn small_item<'a>(item: Option<image::Handle>) -> Element<'a, Message> {
    match item {
        Some(handle) => image(handle).width(20.0).height(20.0).into(),
        None => container(iced::widget::Space::new(20.0, 20.0))
            .style(theme::team_header)
            .into(),
    }
}

fn header<'a>(content: impl text::IntoFragment<'a>) -> Element<'a, Message> {
    container(text(content).font(theme::BOLD).size(12).style(theme::text))
        .padding(4)
        .into()
}

fn smaller_text<'a>(content: impl text::IntoFragment<'a>) -> Element<'a, Message> {
    text(content).size(11).style(theme::text).into()
}

fn truncated(mut string: String, max: usize) -> String {
    let count = string.chars().filter(|c| !c.is_ascii()).count();
    let max = if count > max / 2 { max * 2 } else { max };
    let n = string.len().min(max);
    if let Some(i) = (0..=n).rfind(|&m| string.is_char_boundary(m)) {
        string.truncate(i);
        string.push('…');
    }

    string
}

fn player_name<'a>(riot_id: &account::RiotId, size: u32, is_player: bool) -> Element<'a, Message> {
    let mut name = riot_id.name.clone().unwrap_or(String::from("Unknown"));
    let tag = riot_id.tagline.clone().unwrap_or(String::from("UKNW"));

    let overlay = container(
        text(format!("{name}#{tag}"))
            .color(iced::Color::WHITE)
            .font(theme::NOTO_SANS)
            .shaping(text::Shaping::Advanced)
            .size(14),
    )
    .padding(4)
    .style(container::dark);

    if !is_player {
        name = truncated(name, 8);
    }

    let mut name = text(name)
        .font(theme::NOTO_SANS)
        .shaping(text::Shaping::Advanced)
        .line_height(text::LineHeight::Absolute(12.0.into()))
        .size(size);

    if is_player {
        name = name.font(theme::BOLD);
    }

    let content = button(name)
        .style(button::text)
        .padding(0)
        .on_press(Message::NamePressed(riot_id.clone()));

    tooltip(content, overlay, tooltip::Position::Top).into()
}

fn player<'a>(
    player: &'a Player,
    summoner: &'a Player,
    max_damage_dealt: u32,
    max_damage_taken: u32,
    total_team_kills: u32,
    game_duration: time::Duration,
) -> [Element<'a, Message>; 7] {
    let is_player = player.info.puuid == summoner.info.puuid;

    let champion_icon = image(player.assets.champion_image.clone())
        .width(32.0)
        .height(32.0);
    let champion_level = container(
        text(player.info.stats.level)
            .font(theme::EXTRA_BOLD)
            .size(10),
    )
    .padding([1, 2])
    .style(theme::summoner_level);

    let champion = stack![
        champion_icon,
        container(champion_level)
            .align_bottom(Length::Fill)
            .align_right(Length::Fill)
    ];

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
        .align_y(Alignment::Center);

        let champion_runes = row![
            image(player.assets.runes_images[0].clone())
                .width(16.0)
                .height(16.0),
            image(player.assets.runes_images[1].clone())
                .width(16.0)
                .height(16.0)
        ]
        .spacing(2)
        .align_y(Alignment::Center);

        column![champion_spells, champion_runes]
            .spacing(2)
            .align_x(Alignment::Center)
    };

    let name = player_name(&player.info.riot_id, 12, is_player);

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
        .align_x(Alignment::Center)
    };

    let damage = {
        let damage_dealt = column![
            smaller_text(player.info.stats.damage_dealt),
            container(
                progress_bar(
                    0.0..=max_damage_dealt as f32,
                    player.info.stats.damage_dealt as f32
                )
                .style(|theme| theme::fill_bar(theme, theme.palette().danger)),
            )
            .width(48.0)
            .height(6.0),
        ]
        .align_x(Alignment::Center);

        let damage_taken = column![
            smaller_text(player.info.stats.damage_taken),
            container(
                progress_bar(
                    0.0..=max_damage_taken as f32,
                    player.info.stats.damage_taken as f32
                )
                .style(|theme| theme::fill_bar(
                    theme,
                    theme.extended_palette().secondary.strong.color
                )),
            )
            .width(48.0)
            .height(6.0),
        ]
        .align_x(Alignment::Center);

        row![damage_dealt, damage_taken,]
            .spacing(8)
            .align_y(Alignment::Center)
    };

    let wards = column![
        smaller_text(player.info.stats.control_wards),
        row![
            smaller_text(player.info.stats.wards_placed),
            smaller_text("/"),
            smaller_text(player.info.stats.wards_removed)
        ]
        .spacing(1)
        .align_y(Alignment::Center),
    ]
    .align_x(Alignment::Center);

    let cs = column![
        smaller_text(player.info.stats.creep_score),
        smaller_text(format!(
            "{:.1}/m",
            player.info.stats.creep_score as f32 / game_duration.whole_minutes() as f32,
        )),
    ]
    .align_x(Alignment::Center);

    let items = {
        let items = player.assets.item_images.iter().cloned().map(small_item);
        row(items).spacing(2).align_y(Alignment::Center)
    };

    let ward = small_item(player.assets.trinket_image.clone());

    let player = row![champion, spell_and_runes, name]
        .align_y(Alignment::Center)
        .spacing(4)
        .padding(4);

    let styled = |el: Element<'a, Message>| {
        container(el)
            .center(Length::Fill)
            .style(move |theme| theme::team_player(theme, is_player))
    };

    [
        styled(player.into())
            .height(Length::Fill)
            .align_left(Length::Fill),
        styled(kda.into()),
        styled(damage.into()),
        styled(wards.into()),
        styled(cs.into()),
        styled(items.into()),
        styled(ward),
    ]
    .map(Element::from)
}
