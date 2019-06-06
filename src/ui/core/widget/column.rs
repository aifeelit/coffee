use std::hash::Hash;

use crate::graphics::Point;
use crate::ui::core::{
    Align, Element, Event, Hasher, Justify, Layout, MouseCursor, Node, Style,
    Widget,
};

/// A container that places its contents __vertically__.
pub struct Column<'a, M, R> {
    style: Style,
    spacing: u16,
    children: Vec<Element<'a, M, R>>,
}

impl<'a, M, R> Column<'a, M, R> {
    /// Creates an empty [`Column`].
    ///
    /// [`Column`]: struct.Column.html
    pub fn new() -> Self {
        let mut style = Style::default().fill_width();
        style.0.flex_direction = stretch::style::FlexDirection::Column;

        Column {
            style,
            spacing: 0,
            children: Vec::new(),
        }
    }

    /// Sets the vertical spacing _between_ elements in pixels.
    ///
    /// Custom margins per element do not exist in Coffee. You should use this
    /// method instead! While less flexible, it helps you keep spacing between
    /// elements consistent.
    pub fn spacing(mut self, px: u16) -> Self {
        self.spacing = px;
        self
    }

    /// Sets the padding of the [`Column`] in pixels.
    ///
    /// [`Column`]: struct.Column.html
    pub fn padding(mut self, px: u32) -> Self {
        self.style = self.style.padding(px);
        self
    }

    /// Sets the width of the [`Column`] in pixels.
    ///
    /// [`Column`]: struct.Column.html
    pub fn width(mut self, width: u32) -> Self {
        self.style = self.style.width(width);
        self
    }

    /// Sets the height of the [`Column`] in pixels.
    ///
    /// [`Column`]: struct.Column.html
    pub fn height(mut self, height: u32) -> Self {
        self.style = self.style.height(height);
        self
    }

    /// Sets the maximum width of the [`Column`] in pixels.
    ///
    /// [`Column`]: struct.Column.html
    pub fn max_width(mut self, max_width: u32) -> Self {
        self.style = self.style.max_width(max_width);
        self
    }

    /// Sets the maximum height of the [`Column`] in pixels.
    ///
    /// [`Column`]: struct.Column.html
    pub fn max_height(mut self, max_height: u32) -> Self {
        self.style = self.style.max_height(max_height);
        self
    }

    /// Sets the alignment of the [`Column`] itself.
    ///
    /// This is useful if you want to override the default alignment given by
    /// the parent container.
    ///
    /// [`Column`]: struct.Column.html
    pub fn align_self(mut self, align: Align) -> Self {
        self.style = self.style.align_self(align);
        self
    }

    /// Sets the horizontal alignment of the contents of the [`Column`] .
    ///
    /// [`Column`]: struct.Column.html
    pub fn align_items(mut self, align: Align) -> Self {
        self.style = self.style.align_items(align);
        self
    }

    /// Sets the vertical distribution strategy for the contents of the
    /// [`Column`] .
    ///
    /// [`Column`]: struct.Column.html
    pub fn justify_content(mut self, justify: Justify) -> Self {
        self.style = self.style.justify_content(justify);
        self
    }

    /// Adds an [`Element`] to the [`Column`].
    ///
    /// [`Element`]: ../struct.Element.html
    /// [`Column`]: struct.Column.html
    pub fn push<E>(mut self, child: E) -> Column<'a, M, R>
    where
        E: Into<Element<'a, M, R>>,
    {
        self.children.push(child.into());
        self
    }
}

impl<'a, M, R> Widget for Column<'a, M, R> {
    type Message = M;
    type Renderer = R;

    fn node(&self, renderer: &R) -> Node {
        let mut children: Vec<Node> = self
            .children
            .iter()
            .map(|child| {
                let mut node = child.widget.node(renderer);

                let mut style = node.0.style();
                style.margin.bottom =
                    stretch::style::Dimension::Points(self.spacing as f32);

                node.0.set_style(style);
                node
            })
            .collect();

        if let Some(node) = children.last_mut() {
            let mut style = node.0.style();
            style.margin.bottom = stretch::style::Dimension::Undefined;

            node.0.set_style(style);
        }

        Node::with_children(self.style, children)
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout,
        cursor_position: Point,
        messages: &mut Vec<Self::Message>,
    ) {
        self.children.iter_mut().zip(layout.children()).for_each(
            |(child, layout)| {
                child
                    .widget
                    .on_event(event, layout, cursor_position, messages)
            },
        );
    }

    fn draw(
        &self,
        renderer: &mut Self::Renderer,
        layout: Layout,
        cursor_position: Point,
    ) -> MouseCursor {
        let mut cursor = MouseCursor::OutOfBounds;

        self.children.iter().zip(layout.children()).for_each(
            |(child, layout)| {
                let new_cursor =
                    child.widget.draw(renderer, layout, cursor_position);

                if new_cursor != MouseCursor::OutOfBounds {
                    cursor = new_cursor;
                }
            },
        );

        cursor
    }

    fn hash(&self, state: &mut Hasher) {
        self.style.hash(state);
        self.spacing.hash(state);

        for child in &self.children {
            child.widget.hash(state);
        }
    }
}

impl<'a, M, R> From<Column<'a, M, R>> for Element<'a, M, R>
where
    R: 'static,
    M: 'static,
{
    fn from(column: Column<'a, M, R>) -> Element<'a, M, R> {
        Element::new(column)
    }
}
