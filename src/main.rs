use qtbridge::{QApp, qobject, qsignal};

use crate::{ags::AGS, audio::AudioPlayer};

pub mod ags;
pub mod audio;

#[derive(Default)]
pub struct Backend {
    ags: Option<AGS>,
    player: Option<AudioPlayer>,
}

#[qobject]
impl Backend {
    #[qslot]
    fn setup(&mut self) {
        if let None = self.ags {
            self.ags = Some(AGS::new());
        }
        if let None = self.player {
            self.player = Some(AudioPlayer::new());
        }

        self.filenames_changed(
            self.ags
                .as_ref()
                .unwrap()
                .normal_files()
                .keys()
                .map(|f| f.clone())
                .collect::<Vec<_>>(),
        );
    }

    #[qslot]
    fn sound_selected(&mut self, str: String) {
        let ags = self.ags.as_ref().unwrap();
        let player = self.player.as_ref().unwrap();

        let bytes = ags.get_file(str);
        if let Err(err) = player.upload(bytes) {
            println!("{}", err);
        } else {
            player.play();
        };
    }

    #[qsignal(qml_name = "filenamesChanged")]
    fn filenames_changed(&mut self, items: Vec<String>);
}

fn main() {
    QApp::new()
        .register::<Backend>()
        .load_qml(include_bytes!("qml/Main.qml"))
        .run();
}
