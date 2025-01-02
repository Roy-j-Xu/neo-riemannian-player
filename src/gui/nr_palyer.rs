
use iced::Element;
use iced::widget::{text, button}; 

use crate::gui::widgets;



#[derive(Debug, Default)]
pub struct NRPlayer {
    _nr_canvas: widgets::NRCanvas,
    state: NRPlayerState,
}

#[derive(Debug, Default)]
struct NRPlayerState {
    count: u8,
}

#[derive(Debug, Clone)]
pub enum NRPlayerMessage {
    First,
    _Second,
}

impl NRPlayer {
    pub fn _new() -> Self {
        NRPlayer {
            _nr_canvas: widgets::NRCanvas::default(),
            state: NRPlayerState{ count: 0 },
        }
    }

    pub fn view(&self) -> Element<NRPlayerMessage> {
        iced::widget::column![
            text(format!("Testing {}", self.state.count)),
            button("+").on_press(NRPlayerMessage::First)
        ].into()
    }

    pub fn update(&mut self, message: NRPlayerMessage) {
        if let NRPlayerMessage::First = message { self.state.count += 1; }
    }
}