use crate::core::game;

use iced::color;
use iced::font;
use iced::theme;
use iced::widget;
use iced::Background;
use iced::Color;

pub const ROBOTO_FLEX_TTF: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "\\assets\\font\\RobotoFlex-Regular.ttf"
))
.as_slice();

pub const ROBOTO_NORMAL: iced::Font = iced::Font {
    family: font::Family::Name("RobotoFlex"),
    weight: font::Weight::Normal,
    stretch: font::Stretch::Normal,
    style: font::Style::Normal,
};

pub enum Container {
    Dark,
    Icon,
    LeftBorder(game::Result),
    Timeline,
    SummonerIcon,
    SummonerLevel,
    SearchBar,
    LeftBar,
    TeamPlayer,
    TeamMate,
    TeamHeader,
}

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
pub const RED_ALPHA: Color = color!(0xff5733, 0.7);

pub mod icon {
    use crate::core::game;
    use iced::widget::image;
    use iced::widget::svg;

    pub fn chevron_down() -> image::Handle {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "\\assets\\img\\icons\\chevron-down-white.png"
        );
        image::Handle::from_path(path)
    }

    pub fn chevron_up() -> image::Handle {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "\\assets\\img\\icons\\chevron-up-white.png"
        );
        image::Handle::from_path(path)
    }

    pub fn search() -> image::Handle {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "\\assets\\img\\icons\\search-white.png"
        );
        image::Handle::from_path(path)
    }

    pub fn clock() -> svg::Handle {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "\\assets\\img\\icons\\clock2.svg"
        );
        svg::Handle::from_path(path)
    }

    pub fn unranked() -> image::Handle {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "\\assets\\img\\emblems\\minicrests\\unranked.png"
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

        let path = format!(
            "{}\\assets\\img\\position\\{file}",
            env!("CARGO_MANIFEST_DIR"),
        );

        image::Handle::from_path(path)
    }
}

pub fn search_bar_text_input() -> theme::TextInput {
    theme::TextInput::Custom(Box::new(TextInput::SearchBar))
}

pub fn left_bar_container() -> theme::Container {
    theme::Container::Custom(Box::new(Container::LeftBar))
}

pub fn search_bar_container() -> theme::Container {
    theme::Container::Custom(Box::new(Container::SearchBar))
}

pub fn timeline_container() -> theme::Container {
    theme::Container::Custom(Box::new(Container::Timeline))
}

pub fn summoner_icon_container() -> theme::Container {
    theme::Container::Custom(Box::new(Container::SummonerIcon))
}

pub fn summoner_level_container() -> theme::Container {
    theme::Container::Custom(Box::new(Container::SummonerLevel))
}

pub fn dark_container() -> theme::Container {
    theme::Container::Custom(Box::new(Container::Dark))
}

pub fn icon_container() -> theme::Container {
    theme::Container::Custom(Box::new(Container::Icon))
}

pub fn left_border_container(result: game::Result) -> theme::Container {
    theme::Container::Custom(Box::new(Container::LeftBorder(result)))
}

pub fn team_header_container() -> theme::Container {
    theme::Container::Custom(Box::new(Container::TeamHeader))
}

pub fn team_player_container(is_player: bool) -> theme::Container {
    if is_player {
        theme::Container::Custom(Box::new(Container::TeamPlayer))
    } else {
        theme::Container::Custom(Box::new(Container::TeamMate))
    }
}

pub fn ratio_bar() -> theme::ProgressBar {
    theme::ProgressBar::Custom(Box::new(RatioBar))
}

pub fn fill_bar(fill_color: Color) -> theme::ProgressBar {
    theme::ProgressBar::Custom(Box::new(FillBar(fill_color)))
}

pub fn region_button() -> theme::Button {
    theme::Button::Custom(Box::new(Button::Region))
}

pub fn expand_button() -> theme::Button {
    theme::Button::Custom(Box::new(Button::Expand))
}

pub fn win_color(result: impl Into<game::Result>) -> Color {
    match result.into() {
        game::Result::Remake => SUB_TEXT,
        game::Result::Surrender | game::Result::Defeat => RED,
        game::Result::Victory => BLUE,
    }
}

pub fn update_button() -> theme::Button {
    theme::Button::Custom(Box::new(Button::Update))
}

