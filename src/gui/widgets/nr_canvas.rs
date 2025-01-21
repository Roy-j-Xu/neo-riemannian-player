use iced::widget::canvas::Geometry;
use iced::{mouse, Element, Renderer, Theme};
use iced::widget::{text, button, canvas};

use crate::gui::{LocalMessage, GlobalMessage}; 


#[derive(Debug, Clone)]
pub enum CanvasMessage {
    Press(usize),
}

impl LocalMessage for CanvasMessage {
    fn broadcast(self) -> GlobalMessage {
        GlobalMessage::CanvasMessage(self)
    }
}


#[derive(Debug, Default)]
pub struct NRCanvas {
    cache: canvas::Cache
    
}

impl NRCanvas {
    
    pub fn view(&self) -> Element<GlobalMessage> {
        button("+10")
        .on_press(CanvasMessage::Press(10).broadcast())
        .into()
    }
    
}



impl<GlobalMessage> canvas::Program<GlobalMessage> for NRCanvas {
    type State = ();

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: iced::Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let geo = self.cache.draw(renderer, bounds.size(), |frame| {

        });
        todo!()
    }
}