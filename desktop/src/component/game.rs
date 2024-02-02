use super::*;
use crate::assets::load_champion_icon;
use crate::assets::load_item_icon;
use crate::assets::load_runes_icon;
use crate::assets::load_summoner_spell_icon;
use crate::core;
use crate::core::{Duration, Time};
use crate::theme;
use crate::theme::chevron_down_icon;
use crate::theme::chevron_up_icon;
use crate::widget;
use iced::widget::image;
use iced::widget::vertical_space;
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
pub struct Game {
    win: bool,
    queue: Queue,
    time: Time,
    duration: Duration,
    role: Option<Role>,
    champion_image: image::Handle,
    summoner_spell_images: [image::Handle; 2],
    runes_images: [image::Handle; 2],
    item_images: [Option<image::Handle>; 6],
    trinket_image: image::Handle,
    summoner_icons: [image::Handle; 10],
    player_kills: u16,
    player_deaths: u16,
    player_assists: u16,
    player_creep_score: u16,
    player_vision_score: u16,
    player_index: usize,
    summoners: Vec<Summoner>,

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
        game: &core::GameMatch,
    ) -> Self {
        let partipants = game.participants();
        let (player_index, player) = partipants
            .iter()
            .enumerate()
            .find(|(_, p)| p.puuid == summoner.puuid())
            .map(|(i, p)| (i, p.clone()))
            .unwrap();

        let champion_image = load_champion_icon(assets, player.champion);

        let summoner_spell_images = [
            load_summoner_spell_icon(assets, player.summoner_spells.first()),
            load_summoner_spell_icon(assets, player.summoner_spells.second()),
        ];
        let runes_images = [
            load_runes_icon(assets, player.rune_page.primary.keystone()),
            load_runes_icon(assets, player.rune_page.secondary.keystone()),
        ];

        let item_images = player
            .inventory
            .into_iter()
            .map(|item| item.map(|item| load_item_icon(assets, item)))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let trinket_image = load_item_icon(assets, player.trinket.into());

        let summoner_icons = partipants
            .iter()
            .map(|participant| load_champion_icon(assets, participant.champion))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let role = match player.role {
            core::Role::Bottom => Some(Role::Bottom),
            core::Role::Top => Some(Role::Top),
            core::Role::Jungle => Some(Role::Jungle),
            core::Role::Support => Some(Role::Support),
            core::Role::Mid => Some(Role::Mid),
            core::Role::Unknown => None,
        };

        Game {
            win: player.won,
            queue: Queue::RankedFlex,
            time: game.created_at(),
            duration: game.duration(),
            role,
            champion_image,
            summoner_spell_images,
            runes_images,
            item_images,
            trinket_image,
            summoner_icons,
            player_kills: player.stats.kills() as u16,
            player_deaths: player.stats.deaths() as u16,
            player_assists: player.stats.assists() as u16,
            player_creep_score: player.stats.creep_score() as u16
                + player.stats.monster_score() as u16,
            player_vision_score: player.stats.vision_score() as u16,
            player_index,
            summoners: partipants
                .iter()
                .map(|participant| Summoner(participant.name.clone()))
                .collect(),

            is_expanded: false,
        }
    }

    pub fn new(win: bool, assets: &crate::assets::Assets, champion: core::Champion) -> Self {
        let champion_image = load_champion_icon(assets, champion);
        let summoner_spell_images = [
            load_summoner_spell_icon(assets, core::SummonerSpell::new(14)),
            load_summoner_spell_icon(assets, core::SummonerSpell::new(4)),
        ];
        let runes_images = [
            load_runes_icon(assets, core::RuneKeystone::new(8010)),
            load_runes_icon(assets, core::RuneKeystone::new(8400)),
        ];
        let item_images = [
            Some(load_item_icon(assets, core::Item::new(1001))),
            Some(load_item_icon(assets, core::Item::new(6630))),
            Some(load_item_icon(assets, core::Item::new(4401))),
            Some(load_item_icon(assets, core::Item::new(3143))),
            Some(load_item_icon(assets, core::Item::new(3742))),
            Some(load_item_icon(assets, core::Item::new(6333))),
        ];
        let trinket_image = load_item_icon(assets, core::Item::new(3364));

        let summoner_icons = [
            load_champion_icon(assets, champion),
            load_champion_icon(assets, core::Champion::new(1)),
            load_champion_icon(assets, core::Champion::new(101)),
            load_champion_icon(assets, core::Champion::new(14)),
            load_champion_icon(assets, core::Champion::new(122)),
            load_champion_icon(assets, core::Champion::new(897)),
            load_champion_icon(assets, core::Champion::new(62)),
            load_champion_icon(assets, core::Champion::new(4)),
            load_champion_icon(assets, core::Champion::new(61)),
            load_champion_icon(assets, core::Champion::new(202)),
        ];

        Game {
            win,
            queue: Queue::RankedFlex,
            time: Time(time::OffsetDateTime::now_utc().saturating_sub(time::Duration::days(1))),
            duration: Duration(
                time::Duration::minutes(28).saturating_add(time::Duration::seconds(33)),
            ),
            role: Some(Role::Mid),
            champion_image,
            summoner_spell_images,
            runes_images,
            item_images,
            trinket_image,
            summoner_icons,
            player_kills: 1,
            player_deaths: 6,
            player_assists: 12,
            player_creep_score: 151,
            player_vision_score: 18,
            player_index: 0,
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
                    .align_items(Alignment::End)
                    .spacing(2),
                    container(
                        text(self.duration.to_string())
                            .size(10)
                            .style(theme::sub_text())
                    ),
                ]
                .spacing(2)
                .into()
            } else {
                Space::new(0, 0).into()
            };

            column![
                widget::bold(formatting::win(self.win))
                    .style(theme::win_color(self.win))
                    .size(18),
                column![
                    text(self.queue.to_string()).size(11),
                    container(
                        text(self.time.to_string())
                            .style(theme::sub_text())
                            .size(10)
                    ),
                ],
                vertical_space(Length::Fill),
                role,
            ]
            .align_items(Alignment::Start)
            .spacing(2)
            .padding(2)
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
                text(self.player_kills).size(15),
                text("/").style(theme::gray_text()).size(15),
                text(self.player_deaths).style(theme::red_text()).size(15),
                text("/").style(theme::gray_text()).size(15),
                text(self.player_assists).size(15)
            ]
            .align_items(Alignment::Center)
            .spacing(2);

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
                item_icon(Some(self.trinket_image.clone())),
            ]
            .spacing(2)
        };

        let mut left_players: Vec<Element<_>> = self
            .summoners
            .iter()
            .enumerate()
            .map(|(i, summoner)| {
                let summoner_icon = image(self.summoner_icons[i].clone())
                    .width(16.0)
                    .height(16.0);
                let summoner_name = if i == self.player_index {
                    text(summoner.to_string())
                        .size(8.0)
                        .line_height(iced::Pixels(12.0))
                } else {
                    widget::small_text(summoner.to_string())
                        .size(8.0)
                        .line_height(iced::Pixels(12.0))
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
            .height(Length::Shrink)
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
            .spacing(28)
            .padding(4)
            .align_items(Alignment::Center),
            expand_button.padding(0),
        ])
        .max_height(100.0);

        let game = if self.is_expanded {
            let match_details = container(Space::new(0.0, 400.0));

            container(row![
                widget::left_border(self.win),
                column![overview, match_details,]
            ])
            .max_height(600.0)
        } else {
            container(row![
                widget::left_border(self.win).max_height(100.0),
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
