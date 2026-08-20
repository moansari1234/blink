use rodio::{Decoder, OutputStream, Sink, Source};
use std::io::Cursor;

const BELL_WAV: &[u8] = include_bytes!("../sounds/bell.wav");

#[derive(Clone)]
pub struct AudioPlayer;

impl AudioPlayer {
    pub fn new() -> Self {
        Self
    }

    pub fn play_chime(&self, volume: f32) {
        let vol = volume.clamp(0.0, 1.0);
        if vol <= 0.001 {
            return;
        }

        std::thread::spawn(move || {
            let _ = Self::play_internal(vol);
        });
    }

    fn play_internal(volume: f32) -> Result<(), String> {
        let (_stream, stream_handle) = OutputStream::try_default().map_err(|e| e.to_string())?;
        let sink = Sink::try_new(&stream_handle).map_err(|e| e.to_string())?;

        let cursor = Cursor::new(BELL_WAV);
        let source = Decoder::new(cursor).map_err(|e| e.to_string())?;
        let amplified = source.amplify(volume);

        sink.append(amplified);
        sink.sleep_until_end();
        Ok(())
    }
}
