use crate::core;
use crate::text;
use crate::theme;
use crate::widget::bold;
use iced::alignment;
use iced::widget::column;
use iced::widget::{button, container, image, row, text, vertical_space};
use iced::Element;
use iced::Length;

#[derive(Debug, Clone)]
pub enum Message {
    Update,
    SummonerFetched(Result<core::Summoner, core::summoner::RequestError>),
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum Tier {
    Iron(Division),
    Bronze(Division),
    Silver(Division),
    Gold(Division),
    Platinum(Division),
    Diamond(Division),
    Master(u16),
    Grandmaster(u16),
    Challenger(u16),
}

impl Tier {
    pub fn points(&self) -> u16 {
        match self {
            Tier::Challenger(points) | Tier::Grandmaster(points) | Tier::Master(points) => *points,
            Tier::Iron(division)
            | Tier::Bronze(division)
            | Tier::Silver(division)
            | Tier::Gold(division)
            | Tier::Platinum(division)
            | Tier::Diamond(division) => match division {
                Division::One(points)
                | Division::Two(points)
                | Division::Three(points)
                | Division::Four(points) => *points as u16,
            },
        }
    }

    pub fn division(&self) -> String {
        match self {
            Tier::Iron(division)
            | Tier::Bronze(division)
            | Tier::Silver(division)
            | Tier::Gold(division)
            | Tier::Platinum(division)
            | Tier::Diamond(division) => division.to_string(),
            Tier::Master(points) | Tier::Grandmaster(points) | Tier::Challenger(points) => {
                points.to_string()
            }
        }
    }
}

impl ToString for Tier {
    fn to_string(&self) -> String {
        match self {
            Tier::Iron(_) => "Iron",
            Tier::Bronze(_) => "Bronze",
            Tier::Silver(_) => "Silver",
            Tier::Gold(_) => "Gold",
            Tier::Platinum(_) => "Platinum",
            Tier::Diamond(_) => "Diamond",
            Tier::Master(_) => "Master",
            Tier::Grandmaster(_) => "Grandmaster",
            Tier::Challenger(_) => "Challenger",
        }
        .to_string()
    }
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum Division {
    One(u8),
    Two(u8),
    Three(u8),
    Four(u8),
}

impl ToString for Division {
    fn to_string(&self) -> String {
        match self {
            Division::One(_) => "1",
            Division::Two(_) => "2",
            Division::Three(_) => "3",
            Division::Four(_) => "4",
        }
        .to_string()
    }
}

mod modal {
    use iced::advanced::layout::{self, Layout};
    use iced::advanced::overlay;
    use iced::advanced::renderer;
    use iced::advanced::widget::{self, Widget};
    use iced::advanced::{self, Clipboard, Shell};
    use iced::alignment::Alignment;
    use iced::mouse;
    use iced::{event, Vector};
    use iced::{Element, Event, Length, Point, Rectangle, Size};

    /// A widget that centers a modal element over some base element
    pub struct Modal<'a, Message, Theme, Renderer> {
        image: Element<'a, Message, Theme, Renderer>,
        level: Element<'a, Message, Theme, Renderer>,
    }

    impl<'a, Message, Theme, Renderer> Modal<'a, Message, Theme, Renderer> {
        /// Returns a new [`Modal`]
        pub fn new(
            image: impl Into<Element<'a, Message, Theme, Renderer>>,
            level: impl Into<Element<'a, Message, Theme, Renderer>>,
        ) -> Self {
            Self {
                image: image.into(),
                level: level.into(),
            }
        }
    }

    impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
        for Modal<'a, Message, Theme, Renderer>
    where
        Renderer: advanced::Renderer,
        Message: Clone,
    {
        fn children(&self) -> Vec<widget::Tree> {
            vec![
                widget::Tree::new(&self.image),
                widget::Tree::new(&self.level),
            ]
        }

        fn diff(&self, tree: &mut widget::Tree) {
            tree.diff_children(&[&self.image, &self.level]);
        }

        fn size(&self) -> Size<Length> {
            self.image.as_widget().size()
        }

        fn layout(
            &self,
            tree: &mut widget::Tree,
            renderer: &Renderer,
            limits: &layout::Limits,
        ) -> layout::Node {
            self.image
                .as_widget()
                .layout(&mut tree.children[0], renderer, limits)
        }

        fn on_event(
            &mut self,
            state: &mut widget::Tree,
            event: Event,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            renderer: &Renderer,
            clipboard: &mut dyn Clipboard,
            shell: &mut Shell<'_, Message>,
            viewport: &Rectangle,
        ) -> event::Status {
            self.image.as_widget_mut().on_event(
                &mut state.children[0],
                event,
                layout,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            )
        }

        fn draw(
            &self,
            state: &widget::Tree,
            renderer: &mut Renderer,
            theme: &Theme,
            style: &renderer::Style,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            viewport: &Rectangle,
        ) {
            self.image.as_widget().draw(
                &state.children[0],
                renderer,
                theme,
                style,
                layout,
                cursor,
                viewport,
            );
        }

        fn overlay<'b>(
            &'b mut self,
            state: &'b mut widget::Tree,
            layout: Layout<'_>,
            _renderer: &Renderer,
            translation: Vector,
        ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
            Some(overlay::Element::new(Box::new(Overlay {
                position: layout.position() + translation,
                content: &mut self.level,
                tree: &mut state.children[1],
                size: layout.bounds().size(),
            })))
        }

