//! Build and show dropdown menus.
use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::tree::{self, Tree};
use iced::advanced::{Clipboard, Shell, Widget};
use iced::border::{self, Border};
use iced::mouse;
use iced::overlay;
use iced::touch;
use iced::widget::scrollable::{self, Scrollable};
use iced::window;
use iced::{Background, Color, Element, Event, Length, Point, Rectangle, Size, Theme, Vector};

/// A list of selectable options.
#[allow(missing_debug_implementations)]
pub struct Menu<'a, 'b, T, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Theme: Catalog,
    Renderer: iced::advanced::Renderer,
    'b: 'a,
{
    state: &'a mut State,
    options: &'a [T],
    contents: &'a [Element<'a, Message, Theme, Renderer>],
    hovered_option: &'a mut Option<usize>,
    on_selected: Box<dyn FnMut(T) -> Message + 'a>,
    on_option_hovered: Option<&'a dyn Fn(T) -> Message>,
    width: f32,
    class: &'a <Theme as Catalog>::Class<'b>,
}

impl<'a, 'b, T, Message, Theme, Renderer> Menu<'a, 'b, T, Message, Theme, Renderer>
where
    T: Clone,
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: iced::advanced::Renderer + 'a,
    'b: 'a,
{
    /// Creates a new [`Menu`] with the given [`State`], a list of options,
    /// the message to produced when an option is selected, and its [`Style`].
    pub fn new(
        state: &'a mut State,
        options: &'a [T],
        contents: &'a [Element<'a, Message, Theme, Renderer>],
        hovered_option: &'a mut Option<usize>,
        on_selected: impl FnMut(T) -> Message + 'a,
        on_option_hovered: Option<&'a dyn Fn(T) -> Message>,
        class: &'a <Theme as Catalog>::Class<'b>,
    ) -> Self {
        Menu {
            state,
            options,
            contents,
            hovered_option,
            on_selected: Box::new(on_selected),
            on_option_hovered,
            width: 0.0,
            class,
        }
    }

    /// Sets the width of the [`Menu`].
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Turns the [`Menu`] into an overlay [`Element`] at the given target
    /// position.
    ///
    /// The `target_height` will be used to display the menu either on top
    /// of the target or under it, depending on the screen position and the
    /// dimensions of the [`Menu`].
    pub fn overlay(
        self,
        position: Point,
        viewport: Rectangle,
        target_height: f32,
    ) -> overlay::Element<'a, Message, Theme, Renderer> {
        overlay::Element::new(Box::new(Overlay::new(
            position,
            viewport,
            self,
            target_height,
        )))
    }
}

/// The local state of a [`Menu`].
#[derive(Debug)]
pub struct State {
    tree: Tree,
}

