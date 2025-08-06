use crate::widget::menu::{self, Menu};
use iced::advanced::layout;
use iced::advanced::renderer;
use iced::advanced::text;
use iced::advanced::widget::tree::{self, Tree};
use iced::advanced::{Clipboard, Layout, Shell, Widget};
use iced::keyboard;
use iced::mouse;
use iced::overlay;
use iced::touch;
use iced::widget::container;
use iced::window;
use iced::{
    Background, Border, Color, Element, Event, Length, Padding, Pixels, Rectangle, Size, Theme,
    Vector,
};

use std::borrow::Borrow;

const DEFAULT_PADDING: Padding = Padding {
    top: 5.0,
    bottom: 5.0,
    right: 10.0,
    left: 10.0,
};

#[allow(missing_debug_implementations)]
pub struct PickList<'a, T, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    T: PartialEq + Clone,
    Theme: Catalog,
    Renderer: text::Renderer,
{
    on_select: Option<Box<dyn Fn(T) -> Message + 'a>>,
    on_open: Option<Message>,
    on_close: Option<Message>,
    options: &'a [T],
    placeholder: Option<&'a T>,
    selected: Option<&'a T>,
    contents: Vec<Element<'a, Message, Theme, Renderer>>,
    width: Length,
    padding: Padding,
    text_size: Option<Pixels>,
    text_line_height: text::LineHeight,
    text_shaping: text::Shaping,
    font: Option<Renderer::Font>,
    handle: Handle<Renderer::Font>,
    class: <Theme as Catalog>::Class<'a>,
    menu_class: <Theme as menu::Catalog>::Class<'a>,
    last_status: Option<Status>,
}

