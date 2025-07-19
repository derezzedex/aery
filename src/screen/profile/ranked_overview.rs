use iced::{
    Alignment, Element, Length, padding,
    widget::{button, column, container, horizontal_space, image, progress_bar, row, text},
};

use crate::core::game;
use crate::core::summoner::Tier;
use crate::formatting;
use crate::profile;
use crate::theme;
use crate::theme::icon;

fn ranked_container<'a>(
    queue: game::Queue,
    tier: Tier,
    wins: u16,
    losses: u16,
    handle: image::Handle,
) -> Element<'a, Message> {
    let left_bar = container(horizontal_space().width(2))
        .style(theme::left_bar)
        .height(18);

    let chevron_down = icon::chevron_down().width(12.0).height(12.0);

    let size = match queue {
        game::Queue::RankedSolo => 100.0,
        game::Queue::RankedFlex => 80.0,
        _ => unreachable!(),
    };
    let emblem_size = match queue {
        game::Queue::RankedSolo => match tier {
            Tier::Challenger(_) | Tier::Grandmaster(_) | Tier::Master(_) => 100.0,
            Tier::Emerald(_) | Tier::Diamond(_) => 90.0,
            Tier::Platinum(_) | Tier::Gold(_) | Tier::Silver(_) => 80.0,
            Tier::Bronze(_) | Tier::Iron(_) => 70.0,
        },
        game::Queue::RankedFlex => match tier {
            Tier::Challenger(_) | Tier::Grandmaster(_) | Tier::Master(_) => 80.0,
            Tier::Emerald(_) | Tier::Diamond(_) => 70.0,
            Tier::Platinum(_) | Tier::Gold(_) | Tier::Silver(_) => 60.0,
            Tier::Bronze(_) | Tier::Iron(_) => 50.0,
        },
        _ => unreachable!(),
    };
    let lp = tier.points();
    let tier = match tier {
        Tier::Challenger(_) | Tier::Grandmaster(_) | Tier::Master(_) => formatting::tier(tier),
        _ => format!(
            "{} {}",
            formatting::tier(tier),
            formatting::division_or_points(tier)
        ),
    };

    let win_rate = ((wins as f32 / (wins + losses) as f32) * 100.0).ceil();

    container(column![
        row![
            left_bar,
            horizontal_space().width(4),
            text(queue.to_string()).font(theme::BOLD).size(14),
            horizontal_space().width(Length::Fill),
            button(chevron_down)
                .style(theme::expand)
                .padding(4)
                .on_press(Message::Expand),
        ]
        .padding(padding::all(12).bottom(0))
        .spacing(2)
        .align_y(Alignment::Center),
        row![
            container(image(handle).width(emblem_size).height(emblem_size))
                .center_x(size)
                .center_y(size),
            column![
                row![
                    text(tier).font(theme::BOLD).size(16),
                    text("·").style(theme::text).size(16),
                    text(format!("{lp} LP")).style(theme::text).size(12)
                ]
                .align_y(Alignment::Center)
                .spacing(4),
                row![
                    text(format!("{wins}W {losses}L"))
                        .style(theme::text)
                        .size(12),
                    text("·").style(theme::text),
                    text(format!("{win_rate:.0}%"))
                        .font(theme::BOLD)
                        .style(theme::victory)
                        .size(12)
                ]
                .align_y(Alignment::Center)
                .spacing(4),
                container(progress_bar(0.0..=100.0, win_rate).style(theme::ratio_bar))
                    .width(120)
                    .height(4),
            ]
            .spacing(2)
        ]
        .padding(padding::left(18).right(18))
        .spacing(16)
        .align_y(Alignment::Center),
    ])
    .style(theme::dark)
    .width(280)
    .into()
}

fn unranked_container<'a>(queue: game::Queue) -> Element<'a, Message> {
    let left_bar = container(horizontal_space().width(2))
        .style(theme::left_bar)
        .height(18);

    container(
        row![
            left_bar,
            horizontal_space().width(4),
            text(queue.to_string()).font(theme::BOLD).size(14),
            horizontal_space().width(Length::Fill),
            row![
                icon::unranked().width(18.0).height(18.0),
                text("Unranked").style(theme::text).size(12)
            ]
            .align_y(Alignment::Center)
            .spacing(4),
        ]
        .align_y(Alignment::Center),
    )
    .padding(10)
    .style(theme::dark)
    .width(280)
    .into()
}

#[derive(Debug, Clone)]
pub enum Message {
    Expand,
}

#[derive(Debug, Clone)]
struct Stats {
    tier: Tier,
    wins: u16,
    losses: u16,
    handle: image::Handle,
}

#[derive(Debug, Clone)]
pub struct RankedOverview {
    solo_duo: Option<Stats>,
    flex: Option<Stats>,
}

impl RankedOverview {
    pub fn from_profile(assets: &crate::assets::Assets, profile: &profile::Data) -> Self {
        let solo_duo = profile
            .leagues
            .iter()
            .find(|league| league.queue_kind() == game::Queue::RankedSolo)
            .filter(|league| league.tier().is_some())
            .map(|league| Stats {
                tier: league.tier().unwrap(),
                wins: league.wins() as u16,
                losses: league.losses() as u16,
                handle: assets.emblem(&league.tier().unwrap()),
            });

        let flex = profile
            .leagues
            .iter()
            .find(|league| league.queue_kind() == game::Queue::RankedFlex)
            .filter(|league| league.tier().is_some())
            .map(|league| Stats {
                tier: league.tier().unwrap(),
                wins: league.wins() as u16,
                losses: league.losses() as u16,
                handle: assets.emblem(&league.tier().unwrap()),
            });

        Self { solo_duo, flex }
    }

    pub fn update(&mut self, _message: Message) {}

    pub fn view(&self) -> Element<Message> {
        let solo_duo = match &self.solo_duo {
            Some(stats) => ranked_container(
                game::Queue::RankedSolo,
                stats.tier,
                stats.wins,
                stats.losses,
                stats.handle.clone(),
            ),
            None => unranked_container(game::Queue::RankedSolo),
        };

        let flex = match &self.flex {
            Some(stats) => ranked_container(
                game::Queue::RankedFlex,
                stats.tier,
                stats.wins,
                stats.losses,
                stats.handle.clone(),
            ),
            None => unranked_container(game::Queue::RankedFlex),
        };

        column![solo_duo, flex,]
            .spacing(4)
            .align_x(Alignment::Center)
            .into()
    }
}