impl State {
    /// Creates a new [`State`] for a [`Menu`].
    pub fn new() -> Self {
        Self {
            tree: Tree::empty(),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

struct Overlay<'a, 'b, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: iced::advanced::Renderer,
{
    position: Point,
    viewport: Rectangle,
    state: &'a mut Tree,
    list: Scrollable<'a, Message, Theme, Renderer>,
    width: f32,
    target_height: f32,
    class: &'a <Theme as Catalog>::Class<'b>,
}

impl<'a, 'b, Message, Theme, Renderer> Overlay<'a, 'b, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: Catalog + scrollable::Catalog + 'a,
    Renderer: iced::advanced::Renderer + 'a,
    'b: 'a,
{
    pub fn new<T>(
        position: Point,
        viewport: Rectangle,
        menu: Menu<'a, 'b, T, Message, Theme, Renderer>,
        target_height: f32,
    ) -> Self
    where
        T: Clone,
    {
        let Menu {
            state,
            options,
            contents,
            hovered_option,
            on_selected,
            on_option_hovered,
            width,
            class,
        } = menu;

        let list = Scrollable::new(List {
            options,
            contents,
            hovered_option,
            on_selected,
            on_option_hovered,
            class,
        });

        state.tree.diff(&list as &dyn Widget<_, _, _>);

        Self {
            position,
            viewport,
            state: &mut state.tree,
            list,
            width,
            target_height,
            class,
        }
    }
}

impl<Message, Theme, Renderer> iced::advanced::Overlay<Message, Theme, Renderer>
    for Overlay<'_, '_, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: iced::advanced::Renderer,
{
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> layout::Node {
        let space_below = bounds.height - (self.position.y + self.target_height);
        let space_above = self.position.y;

        let limits = layout::Limits::new(
            Size::ZERO,
            Size::new(
                bounds.width - self.position.x,
                if space_below > space_above {
                    space_below
                } else {
                    space_above
                },
            ),
        )
        .width(self.width);

        let node = self.list.layout(self.state, renderer, &limits);
        let size = node.size();

        node.move_to(if space_below > space_above {
            self.position + Vector::new(0.0, self.target_height)
        } else {
            self.position - Vector::new(0.0, size.height)
        })
    }

    fn update(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) {
        let bounds = layout.bounds();

        self.list.update(
            self.state, event, layout, cursor, renderer, clipboard, shell, &bounds,
        );
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.list
            .mouse_interaction(self.state, layout, cursor, &self.viewport, renderer)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        defaults: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        let bounds = layout.bounds();

        let style = Catalog::style(theme, self.class);

        renderer.with_layer(bounds, |renderer| {
            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: style.border,
                    ..renderer::Quad::default()
                },
                style.background,
            );

            self.list.draw(
                self.state, renderer, theme, defaults, layout, cursor, &bounds,
            );
        });
    }
}

struct List<'a, 'b, T, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: iced::advanced::Renderer,
{
    options: &'a [T],
    contents: &'a [Element<'a, Message, Theme, Renderer>],
    hovered_option: &'a mut Option<usize>,
    on_selected: Box<dyn FnMut(T) -> Message + 'a>,
    on_option_hovered: Option<&'a dyn Fn(T) -> Message>,
    class: &'a <Theme as Catalog>::Class<'b>,
}

struct ListState {
    is_hovered: Option<bool>,
}

impl<T, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for List<'_, '_, T, Message, Theme, Renderer>
where
    T: Clone,
    Theme: Catalog,
    Renderer: iced::advanced::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<Option<bool>>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(ListState { is_hovered: None })
    }

    fn children(&self) -> Vec<Tree> {
        self.contents.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(self.contents);
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Shrink,
        }
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        if self.contents.is_empty() {
            return layout::Node::new(limits.resolve(Length::Fill, Length::Shrink, Size::ZERO));
        }

        let base = self
            .contents
            .iter()
            .zip(tree.children.iter_mut())
            .map(|(el, tree)| el.as_widget().layout(tree, renderer, limits).size())
            .max_by(|a, b| a.width.total_cmp(&b.width))
            .unwrap();

        let size = limits.resolve(Length::Fill, Length::Shrink, base);
        let limits = layout::Limits::new(Size::ZERO, size);

        let nodes = self
            .contents
            .iter()
            .enumerate()
            .zip(&mut tree.children)
            .map(|((i, layer), tree)| {
                layer
                    .as_widget()
                    .layout(tree, renderer, &limits)
                    .translate(Size::new(0.0, i as f32 * base.height))
            })
            .collect();

        let total_size = Size::new(size.width, size.height * self.options.len() as f32);

        layout::Node::with_children(total_size, nodes)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if cursor.is_over(layout.bounds()) {
                    if let Some(index) = *self.hovered_option {
                        if let Some(option) = self.options.get(index) {
                            shell.publish((self.on_selected)(option.clone()));
                            shell.capture_event();
                        }
                    }
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if let Some(cursor_position) = cursor.position_in(layout.bounds()) {
                    let option = layout.children().next().unwrap().bounds().size();
                    let new_hovered_option = (cursor_position.y / option.height) as usize;

                    if *self.hovered_option != Some(new_hovered_option) {
                        if let Some(option) = self.options.get(new_hovered_option) {
                            if let Some(on_option_hovered) = self.on_option_hovered {
                                shell.publish(on_option_hovered(option.clone()));
                            }

                            shell.request_redraw();
                        }
                    }

                    *self.hovered_option = Some(new_hovered_option);
                }
            }
            Event::Touch(touch::Event::FingerPressed { .. }) => {
                if let Some(cursor_position) = cursor.position_in(layout.bounds()) {
                    let option = layout.children().next().unwrap().bounds().size();

                    *self.hovered_option = Some((cursor_position.y / option.height) as usize);

                    if let Some(index) = *self.hovered_option {
                        if let Some(option) = self.options.get(index) {
                            shell.publish((self.on_selected)(option.clone()));
                            shell.capture_event();
                        }
                    }
                }
            }
            _ => {}
        }

        let state = tree.state.downcast_mut::<ListState>();

        if let Event::Window(window::Event::RedrawRequested(_now)) = event {
            state.is_hovered = Some(cursor.is_over(layout.bounds()));
        } else if state
            .is_hovered
            .is_some_and(|is_hovered| is_hovered != cursor.is_over(layout.bounds()))
        {
            shell.request_redraw();
        }
    }

    fn mouse_interaction(
        &self,
        _state: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let is_mouse_over = cursor.is_over(layout.bounds());

        if is_mouse_over {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let menu_style = Catalog::style(theme, self.class);
        let bounds = layout.bounds();

        let option_height = layout.children().next().unwrap().bounds().size().height;

        let offset = viewport.y - bounds.y;
        let start = (offset / option_height) as usize;
        let end = ((offset + viewport.height) / option_height).ceil() as usize;

        for (i, ((option, layout), tree)) in self
            .contents
            .iter()
            .zip(layout.children())
            .zip(tree.children.iter())
            .skip(start)
            .take(end.min(self.contents.len()))
            .enumerate()
        {
            let i = start + i;
            let is_selected = *self.hovered_option == Some(i);

            let bounds = Rectangle {
                x: bounds.x,
                y: bounds.y + (option_height * i as f32),
                width: bounds.width,
                height: option_height,
            };

            if is_selected {
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: Rectangle {
                            x: bounds.x + menu_style.border.width,
                            width: bounds.width - menu_style.border.width * 2.0,
                            // width: layout.bounds().width,
                            ..bounds
                        },
                        border: border::rounded(menu_style.border.radius),
                        ..renderer::Quad::default()
                    },
                    menu_style.selected_background,
                );
            }

            let style = is_selected
                .then_some(renderer::Style {
                    text_color: menu_style.selected_text_color,
                })
                .unwrap_or(renderer::Style {
                    text_color: menu_style.text_color,
                });

            option
                .as_widget()
                .draw(tree, renderer, theme, &style, layout, cursor, viewport);
        }
    }
}