        fn mouse_interaction(
            &self,
            state: &widget::Tree,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            viewport: &Rectangle,
            renderer: &Renderer,
        ) -> mouse::Interaction {
            self.image.as_widget().mouse_interaction(
                &state.children[0],
                layout,
                cursor,
                viewport,
                renderer,
            )
        }

        fn operate(
            &self,
            state: &mut widget::Tree,
            layout: Layout<'_>,
            renderer: &Renderer,
            operation: &mut dyn widget::Operation<Message>,
        ) {
            self.image
                .as_widget()
                .operate(&mut state.children[0], layout, renderer, operation);
        }
    }

    struct Overlay<'a, 'b, Message, Theme, Renderer> {
        position: Point,
        content: &'b mut Element<'a, Message, Theme, Renderer>,
        tree: &'b mut widget::Tree,
        size: Size,
    }

    impl<'a, 'b, Message, Theme, Renderer> overlay::Overlay<Message, Theme, Renderer>
        for Overlay<'a, 'b, Message, Theme, Renderer>
    where
        Renderer: advanced::Renderer,
        Message: Clone,
    {
        fn layout(&mut self, renderer: &Renderer, _bounds: Size) -> layout::Node {
            let limits = layout::Limits::new(Size::ZERO, self.size)
                .width(Length::Fill)
                .height(Length::Fill);

            let child = self
                .content
                .as_widget()
                .layout(self.tree, renderer, &limits)
                .align(
                    Alignment::Center,
                    Alignment::End,
                    limits.max(),
                    // .pad([0.0, 0.0, child.size().height / 2.0, 0.0].into()),
                );

            layout::Node::with_children(self.size, vec![child]).move_to(self.position)
        }

        fn on_event(
            &mut self,
            event: Event,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            renderer: &Renderer,
            clipboard: &mut dyn Clipboard,
            shell: &mut Shell<'_, Message>,
        ) -> event::Status {
            self.content.as_widget_mut().on_event(
                self.tree,
                event,
                layout.children().next().unwrap(),
                cursor,
                renderer,
                clipboard,
                shell,
                &layout.bounds(),
            )
        }

        fn draw(
            &self,
            renderer: &mut Renderer,
            theme: &Theme,
            style: &renderer::Style,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
        ) {
            self.content.as_widget().draw(
                self.tree,
                renderer,
                theme,
                style,
                layout.children().next().unwrap(),
                cursor,
                &layout.bounds(),
            );
        }

        fn operate(
            &mut self,
            layout: Layout<'_>,
            renderer: &Renderer,
            operation: &mut dyn widget::Operation<Message>,
        ) {
            self.content.as_widget().operate(
                self.tree,
                layout.children().next().unwrap(),
                renderer,
                operation,
            );
        }

        fn mouse_interaction(
            &self,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            viewport: &Rectangle,
            renderer: &Renderer,
        ) -> mouse::Interaction {
            self.content.as_widget().mouse_interaction(
                self.tree,
                layout.children().next().unwrap(),
                cursor,
                viewport,
                renderer,
            )
        }

        fn overlay<'c>(
            &'c mut self,
            layout: Layout<'_>,
            renderer: &Renderer,
        ) -> Option<overlay::Element<'c, Message, Theme, Renderer>> {
            self.content.as_widget_mut().overlay(
                self.tree,
                layout.children().next().unwrap(),
                renderer,
                Vector::ZERO,
            )
        }
    }

    impl<'a, Message, Theme, Renderer> From<Modal<'a, Message, Theme, Renderer>>
        for Element<'a, Message, Theme, Renderer>
    where
        Theme: 'a,
        Message: 'a + Clone,
        Renderer: 'a + advanced::Renderer,
    {
        fn from(modal: Modal<'a, Message, Theme, Renderer>) -> Self {
            Element::new(modal)
        }
    }
}