impl<'a, T, Message, Theme, Renderer> PickList<'a, T, Message, Theme, Renderer>
where
    T: PartialEq + Clone,
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: text::Renderer + 'a,
{
    /// Creates a new [`PickList`] with the given list of options, the current
    /// selected value, and the message to produce when an option is selected.
    pub fn new(
        options: &'a [T],
        selected: Option<&'a T>,
        view: impl Fn(&'a T) -> Element<'a, Message, Theme, Renderer>,
    ) -> Self {
        let contents = options.iter().map(view).collect();

        Self {
            on_select: None,
            on_open: None,
            on_close: None,
            options,
            placeholder: None,
            selected,
            contents,
            width: Length::Shrink,
            padding: DEFAULT_PADDING,
            text_size: None,
            text_line_height: text::LineHeight::default(),
            text_shaping: text::Shaping::default(),
            font: None,
            handle: Handle::default(),
            class: <Theme as Catalog>::default(),
            menu_class: <Theme as Catalog>::default_menu(),
            last_status: None,
        }
    }

    pub fn on_select(mut self, on_select: impl Fn(T) -> Message + 'a) -> Self {
        self.on_select = Some(Box::new(on_select));
        self
    }

    /// Sets the placeholder of the [`PickList`].
    pub fn placeholder(mut self, placeholder: &'a T) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    /// Sets the width of the [`PickList`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the [`Padding`] of the [`PickList`].
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the text size of the [`PickList`].
    pub fn text_size(mut self, size: impl Into<Pixels>) -> Self {
        self.text_size = Some(size.into());
        self
    }

    /// Sets the text [`text::LineHeight`] of the [`PickList`].
    pub fn text_line_height(mut self, line_height: impl Into<text::LineHeight>) -> Self {
        self.text_line_height = line_height.into();
        self
    }

    /// Sets the [`text::Shaping`] strategy of the [`PickList`].
    pub fn text_shaping(mut self, shaping: text::Shaping) -> Self {
        self.text_shaping = shaping;
        self
    }

    /// Sets the font of the [`PickList`].
    pub fn font(mut self, font: impl Into<Renderer::Font>) -> Self {
        self.font = Some(font.into());
        self
    }

    /// Sets the [`Handle`] of the [`PickList`].
    pub fn handle(mut self, handle: Handle<Renderer::Font>) -> Self {
        self.handle = handle;
        self
    }

    /// Sets the message that will be produced when the [`PickList`] is opened.
    pub fn on_open(mut self, on_open: Message) -> Self {
        self.on_open = Some(on_open);
        self
    }

    /// Sets the message that will be produced when the [`PickList`] is closed.
    pub fn on_close(mut self, on_close: Message) -> Self {
        self.on_close = Some(on_close);
        self
    }

    /// Sets the style of the [`PickList`].
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        <Theme as Catalog>::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }

    /// Sets the style of the [`Menu`].
    #[must_use]
    pub fn menu_style(mut self, style: impl Fn(&Theme) -> menu::Style + 'a) -> Self
    where
        <Theme as menu::Catalog>::Class<'a>: From<menu::StyleFn<'a, Theme>>,
    {
        self.menu_class = (Box::new(style) as menu::StyleFn<'a, Theme>).into();
        self
    }

    /// Sets the style class of the [`PickList`].
    #[must_use]
    pub fn class(mut self, class: impl Into<<Theme as Catalog>::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }

    /// Sets the style class of the [`Menu`].
    #[must_use]
    pub fn menu_class(mut self, class: impl Into<<Theme as menu::Catalog>::Class<'a>>) -> Self {
        self.menu_class = class.into();
        self
    }

    fn current(&self) -> Option<&Element<'a, Message, Theme, Renderer>> {
        self.options
            .iter()
            .position(|option| {
                self.selected
                    .or(self.placeholder)
                    .map(|visible| option == visible)
                    .unwrap_or(false)
            })
            .and_then(|i| self.contents.get(i))
    }
}

impl<'a, T, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for PickList<'a, T, Message, Theme, Renderer>
where
    T: Clone + PartialEq,
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: text::Renderer + 'a,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new())
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: Length::Shrink,
        }
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let intrinsic = self
            .current()
            .map(Element::as_widget)
            .map(|widget| widget.layout(tree, renderer, limits).size());

        let size = limits
            .width(self.width)
            .shrink(self.padding)
            .resolve(self.width, Length::Shrink, intrinsic.unwrap_or(Size::ZERO))
            .expand(self.padding);

        layout::Node::new(size)
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
        let state = tree.state.downcast_mut::<State>();

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if state.is_open {
                    // Event wasn't processed by overlay, so cursor was clicked either outside its
                    // bounds or on the drop-down, either way we close the overlay.
                    state.is_open = false;

                    if let Some(on_close) = &self.on_close {
                        shell.publish(on_close.clone());
                    }

                    shell.capture_event();
                } else if cursor.is_over(layout.bounds()) {
                    let selected = self.selected.as_ref().map(Borrow::borrow);

                    state.is_open = true;
                    state.hovered_option = self
                        .options
                        .iter()
                        .position(|option| Some(option) == selected);

                    if let Some(on_open) = &self.on_open {
                        shell.publish(on_open.clone());
                    }

                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::WheelScrolled {
                delta: mouse::ScrollDelta::Lines { y, .. },
            }) => {
                if let Some(on_select) = &self.on_select
                    && state.keyboard_modifiers.command()
                    && cursor.is_over(layout.bounds())
                    && !state.is_open
                {
                    fn find_next<'a, T: PartialEq>(
                        selected: &'a T,
                        mut options: impl Iterator<Item = &'a T>,
                    ) -> Option<&'a T> {
                        let _ = options.find(|&option| option == selected);

                        options.next()
                    }

                    let selected = self.selected.as_ref().map(Borrow::borrow);

                    let next_option = if *y < 0.0 {
                        if let Some(selected) = selected {
                            find_next(selected, self.options.iter())
                        } else {
                            self.options.first()
                        }
                    } else if *y > 0.0 {
                        if let Some(selected) = selected {
                            find_next(selected, self.options.iter().rev())
                        } else {
                            self.options.last()
                        }
                    } else {
                        None
                    };

                    if let Some(next_option) = next_option {
                        shell.publish((on_select)(next_option.clone()));
                    }

                    shell.capture_event();
                }
            }
            Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) => {
                state.keyboard_modifiers = *modifiers;
            }
            _ => {}
        };

        let status = {
            let is_hovered = cursor.is_over(layout.bounds());

            if self.on_select.is_none() {
                Status::Disabled
            } else if state.is_open {
                Status::Opened { is_hovered }
            } else if is_hovered {
                Status::Hovered
            } else {
                Status::Active
            }
        };

        if let Event::Window(window::Event::RedrawRequested(_now)) = event {
            self.last_status = Some(status);
        } else if self
            .last_status
            .is_some_and(|last_status| last_status != status)
        {
            shell.request_redraw();
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let bounds = layout.bounds();
        let is_mouse_over = cursor.is_over(bounds);

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
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();

        {
            let style = Catalog::style(
                theme,
                &self.class,
                self.last_status.unwrap_or(Status::Disabled),
            );

            let container = container::Style {
                text_color: Some(style.text_color),
                background: Some(style.background),
                border: style.border,
                ..Default::default()
            };

            container::draw_background(renderer, &container, bounds);
        }

        if let Some(element) = self.current().map(Element::as_widget) {
            element.draw(tree, renderer, theme, style, layout, cursor, viewport);
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        let state = tree.state.downcast_mut::<State>();
        let font = self.font.unwrap_or_else(|| renderer.default_font());

        if let Some(on_select) = &self.on_select
            && state.is_open
        {
            let bounds = layout.bounds();

            let mut menu = Menu::new(
                &mut state.menu,
                self.options,
                &mut state.hovered_option,
                |option| {
                    state.is_open = false;

                    (on_select)(option)
                },
                None,
                &self.menu_class,
            )
            .width(bounds.width)
            .padding(self.padding)
            .font(font)
            .text_shaping(self.text_shaping);

            if let Some(text_size) = self.text_size {
                menu = menu.text_size(text_size);
            }

            Some(menu.overlay(layout.position() + translation, *viewport, bounds.height))
        } else {
            None
        }
    }
}

