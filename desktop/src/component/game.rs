use super::*;
use crate::assets::load_champion_icon;
use crate::assets::load_item_icon;
use crate::assets::load_runes_icon;
use crate::assets::load_summoner_spell_icon;
use crate::theme;
use crate::theme::chevron_down_icon;
use crate::theme::chevron_up_icon;
use crate::widget;
use iced::widget::image;
use iced::widget::{button, column, container, row, text, vertical_space, Space};
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
    pub fn new(win: bool, assets: &crate::assets::Assets, champion: &'static str) -> Self {
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
            time: Time(time::OffsetDateTime::now_utc().saturating_sub(time::Duration::days(1))),
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
                    container(text("28:33").size(10).style(theme::sub_text()))
                        .padding([0, 0, 0, 1]),
                ]
                .padding([4, 0, 0, 0])
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
                    )
                    .padding([0, 0, 0, 1]),
                ],
                vertical_space(2),
                role
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
                    widget::bold(summoner.to_string()).size(8.0)
                } else {
                    widget::small_text(summoner.to_string())
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
            .spacing(32)
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
