use iced::widget::{button, column, container, text};
use iced::{Alignment, Element, Length, Sandbox, Settings};

use loupe::loupe;

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}

struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self { value: 0 }
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        container(loupe(
            3.0,
            column![
                button("Increment").on_press(Message::IncrementPressed),
                text(self.value).size(50),
                button("Decrement").on_press(Message::DecrementPressed)
            ]
            .padding(20)
            .align_items(Alignment::Center),
        ))
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}

mod loupe {
    use iced::advanced::layout::{self, Layout};
    use iced::advanced::renderer;
    use iced::advanced::widget::{self, Widget};
    use iced::advanced::Renderer as _;
    use iced::mouse;
    use iced::{
        Color, Element, Length, Rectangle, Renderer, Theme, Transformation,
    };

    pub fn loupe<'a, Message>(
        zoom: f32,
        content: impl Into<Element<'a, Message>>,
    ) -> Loupe<'a, Message>
    where
        Message: 'static,
    {
        Loupe {
            zoom,
            content: content.into().explain(Color::BLACK),
        }
    }

    pub struct Loupe<'a, Message> {
        zoom: f32,
        content: Element<'a, Message>,
    }

    impl<'a, Message> Widget<Message, Renderer> for Loupe<'a, Message> {
        fn tag(&self) -> widget::tree::Tag {
            self.content.as_widget().tag()
        }

        fn state(&self) -> widget::tree::State {
            self.content.as_widget().state()
        }

        fn children(&self) -> Vec<widget::Tree> {
            self.content.as_widget().children()
        }

        fn diff(&self, tree: &mut widget::Tree) {
            self.content.as_widget().diff(tree);
        }

        fn width(&self) -> Length {
            self.content.as_widget().width()
        }

        fn height(&self) -> Length {
            self.content.as_widget().height()
        }

        fn layout(
            &self,
            tree: &mut widget::Tree,
            renderer: &Renderer,
            limits: &layout::Limits,
        ) -> layout::Node {
            self.content.as_widget().layout(tree, renderer, limits)
        }

        fn draw(
            &self,
            tree: &widget::Tree,
            renderer: &mut Renderer,
            theme: &Theme,
            style: &renderer::Style,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            viewport: &Rectangle,
        ) {
            let bounds = layout.bounds();

            if let Some(position) = cursor.position_in(bounds) {
                renderer.with_layer(bounds, |renderer| {
                    renderer.with_transformation(
                        Transformation::translate(
                            bounds.x + position.x * (1.0 - self.zoom),
                            bounds.y + position.y * (1.0 - self.zoom),
                        ) * Transformation::scale(self.zoom)
                            * Transformation::translate(-bounds.x, -bounds.y),
                        |renderer| {
                            self.content.as_widget().draw(
                                tree,
                                renderer,
                                theme,
                                style,
                                layout,
                                mouse::Cursor::Unavailable,
                                viewport,
                            );
                        },
                    );
                });
            } else {
                self.content.as_widget().draw(
                    tree, renderer, theme, style, layout, cursor, viewport,
                );
            }
        }

        fn mouse_interaction(
            &self,
            _state: &widget::Tree,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            _viewport: &Rectangle,
            _renderer: &Renderer,
        ) -> mouse::Interaction {
            if cursor.is_over(layout.bounds()) {
                mouse::Interaction::ZoomIn
            } else {
                mouse::Interaction::Idle
            }
        }
    }

    impl<'a, Message> From<Loupe<'a, Message>> for Element<'a, Message, Renderer>
    where
        Message: 'a,
    {
        fn from(loupe: Loupe<'a, Message>) -> Self {
            Self::new(loupe)
        }
    }
}
