pub use crate::model::Model;
pub use crate::process_image::{array_to_tensor, load_image};

mod gui;
mod model;
mod process_image;

fn main() {
    gui::init_window();
}
