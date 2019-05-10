use crate::ui::{Length, Widget};

pub struct Button<'a, M> {
    state: &'a mut State,
    label: String,
    width: Length,
    on_click: Option<M>,
}

impl<'a, M> Button<'a, M> {
    pub fn new(state: &'a mut State, label: &str) -> Self {
        Button {
            state,
            label: String::from(label),
            width: Length::Shrink,
            on_click: None,
        }
    }

    pub fn width(mut self, length: Length) -> Self {
        self.width = length;
        self
    }

    pub fn on_click(mut self, msg: M) -> Self {
        self.on_click = Some(msg);
        self
    }
}

impl<'a, M> Widget for Button<'a, M> {
    type Msg = M;

    fn node(&self) -> stretch::node::Node {
        let mut style = stretch::style::Style::default();
        style.size.height = stretch::style::Dimension::Points(40.0);

        match self.width {
            Length::Shrink => {}
            Length::Fill => {
                style.flex_grow = 1.0;
            }
            Length::Px(width) => {
                style.size.width =
                    stretch::style::Dimension::Points(width as f32);
            }
        }

        stretch::node::Node::new(style, Vec::new())
    }
}

pub struct State {}

impl State {
    pub fn new() -> State {
        State {}
    }
}