impl<'a, T, Message, Theme, Renderer> From<PickList<'a, T, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    T: Clone + PartialEq,
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: text::Renderer + 'a,
{
    fn from(pick_list: PickList<'a, T, Message, Theme, Renderer>) -> Self {
        Self::new(pick_list)
    }
}

#[derive(Debug)]
struct State {
    menu: menu::State,
    keyboard_modifiers: keyboard::Modifiers,
    is_open: bool,
    hovered_option: Option<usize>,
}

impl State {
    /// Creates a new [`State`] for a [`PickList`].
    fn new() -> Self {
        Self {
            menu: menu::State::default(),
            keyboard_modifiers: keyboard::Modifiers::default(),
            is_open: bool::default(),
            hovered_option: Option::default(),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

/// The handle to the right side of the [`PickList`].
#[derive(Debug, Clone, PartialEq)]
pub enum Handle<Font> {
    /// Displays an arrow icon (▼).
    ///
    /// This is the default.
    Arrow {
        /// Font size of the content.
        size: Option<Pixels>,
    },
    /// A custom static handle.
    Static(Icon<Font>),
    /// A custom dynamic handle.
    Dynamic {
        /// The [`Icon`] used when [`PickList`] is closed.
        closed: Icon<Font>,
        /// The [`Icon`] used when [`PickList`] is open.
        open: Icon<Font>,
    },
    /// No handle will be shown.
    None,
}

impl<Font> Default for Handle<Font> {
    fn default() -> Self {
        Self::Arrow { size: None }
    }
}

/// The icon of a [`Handle`].
#[derive(Debug, Clone, PartialEq)]
pub struct Icon<Font> {
    /// Font that will be used to display the `code_point`,
    pub font: Font,
    /// The unicode code point that will be used as the icon.
    pub code_point: char,
    /// Font size of the content.
    pub size: Option<Pixels>,
    /// Line height of the content.
    pub line_height: text::LineHeight,
    /// The shaping strategy of the icon.
    pub shaping: text::Shaping,
}

/// The possible status of a [`PickList`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// The [`PickList`] can be interacted with.
    Active,
    /// The [`PickList`] is being hovered.
    Hovered,
    /// The [`PickList`] is open.
    Opened {
        /// Whether the [`PickList`] is hovered, while open.
        is_hovered: bool,
    },
    /// The [`PickList`] is disabled.
    Disabled,
}

/// The appearance of a pick list.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// The text [`Color`] of the pick list.
    pub text_color: Color,
    /// The placeholder [`Color`] of the pick list.
    pub placeholder_color: Color,
    /// The handle [`Color`] of the pick list.
    pub handle_color: Color,
    /// The [`Background`] of the pick list.
    pub background: Background,
    /// The [`Border`] of the pick list.
    pub border: Border,
}

/// The theme catalog of a [`PickList`].
pub trait Catalog: menu::Catalog {
    /// The item class of the [`Catalog`].
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> <Self as Catalog>::Class<'a>;

    /// The default class for the menu of the [`PickList`].
    fn default_menu<'a>() -> <Self as menu::Catalog>::Class<'a> {
        <Self as menu::Catalog>::default()
    }

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &<Self as Catalog>::Class<'_>, status: Status) -> Style;
}

/// A styling function for a [`PickList`].
///
/// This is just a boxed closure: `Fn(&Theme, Status) -> Style`.
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> StyleFn<'a, Self> {
        Box::new(default)
    }

    fn style(&self, class: &StyleFn<'_, Self>, status: Status) -> Style {
        class(self, status)
    }
}

/// The default style of the field of a [`PickList`].
pub fn default(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();

    let active = Style {
        text_color: palette.background.weak.text,
        background: palette.background.weak.color.into(),
        placeholder_color: palette.background.strong.color,
        handle_color: palette.background.weak.text,
        border: Border {
            radius: 2.0.into(),
            width: 1.0,
            color: palette.background.strong.color,
        },
    };

    match status {
        Status::Active => active,
        Status::Hovered | Status::Opened { .. } => Style {
            border: Border {
                color: palette.primary.strong.color,
                ..active.border
            },
            ..active
        },
        Status::Disabled => Style {
            text_color: active.text_color.scale_alpha(0.5),
            background: active.background.scale_alpha(0.5),
            placeholder_color: active.placeholder_color.scale_alpha(0.5),
            handle_color: active.handle_color.scale_alpha(0.5),
            border: Border {
                color: active.border.color.scale_alpha(0.5),
                ..active.border
            },
        },
    }
}
