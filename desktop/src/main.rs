mod assets;
mod formatting;
mod screen;
mod theme;
mod widget;

use screen::profile;

use iced::{widget::container, Application, Command, Element, Length, Settings};

use assets::Assets;

use aery_core as core;

pub fn main() -> iced::Result {
    Aery::run(Settings {
        antialiasing: true,
        window: iced::window::Settings {
            min_size: Some([1024, 768].into()),
            ..Default::default()
        },
        ..Default::default()
    })
}

enum Screen {
    Profile(screen::Profile),
}

enum Aery {
    Loading,
    Loaded {
        client: core::Client,
        screen: Screen,
        assets: Assets,
    },
}

impl Aery {
    fn with_assets(assets: Assets) -> Self {
        let api_key =
            dotenv::var("RGAPI_KEY").expect("Unable to find `RGAPI_KEY` environment variable");

        let client = core::Client::new(api_key);

        Self::Loaded {
            client,
            screen: Screen::Profile(screen::Profile::dummy(&assets)),
            assets,
        }
    }
}
#[derive(Debug, Clone)]
enum Message {
    LoadedAssets(Assets),

    Profile(profile::Message),
}

impl Application for Aery {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self::Loading,
            Command::perform(Assets::new(), Message::LoadedAssets),
        )
    }

    fn title(&self) -> String {
        String::from("Aery")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::LoadedAssets(assets) => {
                *self = Self::with_assets(assets);
                Command::none()
            }
            Message::Profile(message) => {
                let Self::Loaded {
                    client,
                    screen,
                    assets,
                } = self
                else {
                    return Command::none();
                };

                let Screen::Profile(profile) = screen;

                let command = profile.update(message, client, assets);
                command.map(Message::Profile)
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match self {
            Self::Loading => container(text!("Loading").size(24))
                .style(theme::timeline_container())
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .into(),
            Self::Loaded { screen, .. } => match screen {
                Screen::Profile(profile) => profile.view().map(Message::Profile),
            },
        }
    }
}
