//! One GitHub response feeds the release pages and the update check: one request, both answers,
//! and an error that reaches both.
use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};

fn fixture() -> String {
    let list: Vec<_> = (1..=5)
        .map(|i| {
            serde_json::json!({
                "tag_name": format!("v2026.10.14.{i}"),
                "name": format!("Build {i}"), "body": "Release notes\r\n".repeat(256),
                "published_at": "2026-10-14T12:00:00Z", "draft": false,
                "assets": [{"name": ASSET.unwrap_or("unsupported"),
                    "browser_download_url": "https://example.test/game", "size": 12345}]
            })
        })
        .collect();
    serde_json::to_string(&list).unwrap()
}

#[test]
fn one_request_feeds_releases_and_the_update() {
    let text = fixture();
    let requests = AtomicUsize::new(0);
    let mut published = None;
    let updates = releases_and_updates_with(
        || {
            requests.fetch_add(1, Ordering::Relaxed);
            Ok(serde_json::from_str(&text).unwrap())
        },
        Vec::new,
        |r| published = Some(r.unwrap()),
    )
    .unwrap();
    assert_eq!(requests.load(Ordering::Relaxed), 1);
    let releases = published.unwrap();
    assert_eq!(releases.len(), 5);
    // Nothing installed: the newest release of the five is what the update check offers, and it
    // is one of the releases the pages got.
    let offered = updates.horizon.expect("an update for a fresh launcher");
    assert!(releases.iter().any(|r| r.tag == offered.tag), "{offered:?}");
}

#[test]
fn an_error_reaches_releases_and_updates_without_a_scan() {
    for error in [
        "No connection: offline",
        "GitHub's rate limit was reached.",
        "invalid JSON",
    ] {
        let mut published = None;
        let result = releases_and_updates_with(
            || Err(error.into()),
            || panic!("no installed scan after a failed request"),
            |r| published = Some(r),
        );
        assert_eq!(published.unwrap().unwrap_err(), error);
        assert_eq!(result.unwrap_err(), error);
    }
}
