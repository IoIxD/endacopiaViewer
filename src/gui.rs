use qtrs::prelude::*;

use crate::{ags::AGS, audio::AudioPlayer};

pub struct GUIManager;

impl GUIManager {
    pub fn go(ags: &mut AGS) {
        let player = AudioPlayer::new();

        let app = Application::new();

        let mut window = Widget::new()
            .title("Endacopia Viewer")
            .size(640, 480)
            .build();

        let mut hbox = HBoxLayout::with_parent(&window);

        let mut list = ListWidget::new().build();

        let mut controls = Widget::new().build();
        let mut controls_layout = HBoxLayout::with_parent(&controls);
        let play_pause_btn = PushButton::new("►").build();
        let stop_btn = PushButton::new("■").build();
        let progress_bar = ProgressBar::new().build();
        controls_layout.add(play_pause_btn);
        controls_layout.add(stop_btn);
        controls_layout.add(progress_bar);
        controls.set_layout(&controls_layout);
        controls.hide();

        ags.normal_files()
            .iter()
            .for_each(|(_, f)| list.add_item(f.filename().as_str()));

        list.connect_item_clicked(|str| {
            /* music player */
            if str.contains(".ogg") {
                let bytes = ags.get_file(str);
                let max = bytes.len() as i32;
                if let Err(err) = player.upload(bytes) {
                    println!("{}", err);
                } else {
                    println!("play");
                    // progress_bar.set_range(0, max);
                    player.play();
                };

                controls.show();
            }
        });

        let timer = Timer::new(1)
            .on_timeout(move || {
                if (player.pos() != 0) {
                    println!("{}", player.pos());
                }
            })
            .build();

        hbox.add(list);

        hbox.add(controls);

        window.set_layout(&hbox);
        window.show();

        app.exec();
    }
}
