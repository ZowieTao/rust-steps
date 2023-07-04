extern crate rodio;
use rodio::source::Source;
use std::io::{stdin, stdout, Write};
use std::thread;
use std::time::Duration;

fn main() {
    // create audio device
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    loop {
        // Create a new audio device and play queue
        let sink = rodio::Sink::try_new(&stream_handle).unwrap();

        // play beep
        play_beep_sound(&sink);

        // waiting for user input
        println!("Enter any character to continue, enter q to exit...");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        if input.trim() == "q" {
            break;
        }
    }
}

fn play_beep_sound(sink: &rodio::Sink) {
    // Create audio stream
    let source = rodio::source::SineWave::new(1000).take_duration(Duration::from_secs(1));

    // Add the audio stream to the playback queue
    sink.append(source);

    // wait for playback to complete
    thread::sleep(Duration::from_secs(1));
}
