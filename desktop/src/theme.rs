use crate::core::game;

use iced::border;
use iced::font;
use iced::overlay::menu;
use iced::widget;
use iced::widget::button;
use iced::widget::container;
use iced::widget::pick_list;
use iced::widget::progress_bar;
use iced::widget::text_input;
use iced::Border;
use iced::{Background, Color, Theme};

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

pub fn logo<'a, Message: 'a>() -> iced::widget::Container<'a, Message> {
    container(iced::widget::Space::new(28.0, 28.0))
        .style(icon)
        .max_width(28.0)
        .max_height(28.0)
}

pub mod icon {
    use crate::core::game;
    use iced::widget::image;
    use iced::widget::svg;
    use iced::Theme;

    fn text(theme: &Theme, _status: svg::Status) -> svg::Style {
        svg::Style {
            color: Some(theme.palette().text),
        }
    }

    pub fn chevron_down<'a>() -> svg::Svg<'a> {
        let path = svg::Handle::from_path(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/img/icons/chevron-down.svg"
        ));

        svg(path).style(text)
    }

    pub fn chevron_up<'a>() -> svg::Svg<'a> {
        let path = svg::Handle::from_path(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/img/icons/chevron-up.svg"
        ));

        svg(path).style(text)
    }

    pub fn search<'a>() -> svg::Svg<'a> {
        let path = svg::Handle::from_path(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/img/icons/search.svg"
        ));

        svg(path).style(text)
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

pub fn victory(theme: &Theme) -> iced::widget::text::Style {
    iced::widget::text::Style {
        color: Some(theme.palette().success),
    }
}

pub fn defeat(theme: &Theme) -> iced::widget::text::Style {
    iced::widget::text::Style {
        color: Some(theme.palette().danger),
    }
}

pub fn text(theme: &Theme) -> iced::widget::text::Style {
    iced::widget::text::Style {
        color: Some(theme.palette().text.scale_alpha(0.8)),
    }
}

pub fn left_bar(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();
    container::Style {
        background: Some(Background::Color(palette.primary.strong.color)),
        border: border::width(0),
        text_color: Some(palette.primary.strong.text),
        ..Default::default()
    }
}

pub fn search_bar(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();

    container::Style {
        background: Some(Background::Color(palette.background.weakest.color)),
        border: border::width(0),
        text_color: Some(palette.background.weakest.text),
        ..Default::default()
    }
}

pub fn timeline(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();
    container::Style {
        background: Some(Background::Color(palette.background.strong.color)),
        border: border::width(0),
        text_color: Some(palette.background.strong.text),
        ..Default::default()
    }
}

pub fn summoner_icon(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();
    container::Style {
        background: Some(Background::Color(palette.background.weak.color)),
        border: border::rounded(2)
            .width(2)
            .color(palette.warning.weak.color),
        text_color: Some(palette.background.weak.text),
        ..Default::default()
    }
}

pub fn summoner_level(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();
    container::Style {
        background: Some(Background::Color(palette.background.base.color)),
        border: border::rounded(2)
            .width(1)
            .color(palette.warning.weak.color),
        text_color: Some(palette.background.base.text),
        ..Default::default()
    }
}

pub fn dark(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();

    container::Style {
        background: Some(Background::Color(palette.background.base.color)),
        border: border::rounded(4),
        text_color: Some(palette.background.base.text),
        ..Default::default()
    }
}

pub fn icon(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();
    container::Style {
        background: Some(Background::Color(palette.background.weak.color)),
        border: border::width(0),
        text_color: Some(palette.background.weak.text),
        ..Default::default()
    }
}

pub fn left_border(theme: &Theme, result: game::Result) -> container::Style {
    let palette = theme.extended_palette();
    container::Style {
        background: Some(Background::Color(win_color(theme, result))),
        border: border::rounded(border::left(4)),
        text_color: Some(palette.background.weak.text),
        ..Default::default()
    }
}

