mod assets;
mod formatting;
mod screen;
mod theme;
mod widget;

use assets::Assets;
use screen::{profile, search_bar};

use iced::widget::{column, container, horizontal_space, row, text};
use iced::{font, Alignment, Element, Length, Task, Theme};

use aery_core as core;
use tracing_subscriber::EnvFilter;

pub fn main() -> iced::Result {
    let env_filter = EnvFilter::builder()
        .with_default_directive(tracing::Level::INFO.into())
        .from_env()
        .unwrap_or_default()
        .add_directive("aery_desktop=trace".parse().unwrap_or_default())
        .add_directive("wgpu=warn".parse().unwrap_or_default());

    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    iced::application(Aery::new, Aery::update, Aery::view)
        .theme(Aery::theme)
        .title("Aery")
        .antialiasing(true)
        .default_font(theme::DEFAULT_FONT)
        .window_size([1024.0, 768.0])
        .run()
}

enum Screen {
    Landing(screen::SearchBar),
    Profile(screen::Profile),
}

#[allow(clippy::large_enum_variant)]
enum Aery {
    Loading,
    Loaded { screen: Screen, assets: Assets },
}

#[derive(Debug, Clone)]
enum Message {
    AssetsLoaded(Result<Assets, assets::Error>),
    FontLoaded(Result<(), font::Error>),
    ProfileLoaded(Result<profile::Data, profile::Error>),

    Profile(profile::Message),
    Landing(search_bar::Message),
}

impl Aery {
    fn new() -> (Self, Task<Message>) {
        (Self::Loading, Assets::load())
    }

    fn with_assets(assets: Assets) -> Self {
        Self::Loaded {
            screen: Screen::Landing(screen::SearchBar::new()),
            assets,
        }
    }

    fn theme(&self) -> Theme {
        match self {
            Self::Loading
            | Self::Loaded {
                screen: Screen::Landing(_),
                ..
            } => Theme::Moonfly,
            Self::Loaded {
                screen: Screen::Profile(profile),
                ..
            } => profile.theme(),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::AssetsLoaded(Ok(assets)) => {
                tracing::info!("assets loaded!");
                *self = Self::with_assets(assets);
                Task::none()
            }
            Message::AssetsLoaded(Err(error)) => panic!("assets load failed: {error:?}"),
            Message::FontLoaded(Ok(_)) => {
                tracing::info!("font loaded!");

                Task::none()
            }
            Message::FontLoaded(Err(error)) => panic!("font load failed: {error:?}"),
            Message::ProfileLoaded(Ok(profile)) => {
                let Self::Loaded { screen, assets } = self else {
                    return Task::none();
                };

                *screen = Screen::Profile(screen::Profile::from_profile(assets, profile));

                Task::none()
            }
            Message::ProfileLoaded(Err(error)) => panic!("profile load failed: {error:?}"),
            Message::Profile(message) => {
                let Self::Loaded { screen, assets } = self else {
                    return Task::none();
                };

                if let Screen::Profile(profile) = screen {
                    return profile.update(message, assets).map(Message::Profile);
                }

                Task::none()
            }
            Message::Landing(message) => {
                let Self::Loaded { screen, .. } = self else {
                    return Task::none();
                };

                if let Screen::Landing(search_bar) = screen {
                    if let Some(search_bar::Event::SearchRequested { riot_id, region }) =
                        search_bar.update(message)
                    {
                        return Task::perform(
                            profile::fetch_data(riot_id, region),
                            Message::ProfileLoaded,
                        );
                    }
                }

                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match self {
            Self::Loading => loading(),
            Self::Loaded { screen, .. } => match screen {
                Screen::Profile(profile) => profile.view().map(Message::Profile),
                Screen::Landing(search_bar) => container(
                    column![
                        text("Aery").size(48),
                        row![
                            horizontal_space().width(Length::FillPortion(2)),
                            search_bar.view().map(Message::Landing),
                            horizontal_space().width(Length::FillPortion(2)),
                        ]
                        .align_y(Alignment::Center),
                    ]
                    .spacing(8)
                    .align_x(Alignment::Center),
                )
                .center(Length::Fill)
                .style(theme::timeline)
                .into(),
            },
        }
    }
}

fn loading<'a>() -> Element<'a, Message> {
    container(text!("Loading").size(24))
        .style(theme::timeline)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}
