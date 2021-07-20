use std::time::Duration;
use std::thread::sleep;
use text_io::read;
use std::fs::File;
use std::io::{BufReader, self, Write};
use rodio::{Decoder, OutputStream, source::Source};
use std::*;
use std::convert::TryInto;
use std::io::prelude::*;

fn main() {
    let mut stdin = io::stdin();
    
    print!("Length of timer(in seconds): ");
    io::stdout().flush().unwrap();
    
    let time_to_wait: i32 = read!();

    // sleep for the timer duration
    sleep(Duration::new(time_to_wait.try_into().unwrap(), 0));


    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Load a sound from a file, using a path relative to home
    let ringtone_file;
    match home::home_dir() {
        Some(path) => ringtone_file = BufReader::new(File::open(path.display().to_string() + "/ringtone.mp3").unwrap()),
        None => { 
            println!("Impossible to get your home dir!");
            process::exit(0);
        },
    };

    // Decode that sound file into a source
    let source = Decoder::new(ringtone_file).unwrap();
    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples());

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    print!("Timer of {} seconds completed! Press any key to exit:", time_to_wait);
    io::stdout().flush().unwrap();

    // Read a single byte and discard
    stdin.read(&mut [0u8]).unwrap();
}