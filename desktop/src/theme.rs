use crate::core::game;

use iced::border;
use iced::color;
use iced::font;
use iced::overlay::menu;
use iced::widget;
use iced::widget::button;
use iced::widget::container;
use iced::widget::pick_list;
use iced::widget::progress_bar;
use iced::widget::text_input;
use iced::Background;
use iced::Color;

pub const ROBOTO_FLEX_TTF: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/font/RobotoFlex-Regular.ttf"
))
.as_slice();

pub const DEFAULT_FONT: iced::Font = iced::Font {
    family: font::Family::Name("RobotoFlex"),
    weight: font::Weight::Normal,
    stretch: font::Stretch::Normal,
    style: font::Style::Normal,
};

pub const SEMIBOLD: iced::Font = iced::Font {
    weight: iced::font::Weight::Semibold,
    ..DEFAULT_FONT
};

pub const BOLD: iced::Font = iced::Font {
    weight: iced::font::Weight::Bold,
    ..DEFAULT_FONT
};

pub const DARKER_BACKGROUND: Color = color!(0x0d0d0d);
pub const DARK_BACKGROUND: Color = color!(0x1a1a1a);
pub const LIGHTER_BACKGROUND: Color = color!(0x333333);
pub const LIGHT_BACKGROUND: Color = color!(0xf2f2f2);

pub const RED: Color = color!(0xff5733);
pub const BLUE: Color = color!(0x0094ff);
pub const GOLD: Color = color!(0xcd8837);

pub const GRAY_TEXT: Color = color!(0x808080);
pub const SUB_TEXT: Color = color!(0xcccccc);

pub const LIGHTER_ALPHA: Color = color!(0x333333, 0.7);
pub const LIGHT_ALPHA: Color = color!(0x666666, 0.9);
pub const BLUE_ALPHA: Color = color!(0x0094ff, 0.7);

pub mod icon {
    use crate::core::game;
    use iced::widget::image;
    use iced::widget::svg;
    use iced::Color;

    fn colored(color: Color) -> impl Fn(&iced::Theme, svg::Status) -> svg::Style {
        move |_, _| svg::Style { color: Some(color) }
    }

    pub fn chevron_down<'a>() -> svg::Svg<'a> {
        let path = svg::Handle::from_path(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/img/icons/chevron-down.svg"
        ));

        svg(path).style(colored(Color::WHITE))
    }

    pub fn chevron_up<'a>() -> svg::Svg<'a> {
        let path = svg::Handle::from_path(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/img/icons/chevron-up.svg"
        ));

        svg(path).style(colored(Color::WHITE))
    }

    pub fn search<'a>() -> svg::Svg<'a> {
        let path = svg::Handle::from_path(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/img/icons/search.svg"
        ));

        svg(path).style(colored(Color::WHITE))
    }

    pub fn clock<'a>() -> svg::Svg<'a> {
        let path = svg::Handle::from_path(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/img/icons/clock.svg"
        ));

        svg(path)
    }

    pub fn unranked() -> image::Handle {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/img/emblems/minicrests/unranked.png"
        );
        image::Handle::from_path(path)
    }

    pub fn role(role: game::Role) -> image::Handle {
        let file = match role {
            game::Role::Bottom => "bottom.png",
            game::Role::Jungle => "jungle.png",
            game::Role::Mid => "mid.png",
            game::Role::Support => "support.png",
            game::Role::Top => "top.png",
        };

        let path = format!("{}/assets/img/position/{file}", env!("CARGO_MANIFEST_DIR"),);

        image::Handle::from_path(path)
    }
}

pub fn left_bar(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(BLUE)),
        border: border::width(0),
        text_color: Some(Color::WHITE),
        ..Default::default()
    }
}

pub fn search_bar(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(LIGHTER_BACKGROUND)),
        border: border::width(0),
        text_color: Some(SUB_TEXT),
        ..Default::default()
    }
}

