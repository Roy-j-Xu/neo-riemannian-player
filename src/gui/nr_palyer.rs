use iced::Element;
use iced::widget::{text, button}; 


use crate::gui::widgets;



// #[derive(Debug, Default)]
// pub struct NRPlayer {
//     nr_canvas: NRCanvas,
//     state: State,
// }

// #[derive(Debug, Default)]
// struct State {
//     count: u8,
// }



// impl NRPlayer {
//     pub fn _new() -> Self {
//         NRPlayer::default()
//     }

//     pub fn view(&self) -> Element<GlobalMessage> {
//         iced::widget::column![
//             text(format!("Testing {}", self.state.count)),
//             button("+").on_press(),
//             self.nr_canvas.view(),
//         ].into()
//     }

//     pub fn update(&mut self, message: GlobalMessage) {
//         match message {
            
//         }
//     }
// }