mod songStruct;

use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{SineWave, Source};

fn main () {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

// Add a dummy source of the sake of the example.
    let file = BufReader::new(File::open("/Users/collindrake/Downloads/Airborne_Grooves.wav").unwrap());
// Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    sink.append(source);

// The sound plays in a separate thread. This call will block the current thread until the sink
// has finished playing all its queued sounds.
    sink.sleep_until_end();
}