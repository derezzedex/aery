use crate::core;
use crate::profile;
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
        vertical_space().height(96).into()
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
    summoner_name: String,
    riot_id: Option<core::RiotId>,
    level: u32,
    icon: u32,
    icon_image: Option<image::Handle>,
}

impl Summoner {
    pub fn from_profile(profile: &profile::Data) -> Self {
        let riot_id = profile
            .games
            .first()
            .map(|game| game.participant(profile.summoner.puuid()).unwrap().riot_id);
        let summoner_name = profile.summoner.name().to_string();
        let level = profile.summoner.level();
        let icon = profile.summoner.icon_id() as u32;
        let path = format!(
            "{}{}.png",
            concat!(env!("CARGO_MANIFEST_DIR"), "\\assets\\img\\profileicon\\"),
            icon
        );
        let icon_image = Some(iced::widget::image::Handle::from_path(path));

        Self {
            summoner_name,
            riot_id,
            level,
            icon,
            icon_image,
        }
    }

    pub fn new(icon: u32) -> Self {
        Summoner {
            summoner_name: String::from("Summoner"),
            riot_id: None,
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
            Message::Update => Some(Event::UpdateProfile(self.summoner_name.clone())),
            Message::SummonerFetched(Ok(summoner)) => {
                self.summoner_name = summoner.name().to_string();
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

        let (name, previously): (Element<_>, Option<Element<_>>) = match &self.riot_id {
            Some(riot_id) => match &riot_id.name {
                Some(riot_name) => (
                    row![
                        text(riot_name).size(24),
                        text(format!("#{}", riot_id.tagline))
                            .size(24)
                            .style(theme::sub_text())
                    ]
                    .spacing(8)
                    .align_items(iced::Alignment::Center)
                    .into(),
                    Some(
                        text(format!("Prev. {}", &self.summoner_name))
                            .style(theme::sub_text())
                            .size(12)
                            .into(),
                    ),
                ),
                None => (
                    row![
                        text(&self.summoner_name).size(24),
                        text(format!("#{}", riot_id.tagline))
                            .size(20)
                            .style(theme::gray_text())
                    ]
                    .spacing(2)
                    .align_items(iced::Alignment::Center)
                    .into(),
                    None,
                ),
            },
            None => (text(&self.summoner_name).size(24).into(), None),
        };

        let update_button = button("Update")
            .style(theme::update_button())
            .on_press(Message::Update);

        let mut inner = column![name];

        if let Some(text) = previously {
            inner = inner.push(text);
        }

        inner = inner.push(
            container(update_button)
                .height(48)
                .align_y(alignment::Vertical::Bottom),
        );

        // TODO: display ladder rank and past season ranks

        container(
            column![row![icon, inner.spacing(1)].spacing(16)]
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
