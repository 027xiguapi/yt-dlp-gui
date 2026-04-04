use crate::logger;
use crate::models::{RssFeedPreview, RssFeedPreviewItem};
use regex::Regex;

#[tauri::command]
pub async fn parse_rss_feed(url: String) -> Result<RssFeedPreview, String> {
    logger::log_to_file(&format!("Fetching RSS feed: {}", url));

    let client = reqwest::Client::builder()
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .send()
        .await
        .map_err(|e| format!("Failed to fetch RSS feed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()));
    }

    let xml = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response body: {}", e))?;

    parse_youtube_atom(&xml, &url)
}

fn first_match(text: &str, pattern: &str) -> String {
    let re = Regex::new(pattern).unwrap_or_else(|_| Regex::new("$^").unwrap());
    re.captures(text)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().trim().to_string()))
        .unwrap_or_default()
}

fn parse_youtube_atom(xml: &str, feed_url: &str) -> Result<RssFeedPreview, String> {
    let channel_id = first_match(xml, r"<yt:channelId>([^<]+)</yt:channelId>");

    // Extract feed-level <title> (before the first <entry>)
    let title = {
        let entry_pos = xml.find("<entry>").unwrap_or(xml.len());
        let header = &xml[..entry_pos];
        let cdata_title = first_match(header, r"<title><!\[CDATA\[([\s\S]*?)\]\]></title>");
        if !cdata_title.is_empty() {
            cdata_title
        } else {
            first_match(header, r"<title>([^<]+)</title>")
        }
    };

    let mut items: Vec<RssFeedPreviewItem> = Vec::new();

    // Extract entries
    let entry_re = Regex::new(r"<entry>([\s\S]*?)</entry>").map_err(|e| format!("Regex error: {}", e))?;
    for entry_caps in entry_re.captures_iter(xml) {
        let entry = &entry_caps[1];

        let video_id = first_match(entry, r"<yt:videoId>([^<]+)</yt:videoId>");
        if video_id.is_empty() {
            continue;
        }

        let item_title = {
            let cdata_title = first_match(entry, r"<title[^>]*><!\[CDATA\[([\s\S]*?)\]\]></title>");
            if !cdata_title.is_empty() {
                cdata_title
            } else {
                first_match(entry, r"<title[^>]*>([^<]+)</title>")
            }
        };

        let item_url = {
            let url = first_match(entry, r#"<link[^>]+href=['"]([^'"]+)['"][^>]*>"#);
            if !url.is_empty() {
                url
            } else {
                format!("https://www.youtube.com/watch?v={}", video_id)
            }
        };

        let item_published = first_match(entry, r"<published>([^<]+)</published>");

        let item_thumbnail = first_match(entry, r#"<media:thumbnail[^>]+url=['"]([^'"]+)['"][^>]*>"#);

        items.push(RssFeedPreviewItem {
            video_id: video_id.clone(),
            title: item_title,
            url: item_url,
            thumbnail: item_thumbnail,
            published_at: item_published,
        });
    }

    let thumbnail = items.first().map(|i| i.thumbnail.clone()).unwrap_or_default();

    logger::log_to_file(&format!(
        "Parsed RSS feed: {} with {} items",
        title,
        items.len()
    ));

    Ok(RssFeedPreview {
        channel_id,
        title,
        url: feed_url.to_string(),
        thumbnail,
        description: String::new(),
        items,
    })
}
