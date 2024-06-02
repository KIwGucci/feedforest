// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod rssread;
use rssread::{FeedItem, RssReader};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
struct FeedState {
    feeditems: Vec<FeedItem>,
    message: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct SearchToken {
    selected_genre: String,
    rss_urls: HashMap<String, Vec<String>>,
    search_word: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn get_feeds(search_token: SearchToken, from_stock: bool) -> FeedState {
    let mut rssreader = RssReader::new().unwrap();

    rssreader.selected_genre = search_token.selected_genre;
    search_token
        .search_word
        .trim()
        .clone_into(&mut rssreader.search_word);

    if !rssreader.search_word.is_empty() || from_stock {
        match rssreader.read_feed() {
            Ok(feeds) => {
                rssreader.feeds = feeds;

                // rssreaderのfeedをsearch word で filter処理
                if let Err(e) = rssreader.filter_word() {
                    rssreader.status_message = e.to_string()
                }

                rssreader.status_message = format!("Stock feed is {}", rssreader.feeds.len());
                rssreader.feeds.sort_by_key(|item| item.date.to_owned());
                rssreader.feeds.reverse();
            }
            Err(e) => rssreader.status_message = e.to_string(),
        };
    } else {
        match rssreader.getfeed(&search_token.rss_urls) {
            Ok(_) => rssreader.savefeed().unwrap(),
            Err(e) => rssreader.status_message = e.to_string(),
        };
    }

    // feedが一定数以上あればカット
    rssreader.feeds.truncate(1000);

    FeedState {
        feeditems: rssreader.feeds,
        message: rssreader.status_message,
    }
}
#[tauri::command]
fn get_urls() -> HashMap<String, Vec<String>> {
    let mut rssreader = RssReader::new().unwrap();
    rssreader.geturls().unwrap()
}

#[tauri::command(async)]
async fn get_all_feeds() {
    let mut rssreader = RssReader::new().unwrap();
    let url_dic = rssreader.geturls().unwrap();
    let genres = url_dic.keys();

    for genre in genres {
        rssreader.selected_genre = genre.to_string();
        if rssreader.getfeed(&url_dic.to_owned()).is_ok() {
            rssreader.savefeed().unwrap()
        };
    }
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_feeds, get_urls, get_all_feeds])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
