use protobuf;
use protocol;
use util::{SpotifyId, FileId, StrChunksExt};
use session::Session;
use types::*;
use futures::Future;

pub use protocol::metadata::AudioFile_Format as FileFormat;

fn countrylist_contains(list: &str, country: &str) -> bool {
    list.chunks(2).any(|cc| cc == country)
}

fn country_is_allowed(restriction: &protocol::metadata::Restriction, country: &str) -> bool {

    if restriction.has_countries_allowed() {
        // Whitelist
        countrylist_contains(restriction.get_countries_allowed(), country)
    } else if restriction.has_countries_forbidden() {
        // Blacklist
        !countrylist_contains(restriction.get_countries_forbidden(), country)
    } else {
        // Is this possible ??
        false
    }
}

pub trait Metadata: Sized + Send + 'static {
    type Message: protobuf::MessageStatic + Into<Self>;
    fn base_url() -> &'static str;
    fn id(&self) -> SpotifyId;
}

#[derive(Clone)]
pub struct Track(protocol::metadata::Track);
pub struct Album(protocol::metadata::Album);
pub struct Artist(protocol::metadata::Artist);

impl From<protocol::metadata::Track> for Track {
    fn from(track: protocol::metadata::Track) -> Track {
        Track(track)
    }
}
impl Metadata for Track {
    type Message = protocol::metadata::Track;

    fn base_url() -> &'static str {
        "hm://metadata/3/track"
    }

    fn id(&self) -> SpotifyId {
        SpotifyId::from_raw(self.0.get_gid())
    }
}

impl Track {
    pub fn available(&self, country: &str, catalogue: &str) -> bool {
        self.0
            .get_restriction()
            .into_iter()
            .filter(|r| r.get_catalogue_str().contains(&catalogue.to_owned()))
            .all(|r| country_is_allowed(r, country))
    }

    pub fn files<'a>(&'a self) -> Box<Iterator<Item = (FileId, FileFormat)> + 'a> {
        Box::new(self.0
            .get_file()
            .iter()
            .filter(|file| file.has_file_id())
            .map(|file| {
                let mut dst = [0u8; 20];
                dst.copy_from_slice(&file.get_file_id());
                (FileId(dst), file.get_format())
            }))
    }

    pub fn find_file(&self, format: FileFormat) -> Option<FileId> {
        self.0
            .get_file()
            .iter()
            .filter(|file| file.has_file_id())
            .find(|file| file.get_format() == format)
            .map(|file| {
                let mut dst = [0u8; 20];
                dst.copy_from_slice(&file.get_file_id());
                FileId(dst)
            })
    }

    pub fn alternatives<'a>(&'a self) -> Box<Iterator<Item = SpotifyId> + 'a> {
        Box::new(self.0
            .get_alternative()
            .iter()
            .map(protocol::metadata::Track::get_gid)
            .map(SpotifyId::from_raw))
    }

    pub fn find_alternative(&self, country: &str, catalogue: &str) -> Option<SpotifyId> {
        if self.available(country, catalogue) {
            Some(self.id())
        } else {
            self.0
                .get_alternative()
                .iter()
                .cloned()
                .map(Track::from)
                .find(|track| track.available(country, catalogue))
                .map(|track| track.id())
        }
    }
}

impl From<protocol::metadata::Album> for Album {
    fn from(album: protocol::metadata::Album) -> Album {
        Album(album)
    }
}
impl Metadata for Album {
    type Message = protocol::metadata::Album;

    fn base_url() -> &'static str {
        "hm://metadata/3/album"
    }

    fn id(&self) -> SpotifyId {
        SpotifyId::from_raw(self.0.get_gid())
    }
}

impl Album {
    pub fn tracks<'a>(&'a self) -> Box<Iterator<Item = SpotifyId> + 'a> {
        Box::new(self.0
            .get_disc()
            .iter()
            .flat_map(|disc| disc.get_track())
            .filter(|track| track.has_gid())
            .map(|track| SpotifyId::from_raw(track.get_gid())))
    }
}

