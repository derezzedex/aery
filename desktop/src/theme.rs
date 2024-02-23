use crate::core;

use iced::advanced::svg;
use iced::theme;
use iced::widget;
use iced::widget::image;
use iced::Background;
use iced::Color;

pub enum Container {
    Dark,
    Icon,
    LeftBorder(core::GameResult),
    Timeline,
    SummonerIcon,
    SummonerLevel,
    SearchBar,
    LeftBar,
    TeamPlayer(core::GameResult, bool),
    TeamHeader,
}

pub const DARKER_BACKGROUND: Color = Color::from_rgb(0.05, 0.05, 0.05);
pub const DARK_BACKGROUND: Color = Color::from_rgb(0.1, 0.1, 0.1);
pub const LIGHTER_BACKGROUND: Color = Color::from_rgb(0.2, 0.2, 0.2);
pub const LIGHT_BACKGROUND: Color = Color::from_rgb(0.95, 0.95, 0.95);

pub const RED: Color = Color::from_rgb(1.0, 0.34, 0.2);
pub const BLUE: Color = Color::from_rgb(0.0, 0.58, 1.0);
pub const GOLD: Color = Color::from_rgb(205.0 / 255.0, 136.0 / 255.0, 55.0 / 255.0);

pub const RED_DARK: Color = Color::from_rgb(0.349, 0.204, 0.231);
pub const RED_DARK_HIGHLIGHT: Color = Color::from_rgb(0.439, 0.235, 0.278);

pub const BLUE_DARK: Color = Color::from_rgb(0.094, 0.224, 0.333);
pub const BLUE_DARK_HIGHLIGHT: Color = Color::from_rgb(0.067, 0.282, 0.51);

pub fn chevron_down_icon() -> image::Handle {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "\\assets\\img\\icons\\chevron-down-white.png"
    );
    image::Handle::from_path(path)
}

pub fn chevron_up_icon() -> image::Handle {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "\\assets\\img\\icons\\chevron-up-white.png"
    );
    image::Handle::from_path(path)
}

pub fn search_icon() -> image::Handle {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "\\assets\\img\\icons\\search-white.png"
    );
    image::Handle::from_path(path)
}

pub fn clock_icon() -> svg::Handle {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "\\assets\\img\\icons\\clock2.svg"
    );
    svg::Handle::from_path(path)
}

pub fn unranked_icon() -> image::Handle {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "\\assets\\img\\emblems\\minicrests\\unranked.png"
    );
    image::Handle::from_path(path)
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

pub fn left_border_container(result: core::GameResult) -> theme::Container {
    theme::Container::Custom(Box::new(Container::LeftBorder(result)))
}

pub fn team_header_container() -> theme::Container {
    theme::Container::Custom(Box::new(Container::TeamHeader))
}

pub fn team_player_container(result: core::GameResult, is_player: bool) -> theme::Container {
    theme::Container::Custom(Box::new(Container::TeamPlayer(result, is_player)))
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

pub fn win_color(result: impl Into<core::GameResult>) -> Color {
    match result.into() {
        core::GameResult::Remake => Color::from_rgb(0.8, 0.8, 0.8),
        core::GameResult::Surrender | core::GameResult::Defeat => RED,
        core::GameResult::Victory => BLUE,
    }
}

pub fn gray_text() -> Color {
    Color::from_rgb(0.5, 0.5, 0.5)
}

pub fn red_text() -> Color {
    RED
}

pub fn sub_text() -> Color {
    Color::from_rgb(0.8, 0.8, 0.8)
}

pub fn blue_text() -> Color {
    BLUE
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
            Container::TeamPlayer(core::GameResult::Defeat, false)
            | Container::TeamPlayer(core::GameResult::Surrender, false) => {
                Background::Color(RED_DARK)
            }
            Container::TeamPlayer(core::GameResult::Defeat, true)
            | Container::TeamPlayer(core::GameResult::Surrender, true) => {
                Background::Color(RED_DARK_HIGHLIGHT)
            }
            Container::TeamPlayer(core::GameResult::Victory, false) => Background::Color(BLUE_DARK),
            Container::TeamPlayer(core::GameResult::Victory, true) => {
                Background::Color(BLUE_DARK_HIGHLIGHT)
            }
            Container::TeamPlayer(core::GameResult::Remake, false) => {
                Background::Color(gray_text())
            }
            Container::TeamPlayer(core::GameResult::Remake, true) => Background::Color(sub_text()),
        };

        let text_color = match self {
            Container::Dark
            | Container::LeftBorder(_)
            | Container::Timeline
            | Container::SummonerIcon
            | Container::SummonerLevel
            | Container::LeftBar
            | Container::TeamPlayer(_, _)
            | Container::TeamHeader => Color::WHITE,
            Container::Icon => Color::BLACK,
            Container::SearchBar => sub_text(),
        };

        let border_radius = match self {
            Container::Dark => 4.0.into(),
            Container::SummonerLevel => 2.0.into(),
            Container::SummonerIcon => 2.0.into(),
            Container::Timeline | Container::Icon => 0.0.into(),
            Container::LeftBorder(_) => [4.0, 0.0, 0.0, 4.0].into(),
            Container::SearchBar
            | Container::LeftBar
            | Container::TeamPlayer(_, _)
            | Container::TeamHeader => 0.0.into(),
        };

        let border_color = match self {
            Container::Dark | Container::Timeline | Container::LeftBorder(_) | Container::Icon => {
                Color::TRANSPARENT
            }
            Container::SummonerIcon | Container::SummonerLevel => GOLD,
            Container::SearchBar => Color::TRANSPARENT,
            Container::LeftBar => Color::TRANSPARENT,
            Container::TeamPlayer(_, _) => Color::TRANSPARENT,
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
            Container::TeamPlayer(_, _) => 0.0,
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
            Button::Expander(false) => Color::from_rgba(0.2, 0.2, 0.2, 0.6),
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
            Button::Update => Color::from_rgba(0.0, 0.58, 1.0, 0.7),
            Button::Expander(_) => Color::from_rgba(0.4, 0.4, 0.4, 0.9),
            Button::Region => Color::from_rgba(205.0 / 255.0, 136.0 / 255.0, 55.0 / 255.0, 0.7),
            Button::Expand => Color::from_rgba(0.4, 0.4, 0.4, 0.9),
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
                    color: Color::from_rgb(0.2, 0.2, 0.2),
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
                    background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.1))),
                    scroller: widget::scrollable::Scroller {
                        color: Color::from_rgba(0.4, 0.4, 0.4, 0.9),
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
            background: Background::Color(Color::from_rgba(0.2, 0.2, 0.2, 0.7)),
            ..self.active(style)
        }
    }

    fn focused(&self, style: &Self::Style) -> widget::text_input::Appearance {
        self.hovered(style)
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        sub_text()
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
        sub_text()
    }
}
