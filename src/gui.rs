use qtbridge::{qobject, QApp};

use crate::{ags::AGS, audio::AudioPlayer};

#[derive(Default)]
pub struct Backend {}

#[qobject(Singleton)]
impl Backend {
    #[qslot]
    fn say_hello(&self) {
        println!("Hello World!")
    }
}

pub struct GUIManager;

impl GUIManager {
    pub fn go(ags: &mut AGS) {
        // let player = AudioPlayer::new();

        QApp::new()
            .register::<Backend>()
            .load_qml(include_bytes!("qml/Main.qml"))
            .run();
    }
}