pub fn team_header(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();
    container::Style {
        background: Some(Background::Color(palette.background.base.color)),
        border: border::width(0),
        text_color: Some(palette.background.base.color),
        ..Default::default()
    }
}

pub fn team_player(theme: &Theme, is_player: bool) -> container::Style {
    let palette = theme.extended_palette();
    if is_player {
        container::Style {
            background: Some(Background::Color(palette.background.weak.color)),
            border: border::width(0),
            text_color: Some(palette.background.weak.text),
            ..Default::default()
        }
    } else {
        container::Style {
            background: Some(Background::Color(palette.background.base.color)),
            border: border::width(0),
            text_color: Some(palette.background.base.text),
            ..Default::default()
        }
    }
}

pub fn win_color(theme: &Theme, result: impl Into<game::Result>) -> Color {
    let palette = theme.extended_palette();
    match result.into() {
        game::Result::Remake => palette.background.weak.color,
        game::Result::Surrender | game::Result::Defeat => palette.danger.base.color,
        game::Result::Victory => palette.success.base.color,
    }
}

pub fn expander(theme: &Theme, status: button::Status, expanded: bool) -> button::Style {
    let palette = theme.extended_palette();
    let background = if matches!(status, button::Status::Hovered) {
        Some(Background::Color(palette.background.strongest.color))
    } else {
        Some(Background::Color(palette.background.weak.color))
    };

    if expanded {
        button::Style {
            background: Some(Background::Color(palette.background.strongest.color)),
            border: border::rounded(border::right(4)),
            text_color: palette.background.base.text,
            ..Default::default()
        }
    } else {
        button::Style {
            background,
            border: border::rounded(border::right(4).bottom(4)),
            text_color: palette.background.base.text,
            ..Default::default()
        }
    }
}

pub fn update(theme: &Theme, status: button::Status) -> button::Style {
    button::Style {
        border: border::rounded(2),
        ..button::primary(theme, status)
    }
}

pub fn region(theme: &Theme, status: pick_list::Status) -> pick_list::Style {
    let palette = theme.extended_palette();
    let background = if matches!(status, pick_list::Status::Hovered) {
        Background::Color(palette.background.strong.color)
    } else {
        Background::Color(palette.background.weakest.color)
    };

    pick_list::Style {
        background,
        border: border::width(0),
        ..pick_list::default(theme, status)
    }
}

pub fn region_menu(theme: &Theme) -> menu::Style {
    let palette = theme.extended_palette();
    menu::Style {
        background: Background::Color(palette.background.weakest.color),
        border: border::width(0),
        selected_background: Background::Color(palette.background.weak.color.scale_alpha(0.9)),
        selected_text_color: palette.background.base.text,
        ..menu::default(theme)
    }
}

pub fn expand(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();
    let background = if matches!(status, button::Status::Hovered) {
        Some(Background::Color(palette.background.weak.color))
    } else {
        Some(Background::Color(palette.background.weakest.color))
    };

    button::Style {
        background,
        border: border::rounded(2),
        text_color: palette.background.weak.text,
        ..Default::default()
    }
}

pub fn queue_picklist(
    selected: bool,
    theme: &Theme,
    status: pick_list::Status,
) -> pick_list::Style {
    let palette = theme.extended_palette();

    let background = if matches!(status, pick_list::Status::Hovered) {
        Background::Color(palette.background.strong.color)
    } else if selected {
        Background::Color(palette.background.strongest.color)
    } else {
        Background::Color(palette.background.weakest.color)
    };

    pick_list::Style {
        text_color: palette.background.weak.text,
        placeholder_color: palette.background.base.text.scale_alpha(0.7),
        background,
        border: border::rounded(2),
        ..pick_list::default(theme, status)
    }
}

