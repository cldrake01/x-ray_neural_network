mod playlist_manager;

use crate::playlist_manager::{Playlist, Song};

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