// #[derive(Debug, Clone)]
// pub struct Track {
// pub id: SpotifyId,
// pub name: String,
// pub album: SpotifyId,
// pub artists: Vec<SpotifyId>,
// pub files: LinearMap<FileFormat, FileId>,
// pub alternatives: Vec<SpotifyId>,
// pub available: bool,
// }
//
// #[derive(Debug, Clone)]
// pub struct Album {
// pub id: SpotifyId,
// pub name: String,
// pub artists: Vec<SpotifyId>,
// pub tracks: Vec<SpotifyId>,
// pub covers: Vec<FileId>,
// }
//
// #[derive(Debug, Clone)]
// pub struct Artist {
// pub id: SpotifyId,
// pub name: String,
// pub top_tracks: Vec<SpotifyId>,
// }
//
// impl Metadata for Track {
// type Message = protocol::metadata::Track;
//
// fn base_url() -> &'static str {
// "hm://metadata/3/track"
// }
//
// fn parse(msg: &Self::Message, session: &Session) -> Self {
// let country = session.country();
//
// let artists = msg.get_artist()
// .iter()
// .filter(|artist| artist.has_gid())
// .map(|artist| SpotifyId::from_raw(artist.get_gid()))
// .collect::<Vec<_>>();
//
// let files = msg.get_file()
// .iter()
// .filter(|file| file.has_file_id())
// .map(|file| {
// let mut dst = [0u8; 20];
// dst.clone_from_slice(&file.get_file_id());
// (file.get_format(), FileId(dst))
// })
// .collect();
//
// Track {
// id: SpotifyId::from_raw(msg.get_gid()),
// name: msg.get_name().to_owned(),
// album: SpotifyId::from_raw(msg.get_album().get_gid()),
// artists: artists,
// files: files,
// alternatives: msg.get_alternative()
// .iter()
// .map(|alt| SpotifyId::from_raw(alt.get_gid()))
// .collect(),
// available: parse_restrictions(msg.get_restriction(),
// &country,
// "premium"),
// }
// }
// }
//
// impl Metadata for Album {
// type Message = protocol::metadata::Album;
//
// fn base_url() -> &'static str {
// "hm://metadata/3/album"
// }
//
// fn parse(msg: &Self::Message, _: &Session) -> Self {
// let artists = msg.get_artist()
// .iter()
// .filter(|artist| artist.has_gid())
// .map(|artist| SpotifyId::from_raw(artist.get_gid()))
// .collect::<Vec<_>>();
//
// let tracks = msg.get_disc()
// .iter()
// .flat_map(|disc| disc.get_track())
// .filter(|track| track.has_gid())
// .map(|track| SpotifyId::from_raw(track.get_gid()))
// .collect::<Vec<_>>();
//
// let covers = msg.get_cover_group()
// .get_image()
// .iter()
// .filter(|image| image.has_file_id())
// .map(|image| {
// let mut dst = [0u8; 20];
// dst.clone_from_slice(&image.get_file_id());
// FileId(dst)
// })
// .collect::<Vec<_>>();
//
// Album {
// id: SpotifyId::from_raw(msg.get_gid()),
// name: msg.get_name().to_owned(),
// artists: artists,
// tracks: tracks,
// covers: covers,
// }
// }
// }
//
//
// impl Metadata for Artist {
// type Message = protocol::metadata::Artist;
//
// fn base_url() -> &'static str {
// "hm://metadata/3/artist"
// }
//
// fn parse(msg: &Self::Message, session: &Session) -> Self {
// let country = session.country();
//
// let top_tracks = msg.get_top_track()
// .iter()
// .filter(|tt| !tt.has_country() ||
// countrylist_contains(tt.get_country(), &country))
// .next()
// .unwrap()
// .get_track()
// .iter()
// .filter(|track| track.has_gid())
// .map(|track| SpotifyId::from_raw(track.get_gid()))
// .collect::<Vec<_>>();
//
// Artist {
// id: SpotifyId::from_raw(msg.get_gid()),
// name: msg.get_name().to_owned(),
// top_tracks: top_tracks
// }
// }
// }
//

pub fn get<'a, T: Metadata>(session: &Session, id: SpotifyId) -> SpFuture<'a, T> {
    let uri = format!("{}/{}", T::base_url(), id.to_base16());
    session
        .mercury()
        .get(uri)
        .and_then(move |response| {
            let data = response.payload.first().expect("Empty payload");
            let msg: T::Message = protobuf::parse_from_bytes(data)?;
            Ok(msg.into())
        })
        .sp_boxed()
}
