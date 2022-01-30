mod view;
use iced::{
    Sandbox, Settings
};

fn main() -> iced::Result {
    view::ventana::AutomataCfe::run(Settings::default())
}
