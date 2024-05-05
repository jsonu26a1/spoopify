use std::marker::PhantomData;

pub mod schema;

pub type Key = i64;

pub struct ForiegnKey<T> {
    _pd: PhantomData<Box<T>>,
    id: Option<Key>,
}

pub struct SourceBase {
    id: Key,
    protocol: String,
    host: String,
    path: String,
}

pub struct Source {
    id: Key,
    base: ForiegnKey<SourceBase>,
    path: String,
}

pub struct Track {
    id: Key,
    title: String,
    album: ForiegnKey<Album>,
    number: String,
    artist: ForiegnKey<Group>,
}

pub struct TrackSource {
    track: ForiegnKey<Track>,
    source: ForiegnKey<Source>,
    priority: i32,
    time_start: f64,
    time_end: f64,
}

pub struct TrackTag {
    track: ForiegnKey<Track>,
    tag: ForiegnKey<Tag>,
}

pub struct TrackRole {
    track: ForiegnKey<Track>,
    role: String,
    artist: ForiegnKey<Group>,
}

pub struct Album {
    id: Key,
    title: String,
    artist: ForiegnKey<Group>,
}

pub struct AlbumTag {
    album: ForiegnKey<Album>,
    tag: ForiegnKey<Tag>,
}

pub struct Group {
    id: Key,
    name: String,
}

pub struct GroupTag {
    group: ForiegnKey<Group>,
    tag: ForiegnKey<Tag>,
}

pub struct GroupMember {
    group: ForiegnKey<Group>,
    role: String,
    member: ForiegnKey<Group>,
}

pub struct TagCategory {
    id: Key,
    name: String,
}

pub struct Tag {
    id: Key,
    category: ForiegnKey<TagCategory>,
    value: String,
}

pub struct Image {
    id: Key,
    source: ForiegnKey<Source>,
    left: i32,
    top: i32,
    width: i32,
    height: i32,
}

pub struct ImageTag {
    image: ForiegnKey<Image>,
    tag: ForiegnKey<Tag>,
    priority: i32,
}
