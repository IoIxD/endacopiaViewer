use parking_lot::Mutex;
use rodio::{Decoder, MixerDeviceSink, Player, Source};
use std::{io::Cursor, time::Duration};

pub struct AudioPlayer {
    handle: &'static mut MixerDeviceSink,
    player: Mutex<Player>,
    loaded_bytes: Mutex<Vec<u8>>,
    source: Mutex<Option<Decoder<Cursor<Vec<u8>>>>>,

    sample_rate: Mutex<u32>,
    channels: Mutex<u16>,
    sample_count: Mutex<usize>,
}

impl AudioPlayer {
    pub fn new() -> Self {
        // Get an OS-Sink handle to the default physical sound device.
        // Note that the playback stops when the handle is dropped.//!
        let handle = Box::leak(Box::new(
            rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream"),
        ));
        let player = rodio::Player::connect_new(&handle.mixer());
        Self {
            handle,
            player: Mutex::new(player),
            loaded_bytes: Mutex::new(vec![]),
            source: Mutex::new(None),
            sample_rate: Mutex::new(0),
            channels: Mutex::new(0),
            sample_count: Mutex::new(0),
        }
    }
    pub fn upload(&self, bytes: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let player = self.player.lock();
        let mut loaded_bytes = self.loaded_bytes.lock();
        *loaded_bytes = bytes;

        /* calculate params (see len) */
        {
            let file = Cursor::new(loaded_bytes.clone());
            let src = Decoder::try_from(file).unwrap();
            *self.sample_rate.lock() = src.sample_rate().get();
            *self.channels.lock() = src.channels().get();
            *self.sample_count.lock() = src.count();
        }

        player.try_seek(Duration::from_secs(0))?;

        Ok(())
    }

    pub fn play(&self) {
        let player = self.player.lock();
        if player.empty() {
            let file = Cursor::new(self.loaded_bytes.lock().clone());
            let source = Decoder::try_from(file).unwrap();
            player.append(source);
            player.play();
        } else {
            if !player.is_paused() {
                player.pause()
            } else {
                player.play()
            }
        }
    }
    pub fn stop(&self) {
        let player = self.player.lock();
        player.stop();
    }

    pub fn pos(&self) -> String {
        let pos = self.player.lock().get_pos();
        format!("{:.03}", pos.as_secs_f32())
    }

    pub fn len(&self) -> String {
        /* for some reason the decoder doesn't really want
         * to give us the duration of these ogg files, hence
         * us having to calculate it ourselves */
        let sample_rate = self.sample_rate.lock().clone();
        let channels = self.channels.lock().clone();
        let sample_count = self.sample_count.lock().clone();

        if sample_rate != 0 && sample_count != 0 && channels != 0 {
            let secs = sample_count as f32 / sample_rate as f32 / channels as f32;
            format!("{:.03}", secs)
        } else {
            String::from("000:000")
        }
    }

    pub fn playing(&self) -> bool {
        let player = self.player.lock();
        !player.is_paused() && !player.empty()
    }
}
