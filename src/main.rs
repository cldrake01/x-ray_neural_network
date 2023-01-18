mod songStruct;

use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{SineWave, Source};
use crate::songStruct::{Playlist, Song};

fn main () {

    let mut my_playlist = Playlist {
        name: String::from("Something"),
        songs: vec![],
    };

    my_playlist.add(
        Song {
            plays: 0,
            name: String::from("Air"),
            path: String::from("/Users/collindrake/Downloads/Airborne_Grooves.wav")
        }
    );

    my_playlist.play();
}