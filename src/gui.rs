use qtbridge::{QApp, qobject};

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

        /*
         let app = QApp::new();

        let mut window = QWidget::new()
            .title("Endacopia Viewer")
            .size(640, 480)
            .build();

        let mut hbox = HBoxLayout::with_parent(&window);

        let mut list = ListWidget::new().build();

        let mut controls = Widget::new().build();
        let mut controls_layout = HBoxLayout::with_parent(&controls);
        let play_pause_btn = PushButton::new("►").build();
        let stop_btn = PushButton::new("■").build();
        let mut time = Label::new("").build();
        controls_layout.add(play_pause_btn);
        controls_layout.add(stop_btn);
        time.set_object_name("time_label");

        controls.set_layout(&controls_layout);
        controls.hide();

        ags.normal_files()
            .iter()
            .for_each(|(_, f)| list.add_item(f.filename().as_str()));

        list.connect_item_clicked(|str| {
            /* music player */
            if str.contains(".ogg") {
                let bytes = ags.get_file(str);
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

        hbox.add(list);

        hbox.add(controls);

        window.set_layout(&hbox);
        window.show();

        let timer = Timer::new(1)
            .on_timeout(move || {
                if let Some(mut label) = window.find(WidgetKind::Label, "time_label") {
                    // label.set_text(player.pos());
                }
            })
            .build();

        app.exec();*/
    }
}
