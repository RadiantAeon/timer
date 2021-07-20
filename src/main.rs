use std::time::{Duration, Instant};
use std::thread::sleep;
use text_io::read;
use std::fs::File;
use std::io::{BufReader, self, Write};
use rodio::{Decoder, OutputStream, source::Source};
use std::*;
use std::convert::TryInto;

fn main() {
    
    print!("Length of timer(in seconds): ");
    io::stdout().flush().unwrap();
    
    let time_to_wait: i32 = read!();

    sleep(Duration::new(time_to_wait.try_into().unwrap(), 0));

    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("ringtone.mp3").unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples());

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    sleep(Duration::from_secs(5));
}