pub fn queue_filter(theme: &Theme, status: button::Status, selected: bool) -> button::Style {
    let palette = theme.extended_palette();

    let color = if selected {
        palette.background.strongest.color
    } else {
        palette.background.weakest.color
    };

    let base = button::Style {
        background: Some(Background::Color(color)),
        border: border::rounded(4),
        text_color: palette.background.weakest.text,
        ..Default::default()
    };

    match status {
        button::Status::Active | button::Status::Pressed => base,
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(palette.background.strong.color)),
            ..base
        },
        button::Status::Disabled => button::Style {
            background: Some(Background::Color(
                palette.background.weakest.color.scale_alpha(0.5),
            )),
            text_color: palette.background.weakest.text.scale_alpha(0.5),
            ..base
        },
    }
}

pub fn ratio_bar(theme: &Theme) -> progress_bar::Style {
    let palette = theme.extended_palette();
    progress_bar::Style {
        background: Background::Color(palette.danger.base.color),
        bar: Background::Color(palette.success.base.color),
        border: border::rounded(0),
    }
}

pub fn fill_bar(theme: &Theme, color: Color) -> progress_bar::Style {
    let palette = theme.extended_palette();
    progress_bar::Style {
        background: Background::Color(palette.background.strong.color),
        bar: Background::Color(color),
        border: border::rounded(border::radius(0)),
    }
}

pub fn rule(theme: &Theme) -> widget::rule::Style {
    widget::rule::Style {
        color: theme.palette().text.scale_alpha(0.6),
        snap: true,
        width: 1,
        radius: 0.0.into(),
        fill_mode: widget::rule::FillMode::Full,
    }
}

pub fn scrollable(theme: &Theme, status: widget::scrollable::Status) -> widget::scrollable::Style {
    use widget::scrollable;
    let palette = theme.extended_palette();

    let scrollbar = scrollable::Rail {
        background: Some(Background::Color(palette.background.strongest.color)),
        border: border::rounded(2),
        scroller: scrollable::Scroller {
            color: palette.background.weak.color,
            border: border::rounded(2),
        },
    };

    match status {
        scrollable::Status::Active { .. } => scrollable::Style {
            container: container::Style::default(),
            vertical_rail: scrollbar,
            horizontal_rail: scrollbar,
            gap: None,
        },
        scrollable::Status::Hovered {
            is_horizontal_scrollbar_hovered: is_horizontal_scrollbar,
            is_vertical_scrollbar_hovered: is_vertical_scrollbar,
            ..
        }
        | scrollable::Status::Dragged {
            is_horizontal_scrollbar_dragged: is_horizontal_scrollbar,
            is_vertical_scrollbar_dragged: is_vertical_scrollbar,
            ..
        } => {
            let hovered = scrollable::Rail {
                scroller: scrollable::Scroller {
                    color: palette.background.weakest.color,
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

pub fn search(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();
    let background = if matches!(status, button::Status::Hovered) {
        Some(Background::Color(palette.background.strong.color))
    } else {
        Some(Background::Color(Color::TRANSPARENT))
    };

    button::Style {
        background,
        text_color: palette.background.base.text,
        ..Default::default()
    }
}

pub fn search_text_input(theme: &Theme, status: text_input::Status) -> text_input::Style {
    let palette = theme.extended_palette();
    let background = if matches!(status, text_input::Status::Hovered) {
        Background::Color(palette.background.strongest.color)
    } else {
        Background::Color(Color::TRANSPARENT)
    };

    let active = text_input::Style {
        background,
        border: iced::Border {
            radius: 0.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        icon: Color::TRANSPARENT,
        placeholder: palette.background.base.text,
        value: palette.background.base.text,
        selection: palette.background.base.color,
    };

    match status {
        text_input::Status::Hovered | text_input::Status::Active => active,
        text_input::Status::Focused { .. } => text_input::Style {
            background: Background::Color(palette.background.strongest.color),
            border: Border {
                color: palette.primary.strong.color,
                width: 2.0,
                ..active.border
            },
            ..active
        },
        text_input::Status::Disabled => widget::text_input::Style {
            background: Background::Color(palette.background.strong.color.scale_alpha(0.5)),
            value: palette.background.base.text.scale_alpha(0.5),
            ..active
        },
    }
}
