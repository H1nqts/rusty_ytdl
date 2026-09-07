use rusty_ytdl::search::{Playlist, PlaylistSearchOptions};

/// Playlist whose default listing leaves some entries out.
const HIDES_ENTRIES: &str = "https://www.youtube.com/playlist?list=PLhUsv4pXBJk7mnB1o6PZA8IyF9v9GnxY9";

async fn get(url: &str, include_unavailable: bool) -> Playlist {
    Playlist::get(
        url,
        Some(&PlaylistSearchOptions {
            limit: 6000,
            fetch_all: true,
            include_unavailable,
            ..Default::default()
        }),
    )
    .await
    .expect("playlist request failed")
}

fn report(label: &str, playlist: &Playlist) {
    println!("[{label}] videos: {}", playlist.videos.len());
    println!("[{label}] skipped: {}", playlist.skipped.len());
    println!("[{label}] fetch stopped: {:?}", playlist.fetch_stopped);

    for entry in playlist.skipped.iter() {
        println!("[{label}]   {entry:?}");
    }
}

#[tokio::test]
async fn search_playlist() {
    let playlist = get(
        "https://www.youtube.com/playlist?list=PLwMEL7UNT4o9iMzrvNBXZqXbNPFfT6rVD",
        false,
    )
    .await;

    report("default", &playlist);

    assert!(
        !playlist.videos.is_empty(),
        "no video parsed, skipped: {:#?}",
        playlist.skipped
    );
}

#[tokio::test]
async fn search_playlist_including_unavailable() {
    let default = get(HIDES_ENTRIES, false).await;
    let including = get(HIDES_ENTRIES, true).await;

    report("default", &default);
    report("including", &including);

    assert!(
        including.videos.len() > default.videos.len(),
        "expected hidden entries on top of the {} default ones, got {}",
        default.videos.len(),
        including.videos.len()
    );

    let listed: std::collections::HashSet<&str> =
        default.videos.iter().map(|x| x.id.as_str()).collect();

    let hidden = including
        .videos
        .iter()
        .filter(|x| !listed.contains(x.id.as_str()))
        .count();

    println!("hidden entries recovered: {hidden}");

    // The default listing must stay a subset of the full one.
    let full: std::collections::HashSet<&str> =
        including.videos.iter().map(|x| x.id.as_str()).collect();

    let dropped = listed.difference(&full).collect::<Vec<_>>();
    assert!(dropped.is_empty(), "entries lost when including hidden: {dropped:?}");
}
