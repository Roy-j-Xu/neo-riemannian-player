use iced::widget::canvas::Geometry;
use iced::{mouse, Element, Renderer, Theme};
use iced::widget::{text, button, canvas};




#[derive(Debug, Clone)]
pub enum Message {
    Press(usize),
}


#[derive(Debug, Default)]
pub struct NoteCanvas {
    cache: canvas::Cache
    
}

impl NoteCanvas {
    
    pub fn view(&self) -> Element<Message> {
        button("+10")
        .on_press(Message::Press(10))
        .into()
    }
    
}
