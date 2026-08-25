use parking_lot::Mutex;
use rodio::{Decoder, MixerDeviceSink, Player};
use std::io::Cursor;

pub struct AudioPlayer {
    handle: &'static mut MixerDeviceSink,
    player: Mutex<Player>,
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
        }
    }
    pub fn upload(&self, bytes: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let file = Cursor::new(bytes);
        let source = Decoder::try_from(file)?;
        let player = self.player.lock();

        player.append(source);

        player.play();

        Ok(())
    }

    pub fn play(&self) {
        self.player.lock().play();
    }

    pub fn pos(&self) -> String {
        let pos = self.player.lock().get_pos();
        format!("{}:{}", pos.as_secs() / 60, pos.as_secs())
    }
}
