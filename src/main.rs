#![allow(static_mut_refs)]

use crate::{ags::AGS, gui::GUIManager};

pub mod ags;
pub mod audio;
pub mod gui;

fn main() {
    let mut ags = AGS::new();
    GUIManager::go(&mut ags);
}
