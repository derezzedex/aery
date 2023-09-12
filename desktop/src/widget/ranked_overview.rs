use iced::{
    widget::{button, column, container, horizontal_space, image, progress_bar, row, text},
    Alignment, Element, Length,
};

use crate::{chevron_down_icon, theme};

use super::{
    bold,
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
    pub fn new(assets: &crate::assets::Assets) -> RankedOverview {
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