pub fn scrollable() -> theme::Scrollable {
    theme::Scrollable::Custom(Box::new(Scrollable))
}

impl widget::container::StyleSheet for Container {
    type Style = iced::Theme;

    fn appearance(&self, _theme: &iced::Theme) -> widget::container::Appearance {
        let background = match self {
            Container::Timeline => Background::Color(DARKER_BACKGROUND),
            Container::Dark | Container::TeamHeader => Background::Color(DARK_BACKGROUND),
            Container::Icon => Background::Color(LIGHT_BACKGROUND),
            Container::LeftBorder(result) => Background::Color(win_color(*result)),
            Container::SummonerIcon => Background::Color(LIGHT_BACKGROUND), // todo: switch to image
            Container::SummonerLevel => Background::Color(DARK_BACKGROUND),
            Container::SearchBar => Background::Color(LIGHTER_BACKGROUND),
            Container::LeftBar => Background::Color(BLUE),
            Container::TeamPlayer => Background::Color(LIGHTER_ALPHA),
            Container::TeamMate => Background::Color(DARK_BACKGROUND),
        };

        let text_color = match self {
            Container::Dark
            | Container::LeftBorder(_)
            | Container::Timeline
            | Container::SummonerIcon
            | Container::SummonerLevel
            | Container::LeftBar
            | Container::TeamPlayer
            | Container::TeamMate
            | Container::TeamHeader => Color::WHITE,
            Container::Icon => Color::BLACK,
            Container::SearchBar => SUB_TEXT,
        };

        let border_radius = match self {
            Container::Dark => 4.0.into(),
            Container::SummonerLevel => 2.0.into(),
            Container::SummonerIcon => 2.0.into(),
            Container::Timeline | Container::Icon => 0.0.into(),
            Container::LeftBorder(_) => [4.0, 0.0, 0.0, 4.0].into(),
            Container::SearchBar
            | Container::LeftBar
            | Container::TeamPlayer
            | Container::TeamMate
            | Container::TeamHeader => 0.0.into(),
        };

        let border_color = match self {
            Container::Dark | Container::Timeline | Container::LeftBorder(_) | Container::Icon => {
                Color::TRANSPARENT
            }
            Container::SummonerIcon | Container::SummonerLevel => GOLD,
            Container::SearchBar => Color::TRANSPARENT,
            Container::LeftBar => Color::TRANSPARENT,
            Container::TeamPlayer | Container::TeamMate => Color::TRANSPARENT,
            Container::TeamHeader => Color::TRANSPARENT,
        };

        let border_width = match self {
            Container::Dark
            | Container::Timeline
            | Container::LeftBorder(_)
            | Container::Icon
            | Container::SearchBar => 0.0,
            Container::SummonerIcon => 2.0,
            Container::SummonerLevel => 1.0,
            Container::LeftBar => 0.0,
            Container::TeamPlayer | Container::TeamMate => 0.0,
            Container::TeamHeader => 0.0,
        };

        widget::container::Appearance {
            background: Some(background),
            text_color: Some(text_color),
            border: iced::Border {
                color: border_color,
                width: border_width,
                radius: border_radius,
            },
            ..Default::default()
        }
    }
}

pub fn expander_button(toggled: bool) -> theme::Button {
    theme::Button::custom(Button::Expander(toggled))
}

pub enum Button {
    Expander(bool),
    Update,
    Region,
    Expand,
}

impl widget::button::StyleSheet for Button {
    type Style = iced::Theme;

    fn active(&self, _theme: &iced::Theme) -> widget::button::Appearance {
        let background_color = match self {
            Button::Expander(false) => LIGHTER_ALPHA,
            Button::Expander(true) => Color::TRANSPARENT,
            Button::Update => BLUE,
            Button::Region => GOLD,
            Button::Expand => LIGHTER_BACKGROUND,
        };

        let border_radius = match self {
            Button::Expander(true) => [0.0, 4.0, 0.0, 0.0].into(),
            Button::Expander(false) => [0.0, 4.0, 4.0, 0.0].into(),
            Button::Update => 2.0.into(),
            Button::Region => 0.0.into(),
            Button::Expand => 2.0.into(),
        };

        let text_color = match self {
            Button::Region | Button::Expander(_) | Button::Update | Button::Expand => Color::WHITE,
        };

        widget::button::Appearance {
            background: Some(iced::Background::Color(background_color)),
            border: iced::Border {
                radius: border_radius,
                ..Default::default()
            },
            text_color,
            ..Default::default()
        }
    }

