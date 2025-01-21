mod core;
mod gui;


const NUM_OF_NOTES: usize = 88;

fn main() -> iced::Result {
    iced::application(
        "Neo-Riemannian Player",
        gui::NRPlayer::update,
        gui::NRPlayer::view
    ).run()
}
