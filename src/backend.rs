use qtbridge::*;

use crate::{ags::AGS, filetypes::audio::AudioPlayer};

#[derive(Default)]
pub struct Backend {
    pub ags: Option<AGS>,
    pub player: Option<AudioPlayer>,
    pub image_data_url: String,
    pub image_width: usize,
    pub image_height: usize,
}

#[qobject]
impl Backend {
    qproperty!("image_data_url", Member = image_data_url);
    qproperty!("image_width", Member = image_width);
    qproperty!("image_height", Member = image_height);

    #[qslot]
    fn setup(&mut self) {
        if let None = self.ags {
            self.ags = Some(AGS::new());
        }
        if let None = self.player {
            self.player = Some(AudioPlayer::new());
        }

        let mut items = self
            .ags
            .as_ref()
            .unwrap()
            .normal_files()
            .keys()
            .map(|f| f.clone())
            .collect::<Vec<_>>();
        items.sort();
        items.reverse();

        self.update_filenames(items);
    }

    #[qslot]
    fn do_file_action(&mut self, str: String) {
        if str.contains(".ogg") {
            self.handle_ogg(str);
        } else if str.contains(".ttf") {
            self.handle_ttf(str);
        } else if str.contains(".crm") {
            self.handle_crm(str);
        }
    }

    #[qslot]
    fn play_sound(&self) {
        self.player.as_ref().unwrap().play();
    }

    #[qslot]
    fn stop_sound(&self) {
        self.player.as_ref().unwrap().stop();
    }

    #[qslot]
    fn sound_playing(&self) -> bool {
        self.player.as_ref().unwrap().playing()
    }

    #[qslot]
    fn sound_len(&self) -> String {
        self.player.as_ref().unwrap().len()
    }
    #[qslot]
    fn sound_pos(&self) -> String {
        self.player.as_ref().unwrap().pos()
    }

    #[qsignal(qml_name = "updateFilenames")]
    pub fn update_filenames(&mut self, items: Vec<String>);

    #[qsignal(qml_name = "showPlayerMenu")]
    pub fn show_player_menu(&mut self);

    #[qsignal(qml_name = "imageDisplay")]
    pub fn image_display(&mut self);
}
