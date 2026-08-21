use rodio::{Decoder, OutputStream, Sink, Source};
use std::io::{BufReader, Cursor};
use std::path::Path;

const BELL_WAV: &[u8] = include_bytes!("../sounds/bell.wav");

#[derive(Clone)]
pub struct AudioPlayer;

impl AudioPlayer {
    pub fn new() -> Self {
        Self
    }

    pub fn play_chime(&self, volume: f32, custom_path: Option<&str>) {
        let vol = volume.clamp(0.0, 1.0);
        if vol <= 0.001 {
            return;
        }

        let custom_str = custom_path.map(|s| s.to_string());
        std::thread::spawn(move || {
            let _ = Self::play_internal(vol, custom_str);
        });
    }

    fn play_internal(volume: f32, custom_path: Option<String>) -> Result<(), String> {
        let (_stream, stream_handle) = OutputStream::try_default().map_err(|e| e.to_string())?;
        let sink = Sink::try_new(&stream_handle).map_err(|e| e.to_string())?;

        let mut played_custom = false;
        if let Some(ref path_str) = custom_path {
            let p = Path::new(path_str);
            if p.exists() && p.is_file() {
                if let Ok(file) = std::fs::File::open(p) {
                    let reader = BufReader::new(file);
                    if let Ok(source) = Decoder::new(reader) {
                        sink.append(source.amplify(volume));
                        played_custom = true;
                    }
                }
            }
        }

        if !played_custom {
            let cursor = Cursor::new(BELL_WAV);
            let source = Decoder::new(cursor).map_err(|e| e.to_string())?;
            sink.append(source.amplify(volume));
        }

        sink.sleep_until_end();
        Ok(())
    }
}