    fn hovered(&self, _theme: &iced::Theme) -> widget::button::Appearance {
        let background_color = match self {
            Button::Update => BLUE_ALPHA,
            Button::Expander(_) => LIGHT_ALPHA,
            Button::Region => RED_ALPHA,
            Button::Expand => LIGHT_ALPHA,
        };

        widget::button::Appearance {
            background: Some(iced::Background::Color(background_color)),
            ..self.active(_theme)
        }
    }
}

struct RatioBar;

impl widget::progress_bar::StyleSheet for RatioBar {
    type Style = iced::Theme;

    fn appearance(&self, _theme: &iced::Theme) -> widget::progress_bar::Appearance {
        widget::progress_bar::Appearance {
            background: Background::Color(RED),
            bar: Background::Color(BLUE),
            border_radius: 0.0.into(),
        }
    }
}

struct FillBar(Color);

impl widget::progress_bar::StyleSheet for FillBar {
    type Style = iced::Theme;

    fn appearance(&self, _theme: &iced::Theme) -> widget::progress_bar::Appearance {
        widget::progress_bar::Appearance {
            background: Background::Color(LIGHTER_BACKGROUND),
            bar: Background::Color(self.0),
            border_radius: 0.0.into(),
        }
    }
}

pub fn rule(color: Color) -> theme::Rule {
    theme::Rule::Custom(Box::new(Rule(color)))
}

struct Rule(Color);

impl widget::rule::StyleSheet for Rule {
    type Style = iced::Theme;

    fn appearance(&self, _theme: &iced::Theme) -> widget::rule::Appearance {
        widget::rule::Appearance {
            color: self.0,
            width: 1,
            radius: 0.0.into(),
            fill_mode: widget::rule::FillMode::Full,
        }
    }
}

struct Scrollable;

impl widget::scrollable::StyleSheet for Scrollable {
    type Style = iced::Theme;

    fn active(&self, _theme: &iced::Theme) -> widget::scrollable::Appearance {
        widget::scrollable::Appearance {
            container: widget::container::Appearance {
                background: None,
                border: iced::Border {
                    radius: 2.0.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                ..Default::default()
            },
            scrollbar: widget::scrollable::Scrollbar {
                background: None,
                border: iced::Border {
                    radius: 2.0.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                scroller: widget::scrollable::Scroller {
                    color: LIGHTER_ALPHA,
                    border: iced::Border {
                        radius: 2.0.into(),
                        width: 0.0,
                        color: Color::TRANSPARENT,
                    },
                },
            },
            gap: None,
        }
    }

    fn hovered(
        &self,
        style: &Self::Style,
        is_mouse_over_scrollbar: bool,
    ) -> widget::scrollable::Appearance {
        let active = self.active(style);

        if is_mouse_over_scrollbar {
            widget::scrollable::Appearance {
                scrollbar: widget::scrollable::Scrollbar {
                    background: Some(Background::Color(DARK_BACKGROUND)),
                    scroller: widget::scrollable::Scroller {
                        color: LIGHT_ALPHA,
                        ..active.scrollbar.scroller
                    },
                    ..active.scrollbar
                },
                ..active
            }
        } else {
            active
        }
    }
}

pub enum TextInput {
    SearchBar,
}

impl widget::text_input::StyleSheet for TextInput {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> widget::text_input::Appearance {
        widget::text_input::Appearance {
            background: Background::Color(Color::TRANSPARENT),
            border: iced::Border {
                radius: 0.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            icon_color: Color::TRANSPARENT,
        }
    }

    fn hovered(&self, style: &Self::Style) -> widget::text_input::Appearance {
        widget::text_input::Appearance {
            background: Background::Color(LIGHTER_ALPHA),
            ..self.active(style)
        }
    }

    fn focused(&self, style: &Self::Style) -> widget::text_input::Appearance {
        self.hovered(style)
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        SUB_TEXT
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        Color::WHITE
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        BLUE
    }

    fn disabled(&self, style: &Self::Style) -> widget::text_input::Appearance {
        widget::text_input::Appearance {
            background: Background::Color(DARKER_BACKGROUND),
            ..self.active(style)
        }
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        SUB_TEXT
    }
}
