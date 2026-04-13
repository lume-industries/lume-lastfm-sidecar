use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TrackRow {
    pub song: String,
    pub artist: String,
    pub album: String,
    pub status: String,
    pub played_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LastfmPayload {
    pub username: String,
    pub updated: String,
    pub tracks: Vec<TrackRow>,
}

#[derive(Deserialize)]
struct LastfmResponse {
    #[serde(default)]
    error: Option<i32>,
    #[serde(default)]
    message: Option<String>,
    recenttracks: Option<RecentTracks>,
}

#[derive(Deserialize)]
struct RecentTracks {
    track: TrackField,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum TrackField {
    Many(Vec<TrackEntry>),
    One(TrackEntry),
}

#[derive(Deserialize)]
struct TrackEntry {
    #[serde(default)]
    name: String,
    artist: TextNode,
    album: TextNode,
    #[serde(rename = "@attr", default)]
    attr: Option<TrackAttr>,
    #[serde(default)]
    date: Option<DateNode>,
}

#[derive(Deserialize)]
struct TextNode {
    #[serde(rename = "#text", default)]
    text: String,
}

#[derive(Deserialize)]
struct TrackAttr {
    #[serde(rename = "nowplaying", default)]
    now_playing: Option<String>,
}

#[derive(Deserialize)]
struct DateNode {
    #[serde(default)]
    uts: Option<String>,
}

fn format_played_at(epoch_secs: u64) -> String {
    let days = epoch_secs / 86_400;
    let seconds_today = epoch_secs % 86_400;
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = mp + if mp < 10 { 3 } else { -9 };
    let year = year + if month <= 2 { 1 } else { 0 };
    let hour = (seconds_today / 3_600) as u8;
    let minute = ((seconds_today / 60) % 60) as u8;
    format!("{:02}/{:02} {:02}:{:02} UTC", day, month, hour, minute)
}

pub fn parse_recent_tracks(
    username: &str,
    body: &str,
    now_secs: u64,
) -> Result<LastfmPayload, String> {
    let response: LastfmResponse =
        serde_json::from_str(body).map_err(|error| format!("invalid Last.fm JSON: {error}"))?;
    if let Some(code) = response.error {
        return Err(format!(
            "Last.fm API error {code}: {}",
            response.message.unwrap_or_else(|| "unknown".to_string())
        ));
    }

    let tracks = response
        .recenttracks
        .map(|recent| match recent.track {
            TrackField::Many(tracks) => tracks,
            TrackField::One(track) => vec![track],
        })
        .unwrap_or_default()
        .into_iter()
        .map(|track| TrackRow {
            song: truncate(&track.name, 28),
            artist: truncate(&track.artist.text, 22),
            album: truncate(&track.album.text, 22),
            status: if track
                .attr
                .as_ref()
                .and_then(|attr| attr.now_playing.as_deref())
                .is_some()
            {
                "now playing".to_string()
            } else {
                "recent".to_string()
            },
            played_at: track
                .date
                .and_then(|date| date.uts)
                .and_then(|uts| uts.parse::<u64>().ok())
                .map(format_played_at)
                .unwrap_or_default(),
        })
        .collect();

    let hh = (now_secs % 86400) / 3600;
    let mm = (now_secs % 3600) / 60;
    Ok(LastfmPayload {
        username: username.to_string(),
        updated: format!("Updated {:02}:{:02}", hh, mm),
        tracks,
    })
}

fn truncate(text: &str, max_len: usize) -> String {
    if text.chars().count() <= max_len {
        return text.to_string();
    }
    let mut shortened = String::new();
    for ch in text.chars().take(max_len.saturating_sub(3)) {
        shortened.push(ch);
    }
    shortened.push_str("...");
    shortened
}
