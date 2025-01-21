use iced::Element;
use iced::widget::{text, button}; 


use super::widgets::NRCanvas;
use super::GlobalMessage;



#[derive(Debug, Default)]
pub struct NRPlayer {
    nr_canvas: NRCanvas,
    state: NRPlayerState,
}

#[derive(Debug, Default)]
struct NRPlayerState {
    count: u8,
}



impl NRPlayer {
    pub fn _new() -> Self {
        NRPlayer::default()
    }

    pub fn view(&self) -> Element<GlobalMessage> {
        iced::widget::column![
            text(format!("Testing {}", self.state.count)),
            button("+").on_press(GlobalMessage::First),
            self.nr_canvas.view(),
        ].into()
    }

    pub fn update(&mut self, message: GlobalMessage) {
        match message {
            GlobalMessage::First => { self.state.count += 1; },
            GlobalMessage::CanvasMessage(_) => { self.state.count += 10; },
            _ => { },
        }
    }
}