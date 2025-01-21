mod widgets;
mod nr_palyer;

pub use nr_palyer::NRPlayer;

pub trait LocalMessage {
    fn broadcast(self) -> GlobalMessage;
} 

#[derive(Debug, Clone)]
pub enum GlobalMessage {
    First,
    Press(usize),
    SwitchNet(String),

    CanvasMessage(widgets::CanvasMessage),
    OperatorMessage(),
}

impl GlobalMessage {

}