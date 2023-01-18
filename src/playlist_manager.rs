use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};

pub struct Song {
    pub(crate) plays: i32,
    pub(crate) name: String,
    pub(crate) path: String,
}

pub struct Playlist {
    pub(crate) name: String,
    pub(crate) songs: Vec<Song>,
}

impl Song {
    pub fn set(&mut self, plays: i32, name: String, path: String) {
        self.plays = plays;
        self.name = name;
        self.path = path;
    }

    pub fn get(&self) -> (i32, String, String) {
        (self.plays, self.name.clone(), self.path.clone())
    }

    pub fn play (self) {

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new( & stream_handle).unwrap();

        // Add a dummy source of the sake of the example.
        let file = BufReader::new(File::open(self.path).unwrap());
        // Decode that sound file into a source
        let source = Decoder::new(file).unwrap();
        sink.append(source);

        // The sound plays in a separate thread. This call will block the current thread until the sink
        // has finished playing all its queued sounds.
        sink.sleep_until_end();

    }
}

impl Playlist {
    pub fn playlist(mut self, songs: Vec<Song>) {
        for song in songs {
            self.songs.push(song);
        }
    }

    pub fn add(&mut self, new_song: Song) {
        self.songs.push(new_song);
    }

    pub fn play (self) {
        for _song in self.songs {
            _song.play();
        }
    }
}