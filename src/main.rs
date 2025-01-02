mod core;
mod gui;

fn main() -> iced::Result {
    iced::application(
        "Neo-Riemannian Player",
        gui::NRPlayer::update,
        gui::NRPlayer::view
    ).run()
}