pub fn timeline(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(DARKER_BACKGROUND)),
        border: border::width(0),
        text_color: Some(Color::WHITE),
        ..Default::default()
    }
}

pub fn summoner_icon(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(LIGHT_BACKGROUND)),
        border: border::rounded(2).width(2).color(GOLD),
        text_color: Some(Color::WHITE),
        ..Default::default()
    }
}

pub fn summoner_level(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(DARK_BACKGROUND)),
        border: border::rounded(2).width(1).color(GOLD),
        text_color: Some(Color::WHITE),
        ..Default::default()
    }
}

pub fn dark(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(DARK_BACKGROUND)),
        border: border::rounded(4),
        text_color: Some(Color::WHITE),
        ..Default::default()
    }
}

pub fn icon(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(LIGHT_BACKGROUND)),
        border: border::width(0),
        text_color: Some(Color::BLACK),
        ..Default::default()
    }
}

pub fn left_border(result: game::Result) -> container::Style {
    container::Style {
        background: Some(Background::Color(win_color(result))),
        border: border::rounded(border::left(4)),
        text_color: Some(Color::WHITE),
        ..Default::default()
    }
}

pub fn team_header(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(DARK_BACKGROUND)),
        border: border::width(0),
        text_color: Some(Color::WHITE),
        ..Default::default()
    }
}

pub fn team_player(is_player: bool) -> container::Style {
    if is_player {
        container::Style {
            background: Some(Background::Color(LIGHTER_ALPHA)),
            border: border::width(0),
            text_color: Some(Color::WHITE),
            ..Default::default()
        }
    } else {
        container::Style {
            background: Some(Background::Color(DARK_BACKGROUND)),
            border: border::width(0),
            text_color: Some(Color::WHITE),
            ..Default::default()
        }
    }
}

pub fn win_color(result: impl Into<game::Result>) -> Color {
    match result.into() {
        game::Result::Remake => SUB_TEXT,
        game::Result::Surrender | game::Result::Defeat => RED,
        game::Result::Victory => BLUE,
    }
}

pub fn expander(expanded: bool, status: button::Status) -> button::Style {
    let background = if matches!(status, button::Status::Hovered) {
        Some(Background::Color(LIGHT_ALPHA))
    } else {
        Some(Background::Color(LIGHTER_ALPHA))
    };

    if expanded {
        button::Style {
            background,
            border: border::rounded(border::right(4)),
            text_color: Color::WHITE,
            ..Default::default()
        }
    } else {
        button::Style {
            background,
            border: border::rounded(border::right(4).bottom(4)),
            text_color: Color::WHITE,
            ..Default::default()
        }
    }
}

pub fn update(_theme: &iced::Theme, status: button::Status) -> button::Style {
    let background = if matches!(status, button::Status::Hovered) {
        Some(Background::Color(BLUE_ALPHA))
    } else {
        Some(Background::Color(BLUE))
    };

    button::Style {
        background,
        border: border::rounded(2),
        text_color: Color::WHITE,
        ..Default::default()
    }
}

pub fn region(theme: &iced::Theme, status: pick_list::Status) -> pick_list::Style {
    pick_list::Style {
        background: Background::Color(LIGHTER_BACKGROUND),
        border: border::width(0),
        ..pick_list::default(theme, status)
    }
}

pub fn region_menu(theme: &iced::Theme) -> menu::Style {
    menu::Style {
        background: Background::Color(LIGHTER_BACKGROUND),
        border: border::width(0),
        selected_background: Background::Color(LIGHT_ALPHA),
        ..menu::default(theme)
    }
}

pub fn expand(_theme: &iced::Theme, status: button::Status) -> button::Style {
    let background = if matches!(status, button::Status::Hovered) {
        Some(Background::Color(LIGHT_ALPHA))
    } else {
        Some(Background::Color(LIGHTER_BACKGROUND))
    };

    button::Style {
        background,
        border: border::rounded(2),
        text_color: Color::WHITE,
        ..Default::default()
    }
}

