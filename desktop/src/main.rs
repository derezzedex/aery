use iced::{Application, Command, Element, Settings};

pub fn main() -> iced::Result {
    Aery::run(Settings::default())
}

struct Aery {
    game: widget::Game,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Game(widget::Message),
}

impl Application for Aery {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                game: widget::Game::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Aery")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Game(message) => self.game.update(message),
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        self.game.view().map(Message::Game)
    }
}

mod widget {
    use crate::theme;
    use iced::widget::{button, column, container, row, text, Space};
    use iced::{alignment, Alignment, Element, Length};

    #[derive(Default, Debug, Clone, Copy)]
    pub struct Game {
        is_expanded: bool,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Message {
        ExpandPressed,
    }

    impl Game {
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

                column![
                    column![
                        text("Victory").style(theme::blue_text()).size(16),
                        text("Ranked Flex").size(12),
                        text("a day ago").style(theme::sub_text()).size(10),
                    ],
                    column![
                        row![
                            very_small_icon(),
                            text("Mid").style(theme::sub_text()).size(10),
                        ]
                        .align_items(Alignment::Center)
                        .spacing(2),
                        text("28:33").size(10).style(theme::sub_text()),
                    ]
                    .padding([4, 0, 0, 0])
                ]
                .align_items(Alignment::Start)
                .spacing(2)
                .padding(4)
            };

            let champion_info = {
                let champion_icon = large_icon();

                let champion_spells = row![medium_icon(), medium_icon(),].spacing(2);

                let champion_runes = row![medium_icon(), medium_icon(),].spacing(2);

                row![
                    champion_icon,
                    column![champion_spells, champion_runes].spacing(2),
                ]
                .spacing(2)
            };

            let player_stats = {
                let kda = row![
                    text("1").size(12),
                    text("/").style(theme::gray_text()).size(12),
                    text("6").style(theme::red_text()).size(12),
                    text("/").style(theme::gray_text()).size(12),
                    text("12").size(12)
                ]
                .align_items(Alignment::Center)
                .spacing(3);

                // TODO: add rank score icon and justify content
                let other_stats = column![
                    row![
                        very_small_icon(),
                        text("2.17 KDA").size(10).style(theme::sub_text())
                    ]
                    .spacing(4)
                    .align_items(Alignment::Center),
                    row![
                        very_small_icon(),
                        text("203 CS (5.3)").size(10).style(theme::sub_text())
                    ]
                    .spacing(4)
                    .align_items(Alignment::Center),
                    row![
                        very_small_icon(),
                        text("17 vision").size(10).style(theme::sub_text())
                    ]
                    .spacing(4)
                    .align_items(Alignment::Center),
                ]
                .align_items(Alignment::Start);

                column![kda, other_stats,].align_items(Alignment::Center)
            };

            let player_items = {
                row![
                    column![medium_large_icon(), medium_large_icon()].spacing(2),
                    column![medium_large_icon(), medium_large_icon()].spacing(2),
                    column![medium_large_icon(), medium_large_icon()].spacing(2),
                    medium_large_icon(),
                ]
                .spacing(2)
            };

            let other_players = {
                row![
                    column![
                        row![small_icon(), small_text("Summoner1")]
                            .align_items(Alignment::Center)
                            .spacing(4),
                        row![small_icon(), small_text("Summoner4")]
                            .align_items(Alignment::Center)
                            .spacing(4),
                        row![small_icon(), small_text("Summoner3")]
                            .align_items(Alignment::Center)
                            .spacing(4),
                        row![small_icon(), small_text("Summoner4")]
                            .align_items(Alignment::Center)
                            .spacing(4),
                        row![small_icon(), small_text("Summoner5")]
                            .align_items(Alignment::Center)
                            .spacing(4)
                    ]
                    .spacing(2),
                    column![
                        row![small_icon(), small_text("Summoner6")]
                            .align_items(Alignment::Center)
                            .spacing(4),
                        row![small_icon(), small_text("Summoner7")]
                            .align_items(Alignment::Center)
                            .spacing(4),
                        row![small_icon(), small_text("Summoner8")]
                            .align_items(Alignment::Center)
                            .spacing(4),
                        row![small_icon(), small_text("Summoner9")]
                            .align_items(Alignment::Center)
                            .spacing(4),
                        row![small_icon(), small_text("Summoner10")]
                            .align_items(Alignment::Center)
                            .spacing(4)
                    ]
                    .spacing(2),
                ]
                .spacing(8)
            };

            let expand_content = container(small_icon())
                .center_x()
                .align_y(alignment::Vertical::Bottom)
                .height(Length::Fill)
                .width(24)
                .padding(2);

