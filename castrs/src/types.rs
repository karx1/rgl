use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Episode {
    link: String,
    audio: String,
    thumbnail: String,
    pub_date_ms: u64,
    title_original: String,
    explicit_content: bool,
    description_original: String,
    podcast: Podcast,
}

#[derive(Serialize, Deserialize)]
pub struct Podcast {
    website: String,
    thumbnail: String,
    title_original: String,
    total_episodes: String,
    explicit_content: bool,
    latest_pub_date_ms: String,
    publisher_original: String,
    description_original: String
}

#[derive(Serialize, Deserialize)]
pub struct Curated {
    podcasts: Vec<Podcast>,
    source_url: String,
    pub_date_ms: u64,
    title_original: String,
    description_original: String
}

pub enum {
    Episode(Episode),
    Podcast(Podcast),
    Curated(Curated)
}