pub fn queue_picklist(
    selected: bool,
    theme: &iced::Theme,
    status: pick_list::Status,
) -> pick_list::Style {
    let color = if selected { LIGHT_ALPHA } else { LIGHTER_ALPHA };

    pick_list::Style {
        background: Background::Color(color),
        border: border::rounded(2),
        ..pick_list::default(theme, status)
    }
}

pub fn queue_filter(selected: bool, status: button::Status) -> button::Style {
    let background = if matches!(status, button::Status::Hovered) {
        Some(Background::Color(LIGHT_BACKGROUND.scale_alpha(0.4)))
    } else if selected {
        Some(Background::Color(LIGHT_ALPHA))
    } else {
        Some(Background::Color(LIGHTER_ALPHA))
    };

    button::Style {
        background,
        border: border::rounded(4),
        text_color: Color::WHITE,
        ..Default::default()
    }
}

pub fn ratio_bar(_theme: &iced::Theme) -> progress_bar::Style {
    progress_bar::Style {
        background: Background::Color(RED),
        bar: Background::Color(BLUE),
        border: border::rounded(0),
    }
}

pub fn fill_bar(color: Color) -> progress_bar::Style {
    progress_bar::Style {
        background: Background::Color(LIGHTER_BACKGROUND),
        bar: Background::Color(color),
        border: border::rounded(border::radius(0)),
    }
}

pub fn rule(color: iced::Color) -> widget::rule::Style {
    widget::rule::Style {
        color,
        width: 1,
        radius: 0.0.into(),
        fill_mode: widget::rule::FillMode::Full,
    }
}

pub fn scrollable(
    _theme: &iced::Theme,
    status: widget::scrollable::Status,
) -> widget::scrollable::Style {
    use widget::scrollable;

    let scrollbar = scrollable::Rail {
        background: None,
        border: border::rounded(2),
        scroller: scrollable::Scroller {
            color: LIGHTER_ALPHA,
            border: border::rounded(2),
        },
    };

    match status {
        scrollable::Status::Active => scrollable::Style {
            container: container::Style::default(),
            vertical_rail: scrollbar,
            horizontal_rail: scrollbar,
            gap: None,
        },
        scrollable::Status::Hovered {
            is_horizontal_scrollbar_hovered: is_horizontal_scrollbar,
            is_vertical_scrollbar_hovered: is_vertical_scrollbar,
        }
        | scrollable::Status::Dragged {
            is_horizontal_scrollbar_dragged: is_horizontal_scrollbar,
            is_vertical_scrollbar_dragged: is_vertical_scrollbar,
        } => {
            let hovered = scrollable::Rail {
                background: Some(Background::Color(DARK_BACKGROUND)),
                scroller: scrollable::Scroller {
                    color: LIGHT_ALPHA,
                    ..scrollbar.scroller
                },
                ..scrollbar
            };

            scrollable::Style {
                container: container::Style::default(),
                vertical_rail: if is_vertical_scrollbar {
                    hovered
                } else {
                    scrollbar
                },
                horizontal_rail: if is_horizontal_scrollbar {
                    hovered
                } else {
                    scrollbar
                },
                gap: None,
            }
        }
    }
}

pub fn search_text_input(_theme: &iced::Theme, status: text_input::Status) -> text_input::Style {
    let active = text_input::Style {
        background: Background::Color(Color::TRANSPARENT),
        border: iced::Border {
            radius: 0.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        icon: Color::TRANSPARENT,
        placeholder: SUB_TEXT,
        value: Color::WHITE,
        selection: BLUE,
    };

    match status {
        text_input::Status::Active => active,
        text_input::Status::Hovered | text_input::Status::Focused { .. } => text_input::Style {
            background: Background::Color(LIGHTER_ALPHA),
            ..active
        },
        text_input::Status::Disabled => widget::text_input::Style {
            background: Background::Color(DARKER_BACKGROUND),
            value: SUB_TEXT,
            ..active
        },
    }
}