            let expand_button = button(expand_content)
                .height(Length::Fill)
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
                .spacing(32)
                .padding(4)
                .align_items(Alignment::Center),
                expand_button.padding(0),
            ])
            .max_height(100.0);

            let game = if self.is_expanded {
                let match_details = container(Space::new(0.0, 400.0));

                row![left_border(), column![overview, match_details,]]
            } else {
                row![left_border().max_height(100.0), column![overview]]
            };

            let content = container(game).style(theme::dark_container());

            container(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .align_y(alignment::Vertical::Top)
                .padding(16)
                .into()
        }
    }

    fn left_border<'a, Message: 'a>() -> iced::widget::Container<'a, Message> {
        container(Space::new(6.0, 0.0))
            .style(theme::left_border_container())
            .height(Length::Fill)
    }

    fn small_text<'a>(text: impl ToString) -> iced::widget::Text<'a> {
        iced::widget::Text::new(text.to_string())
            .style(theme::sub_text())
            .size(8.0)
    }

    fn very_small_icon<'a, Message: 'a>() -> iced::widget::Container<'a, Message> {
        container(Space::new(8.0, 8.0))
            .style(theme::icon_container())
            .max_width(8.0)
            .max_height(8.0)
    }

    fn small_icon<'a, Message: 'a>() -> iced::widget::Container<'a, Message> {
        container(Space::new(10.0, 10.0))
            .style(theme::icon_container())
            .max_width(10.0)
            .max_height(10.0)
    }

    fn medium_icon<'a, Message: 'a>() -> iced::widget::Container<'a, Message> {
        container(Space::new(12.0, 12.0))
            .style(theme::icon_container())
            .max_width(12.0)
            .max_height(12.0)
    }

    fn medium_large_icon<'a, Message: 'a>() -> iced::widget::Container<'a, Message> {
        container(Space::new(18.0, 18.0))
            .style(theme::icon_container())
            .max_width(18.0)
            .max_height(18.0)
    }

    fn large_icon<'a, Message: 'a>() -> iced::widget::Container<'a, Message> {
        container(Space::new(48.0, 48.0))
            .style(theme::icon_container())
            .max_width(48.0)
            .max_height(48.0)
    }
}

mod theme {
    use iced::theme;
    use iced::widget;
    use iced::Background;
    use iced::Color;

    pub enum Container {
        Dark,
        Icon,
        LeftBorder,
    }

    pub const DARK_BACKGROUND: Color = Color::from_rgb(0.1, 0.1, 0.1);
    pub const LIGHT_BACKGROUND: Color = Color::from_rgb(0.95, 0.95, 0.95);

    pub fn dark_container() -> theme::Container {
        theme::Container::Custom(Box::new(Container::Dark))
    }

    pub fn icon_container() -> theme::Container {
        theme::Container::Custom(Box::new(Container::Icon))
    }

    pub fn left_border_container() -> theme::Container {
        theme::Container::Custom(Box::new(Container::LeftBorder))
    }

    pub fn gray_text() -> Color {
        Color::from_rgb(0.5, 0.5, 0.5)
    }

    pub fn red_text() -> Color {
        Color::from_rgb(1.0, 0.34, 0.2)
    }

    pub fn sub_text() -> Color {
        Color::from_rgb(0.8, 0.8, 0.8)
    }

    pub fn blue_text() -> Color {
        Color::from_rgb(0.0, 0.58, 1.0)
    }

    impl widget::container::StyleSheet for Container {
        type Style = iced::Theme;

        fn appearance(&self, _theme: &iced::Theme) -> widget::container::Appearance {
            let background_color = match self {
                Container::Dark => DARK_BACKGROUND,
                Container::Icon => LIGHT_BACKGROUND,
                Container::LeftBorder => Color::from_rgb(0.0, 0.58, 1.0),
            };

            let text_color = match self {
                Container::Dark | Container::LeftBorder => Color::WHITE,
                Container::Icon => Color::BLACK,
            };

            let border_radius = match self {
                Container::Dark => 4.0.into(),
                Container::Icon => 0.0.into(),
                Container::LeftBorder => [4.0, 0.0, 0.0, 4.0].into(),
            };

            widget::container::Appearance {
                background: Some(Background::Color(background_color)),
                text_color: Some(text_color),
                border_radius,
                ..Default::default()
            }
        }
    }

    pub fn expander_button(toggled: bool) -> theme::Button {
        theme::Button::custom(Button::Expander(toggled))
    }

    pub enum Button {
        Expander(bool),
    }

    impl widget::button::StyleSheet for Button {
        type Style = iced::Theme;

        fn active(&self, _theme: &iced::Theme) -> widget::button::Appearance {
            let background_color = match self {
                Button::Expander(false) => Color::from_rgba(0.2, 0.2, 0.2, 0.6),
                Button::Expander(true) => Color::TRANSPARENT,
            };

            let border_radius = match self {
                Button::Expander(true) => [0.0, 4.0, 0.0, 0.0].into(),
                Button::Expander(false) => [0.0, 4.0, 4.0, 0.0].into(),
            };

            widget::button::Appearance {
                background: Some(iced::Background::Color(background_color)),
                border_radius,
                ..Default::default()
            }
        }

        fn hovered(&self, _theme: &iced::Theme) -> widget::button::Appearance {
            let background_color = Color::from_rgba(0.4, 0.4, 0.4, 0.9);

            widget::button::Appearance {
                background: Some(iced::Background::Color(background_color)),
                ..self.active(_theme)
            }
        }
    }
}
