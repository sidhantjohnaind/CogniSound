use std::collections::BTreeMap;

pub fn generate_api_sig(params: &BTreeMap<&str, String>, api_secret: &str) -> String {
    let mut sig_base = String::new();
    for (k, v) in params {
        sig_base.push_str(k);
        sig_base.push_str(v);
    }
    sig_base.push_str(api_secret);
    let digest = md5::compute(sig_base.as_bytes());
    format!("{:x}", digest)
}

pub async fn scrobble_to_lastfm(
    api_key: &str,
    api_secret: &str,
    session_key: &str,
    artist: &str,
    track: &str,
    album: &str,
    timestamp: i64,
) -> Result<(), String> {
    let client = reqwest::Client::new();
    let mut params = BTreeMap::new();
    params.insert("method", "track.scrobble".to_string());
    params.insert("api_key", api_key.to_string());
    params.insert("sk", session_key.to_string());
    params.insert("artist", artist.to_string());
    params.insert("track", track.to_string());
    if !album.is_empty() {
        params.insert("album", album.to_string());
    }
    params.insert("timestamp", timestamp.to_string());

    let sig = generate_api_sig(&params, api_secret);
    params.insert("api_sig", sig);
    params.insert("format", "json".to_string());

    let res = client
        .post("https://ws.audioscrobbler.com/2.0/")
        .form(&params)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        println!(" 📻 Last.fm Scrobble Sent: {} - {}", artist, track);
        Ok(())
    } else {
        Err(format!("Last.fm returned HTTP {}", res.status()))
    }
}

pub async fn update_now_playing(
    api_key: &str,
    api_secret: &str,
    session_key: &str,
    artist: &str,
    track: &str,
    album: &str,
) -> Result<(), String> {
    let client = reqwest::Client::new();
    let mut params = BTreeMap::new();
    params.insert("method", "track.updateNowPlaying".to_string());
    params.insert("api_key", api_key.to_string());
    params.insert("sk", session_key.to_string());
    params.insert("artist", artist.to_string());
    params.insert("track", track.to_string());
    if !album.is_empty() {
        params.insert("album", album.to_string());
    }

    let sig = generate_api_sig(&params, api_secret);
    params.insert("api_sig", sig);
    params.insert("format", "json".to_string());

    let res = client
        .post("https://ws.audioscrobbler.com/2.0/")
        .form(&params)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        println!(" 📻 Last.fm Now Playing Sent: {} - {}", artist, track);
        Ok(())
    } else {
        Err(format!("Last.fm returned HTTP {}", res.status()))
    }
}