fn summoner_icon<'a>(icon: Option<image::Handle>, level: u32) -> Element<'a, Message> {
    let image: Element<Message> = if let Some(handle) = icon {
        image(handle).into()
    } else {
        vertical_space(96.0).into()
    };

    modal::Modal::new(
        container(image)
            .width(96.0)
            .height(96.0)
            .center_x()
            .center_y()
            .padding(2.0)
            .style(theme::summoner_icon_container()),
        container(bold(level).size(10))
            .padding([1, 4, 2, 4]) // TODO: fix this alignment issue (text doesnt seem to get centered)
            .center_y()
            .style(theme::summoner_level_container()),
    )
    .into()
}

#[derive(Debug, Clone)]
pub enum Event {
    UpdateProfile(String),
}

pub struct Summoner {
    name: String,
    level: u32,
    icon: u32,
    icon_image: Option<image::Handle>,
}

impl Summoner {
    pub fn new(icon: u32) -> Self {
        Summoner {
            name: String::from("loading..."),
            level: 111,
            icon,
            icon_image: None,
        }
    }

    pub fn load_icon(&mut self) {
        let path = format!(
            "{}{}.png",
            concat!(env!("CARGO_MANIFEST_DIR"), "\\assets\\img\\profileicon\\"),
            self.icon
        );
        self.icon_image = Some(iced::widget::image::Handle::from_path(path));
    }

    pub fn update(&mut self, message: Message) -> Option<Event> {
        match message {
            Message::Update => Some(Event::UpdateProfile(self.name.clone())),
            Message::SummonerFetched(Ok(summoner)) => {
                self.name = summoner.name().to_string();
                self.level = summoner.level();
                self.icon = summoner.icon_id() as u32;
                self.load_icon();

                None
            }
            Message::SummonerFetched(Err(error)) => {
                tracing::error!("{error:?}");

                None
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let icon = summoner_icon(self.icon_image.clone(), self.level);
        let past_ranks = {
            let ranks = [
                (12, Tier::Iron(Division::One(0))),
                (11, Tier::Bronze(Division::Four(100))),
                (10, Tier::Silver(Division::One(0))),
                (9, Tier::Gold(Division::Four(100))),
                (8, Tier::Platinum(Division::One(0))),
                (7, Tier::Diamond(Division::Four(100))),
                (6, Tier::Master(150)),
                (5, Tier::Grandmaster(600)),
                (4, Tier::Challenger(2000)),
            ];

            row(ranks.into_iter().map(|(season, rank)| {
                let division = rank.division();
                let tier = rank.to_string();

                container(
                    row![
                        container(text(format!("S{season}")).size(10))
                            .padding(2)
                            .style(theme::past_rank_badge_container()),
                        container(
                            text!("{tier} {division}")
                                .size(10)
                                .style(theme::tier_color(rank))
                        )
                        .padding(2),
                    ]
                    .align_items(iced::Alignment::Center)
                    .spacing(2),
                )
                .style(theme::past_rank_container())
                .into()
            }))
            .spacing(2)
        };

        let name = text(&self.name)
            .size(24)
            .vertical_alignment(alignment::Vertical::Center);

        let rank = 241768;
        let ladder_rank = rank
            .to_string()
            .as_bytes()
            .rchunks(3)
            .rev()
            .map(std::str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap()
            .join(",");
        let rank_percentage = 24.7;
        let ladder = row![
            text("Ladder rank").size(12).style(theme::gray_text()),
            text!("{ladder_rank}").size(12),
            text!("(top {rank_percentage}%)")
                .size(12)
                .style(theme::gray_text()),
        ]
        .spacing(4);

        let update_button = button("Update")
            .style(theme::update_button())
            .on_press(Message::Update);

        container(
            column![
                past_ranks,
                row![
                    icon,
                    column![
                        column![name, ladder],
                        container(update_button)
                            .height(48)
                            .align_y(alignment::Vertical::Bottom)
                    ]
                    .spacing(1)
                ]
                .spacing(16)
            ]
            .spacing(8)
            .width(Length::Fill)
            .max_width(920)
            .padding([8, 0, 8, 0]),
        )
        .center_x()
        .width(Length::Fill)
        .style(theme::timeline_container())
        .into()
    }
}
