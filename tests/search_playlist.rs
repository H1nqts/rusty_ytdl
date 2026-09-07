#[tokio::test]
async fn search_playlist() {
    use rusty_ytdl::search::{Playlist, PlaylistSearchOptions};

    let playlist = Playlist::get(
        "https://www.youtube.com/playlist?list=PLwMEL7UNT4o9iMzrvNBXZqXbNPFfT6rVD",
        Some(&PlaylistSearchOptions {
            limit: 6000,
            fetch_all: false,
            ..Default::default()
        }),
    )
    .await;

    let playlist = playlist.expect("playlist request failed");

    println!("videos: {}", playlist.videos.len());
    println!("skipped: {}", playlist.skipped.len());
    println!("{:#?}", playlist.skipped);
    println!("fetch stopped: {:?}", playlist.fetch_stopped);

    assert!(
        !playlist.videos.is_empty(),
        "no video parsed, skipped: {:#?}",
        playlist.skipped
    );
}
