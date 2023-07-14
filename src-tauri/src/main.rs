// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod rssread;
use std::collections::HashMap;

use rssread::{FeedItem, RssReader};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct FeedState {
    feeditems: Vec<FeedItem>,
    messege: String,
}
#[derive(Deserialize, Serialize, Debug)]
struct SearchToken {
    selected_genre: String,
    rss_urls: HashMap<String, Vec<String>>,
    search_word: String,
}
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_feeds(search_token: SearchToken) -> FeedState {
    let mut rssreader = RssReader::new().unwrap();
    rssreader.selected_genre = search_token.selected_genre;
    rssreader.search_word = search_token.search_word.trim().to_owned();

    match rssreader.getfeed(search_token.rss_urls) {
        Ok(_) => {
            if !(rssreader.feeds == rssreader.prefeeds) {
                rssreader.savefeed().unwrap()
            }
        }
        Err(e) => rssreader.status_message = e.to_string(),
    };

    if !rssreader.search_word.is_empty() {
        if let Err(e) = rssreader.search_word() {
            rssreader.status_message = e.to_string()
        }
    }

    FeedState {
        feeditems: rssreader.feeds,
        messege: rssreader.status_message,
    }
}
#[tauri::command]
fn get_urls() -> HashMap<String, Vec<String>> {
    let mut rssreader = RssReader::new().unwrap();
    rssreader.geturls().unwrap()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_feeds, get_urls])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
