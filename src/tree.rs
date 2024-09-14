use iced::advanced::{Layout, Widget};
use iced::{Element, Length, Rectangle, Size, Theme};
use iced::advanced::layout::{Limits, Node};
use iced::advanced::renderer::Style;
use iced::advanced::widget::Tree;
use iced::mouse::Cursor;
use crate::Message;
use iced::Renderer;

struct ProjectTree<'a, Message> {
    base: Element<'a, Message>,
    tree: Element<'a, Message>,
}

impl<'a, Message> Widget<Message, Theme, Renderer> for ProjectTree<'a, Message> {
    fn size(&self) -> Size<Length> {
        todo!()
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        todo!()
    }

    fn draw(&self, tree: &Tree, renderer: &mut Renderer, theme: &Theme, style: &Style, layout: Layout<'_>, cursor: Cursor, viewport: &Rectangle) {
        todo!()
    }
}