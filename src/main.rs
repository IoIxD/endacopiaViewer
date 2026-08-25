use qtbridge::QApp;

use crate::backend::Backend;

pub mod ags;
pub mod backend;
pub mod filetypes;
pub mod readutils;

fn main() {
    QApp::new()
        .register::<Backend>()
        .load_qml(include_bytes!("qml/Main.qml"))
        .run();
}
