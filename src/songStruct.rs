pub struct Song {
    plays: i32,
    name: String,
    path: String
}

pub struct Playlist {
    name: String,
    songs: Vec<Song>
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
}

impl Playlist {
    pub fn playlist(mut self, songs: &[Song]) {
        for song in songs {
            self.songs.push(song);
        }
    }

    pub fn add(&mut self, new_song: Song) {
        self.songs.push(new_song);
    }
}