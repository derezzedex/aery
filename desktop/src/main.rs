mod assets;
mod formatting;
mod screen;
mod theme;
mod widget;

use screen::profile;

use iced::{
    font,
    widget::{container, text},
    Element, Length, Task,
};

use assets::Assets;

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

    iced::application("Aery", Aery::update, Aery::view)
        .antialiasing(true)
        .default_font(theme::DEFAULT_FONT)
        .window_size([1024.0, 768.0])
        .run_with(Aery::new)
}

enum Screen {
    Profile(screen::Profile),
}

#[allow(clippy::large_enum_variant)]
enum Aery {
    Loading,
    Loaded { screen: Screen, assets: Assets },
}

#[derive(Debug, Clone)]
enum Message {
    LoadedAssets(Assets),
    FontLoaded(Result<(), font::Error>),

    Profile(profile::Message),
}

impl Aery {
    fn new() -> (Self, Task<Message>) {
        (
            Self::Loading,
            Task::batch(vec![
                Task::perform(Assets::new(), Message::LoadedAssets),
                font::load(theme::ROBOTO_FLEX_TTF).map(Message::FontLoaded),
            ]),
        )
    }

    fn with_assets(assets: Assets) -> Self {
        Self::Loaded {
            screen: Screen::Profile(screen::Profile::dummy(&assets)),
            assets,
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::LoadedAssets(assets) => {
                *self = Self::with_assets(assets);
                Task::none()
            }
            Message::FontLoaded(Err(err)) => panic!("font load failed: {err:?}"),
            Message::FontLoaded(Ok(_)) => Task::none(),
            Message::Profile(message) => {
                let Self::Loaded { screen, assets } = self else {
                    return Task::none();
                };

                let Screen::Profile(profile) = screen;

                profile.update(message, assets).map(Message::Profile)
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match self {
            Self::Loading => container(text!("Loading").size(24))
                .style(theme::timeline)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into(),
            Self::Loaded { screen, .. } => match screen {
                Screen::Profile(profile) => profile.view().map(Message::Profile),
            },
        }
    }
}
