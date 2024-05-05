use rusqlite::{params, Connection, Result};

mod library;

struct LocalDevice {
    dbc: Connection,
}

impl LocalDevice {
    fn open(dbc: Connection) -> Result<LocalDevice> {
        // let mut stmt = dbc.prepare("SELECT * FROM libraries")?;

        // stmt.query_map([], |row| {

        // })?;
        Ok(LocalDevice { dbc })
    }
}

////
mod _old {
struct Library {
    tracks: Vec<AudioTrack>,
}

struct AudioTrack {
    metadata: Metadata,
    playback_stream: PlaybackStream, 
}

enum Metadata {
    Standard(StandardFields),
    Custom(CustomFields),
}

struct StandardFields {
    artist_name: Option<String>,
    track_title: Option<String>,
    album_title: Option<String>,
    date: Option<String>,
    genre: Option<String>,
    composer: Option<String>,
    performer: Option<String>,
    album_artist: Option<String>,
    track_number: Option<String>,
    total_tracks: Option<String>,
    disc_number: Option<String>,
    total_discs: Option<String>,
    comment: Option<String>,
}

struct CustomFields {
    /*TODO*/
}

enum PlaybackStream {
    SingleFile(AudioFile),
    TimeSpanInFile {
        file: AudioFile,
        span: (/*TODO*/),
    },
}

struct AudioFile {
    hash: u64,
    path: String,
}
}