impl<'a, 'b, T, Message, Theme, Renderer> From<List<'a, 'b, T, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    T: Clone,
    Message: 'a,
    Theme: 'a + Catalog,
    Renderer: 'a + iced::advanced::Renderer,
    'b: 'a,
{
    fn from(list: List<'a, 'b, T, Message, Theme, Renderer>) -> Self {
        Element::new(list)
    }
}

/// The appearance of a [`Menu`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// The [`Background`] of the menu.
    pub background: Background,
    /// The [`Border`] of the menu.
    pub border: Border,
    /// The text [`Color`] of the menu.
    pub text_color: Color,
    /// The text [`Color`] of a selected option in the menu.
    pub selected_text_color: Color,
    /// The background [`Color`] of a selected option in the menu.
    pub selected_background: Background,
}

/// The theme catalog of a [`Menu`].
pub trait Catalog: scrollable::Catalog {
    /// The item class of the [`Catalog`].
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> <Self as Catalog>::Class<'a>;

    /// The default class for the scrollable of the [`Menu`].
    fn default_scrollable<'a>() -> <Self as scrollable::Catalog>::Class<'a> {
        <Self as scrollable::Catalog>::default()
    }

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &<Self as Catalog>::Class<'_>) -> Style;
}

/// A styling function for a [`Menu`].
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> StyleFn<'a, Self> {
        Box::new(default)
    }

    fn style(&self, class: &StyleFn<'_, Self>) -> Style {
        class(self)
    }
}

/// The default style of the list of a [`Menu`].
pub fn default(theme: &Theme) -> Style {
    let palette = theme.extended_palette();

    Style {
        background: palette.background.weak.color.into(),
        border: Border {
            width: 1.0,
            radius: 0.0.into(),
            color: palette.background.strong.color,
        },
        text_color: palette.background.weak.text,
        selected_text_color: palette.primary.strong.text,
        selected_background: palette.primary.strong.color.into(),
    }
}
