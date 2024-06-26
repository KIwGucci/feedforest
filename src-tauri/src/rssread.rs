use bytes::Bytes;
use chrono::prelude::*;
use rss::Channel;
use serde_derive::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::{BufReader, BufWriter};
use std::iter::FromIterator;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread::spawn;

// ホームディレクトリ以下の相対パスで記述

// jsonファイルが格納されているフォルダ
const FOLDER_FOR_JSON: &str = "Coding/Database/rssreader/jsondata";
// 全体設定ファイルのファイル名
const SETTING_FILE: &str = "Coding/Database/rssreader/feedsetting.json";
const FEED_URLS: &str = "Coding/Database/rssreader/feedurls.json";

// Feed情報を格納する構造体
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct FeedItem {
    pub title: String,
    pub link: String,
    pub date: String,
}

impl FeedItem {
    fn new(title: String, link: String, date: String) -> Self {
        Self { title, link, date }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SettingItem {
    save_maxsize: usize,
    skip_words: Vec<String>,
    skip_link: Vec<String>,
    replace_words: Vec<Vec<String>>,
}

impl SettingItem {
    fn readsetting() -> Result<Self, Box<dyn Error>> {
        /*
        設定値をJsonファイルから読み出す
        */
        let mut filepath = dirs::home_dir().unwrap();
        filepath.push(SETTING_FILE);
        let f = fs::File::open(filepath)?;
        let buffer = BufReader::new(f);
        let result: Self = serde_json::from_reader(buffer)?;
        Ok(result)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RssReader {
    pub feed_genres: Vec<String>,
    pub selected_genre: String,
    pub search_word: String,
    pub feeds: Vec<FeedItem>,
    pub setting_item: SettingItem,
    pub status_message: String,
}

impl RssReader {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut new_reader = Self {
            feed_genres: Vec::new(),
            selected_genre: String::new(),
            search_word: String::new(),
            feeds: Vec::new(),
            setting_item: SettingItem::readsetting().expect("setting file not found."),
            status_message: String::new(),
        };

        new_reader.geturls()?;
        new_reader
            .selected_genre
            .clone_from(&new_reader.feed_genres[0]);
        Ok(new_reader)
    }

    pub fn geturls(&mut self) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
        // jsonファイルからURL情報を呼び出す
        let mut feed_urls_path = dirs::home_dir().unwrap();
        let mut result = HashMap::new();
        feed_urls_path.push(FEED_URLS);

        let f = fs::File::open(feed_urls_path)?;
        let buffer = BufReader::new(f);
        let my_rssurls: Vec<(String, Vec<String>)> = serde_json::from_reader(buffer)?;

        self.feed_genres = my_rssurls.iter().map(|x| x.0.trim().to_owned()).collect();

        my_rssurls.iter().for_each(|rurls| {
            result.insert(rurls.0.trim().to_owned(), rurls.1.to_owned());
        });

        Ok(result)
    }

    fn get_oldfile_path(&self) -> PathBuf {
        let mut oldfile_path = dirs::home_dir().unwrap();
        oldfile_path.push(FOLDER_FOR_JSON);
        let oldfilename = format!("{}_old", &self.selected_genre);
        oldfile_path.push(oldfilename);
        oldfile_path.set_extension("json");
        oldfile_path
    }

    pub fn read_feed(&mut self) -> Result<Vec<FeedItem>, Box<dyn Error>> {
        // 選択したジャンルのjson化したファイルからfeedを読み込み過去のFeedと閲覧済みのFeedの配列を返す
        let oldfile_path = self.get_oldfile_path();

        let feeds = read_from_json(&oldfile_path)?;

        Ok(feeds)
    }

    pub fn savefeed(&self) -> Result<(), Box<dyn Error>> {
        let oldfile_path = self.get_oldfile_path();
        let save_maxsize = self.setting_item.save_maxsize;
        let save_feed = self.feeds.to_vec();

        match read_from_json(&oldfile_path) {
            Ok(oldfeeds) => {
                if oldfeeds != save_feed {
                    save_to_json(&oldfile_path, save_feed, save_maxsize)?;
                }
            }
            Err(_) => {
                save_to_json(&oldfile_path, save_feed, save_maxsize)?;
            }
        }

        Ok(())
    }

    pub fn filter_word(&mut self) -> Result<(), Box<dyn Error>> {
        let is_match = |item: &FeedItem| {
            let swords = self.search_word.split_whitespace();

            for word in swords {
                if !item.title.to_lowercase().contains(&word.to_lowercase()) {
                    return false;
                }
            }
            true
        };

        let filtered_feed = self
            .feeds
            .iter()
            .filter(|x| is_match(x))
            .map(|x| x.to_owned());

        self.feeds = Vec::from_iter(filtered_feed);

        Ok(())
    }

    pub fn getfeed(&mut self, myurls: &HashMap<String, Vec<String>>) -> Result<(), Box<dyn Error>> {
        // myurls:設定ファイルに記述したrss urlのHashMap
        // skipwords: スキップする単語群
        // // 記事フィードを取得してfeeds:Vec<FeedItem>に格納

        // jsonファイルから既存feedを読み込み
        if let Ok(oldfeed) = self.read_feed() {
            self.feeds = oldfeed;
        } else {
            self.feeds.clear();
        };
        // 分割スレッド数
        let thread_num = 4;

        // 獲得したrssコンテンツを格納するArc Vec
        let webdata = Arc::new(Mutex::new(Vec::new()));

        //選択されたジャンルのurl
        let selected_urls = &myurls[&self.selected_genre];
        let selected_length = selected_urls.len();

        // 生成したスレッドを格納するVec
        let mut threads = Vec::new();
        // １スレッドが受け持つurlの数
        let url_num = selected_urls.len() / thread_num;

        for i in 0..thread_num {
            let start = i * url_num;
            let end = (start + url_num).min(selected_length);
            let webdata = Arc::clone(&webdata);
            let div_urls = selected_urls[start..end].to_vec();

            threads.push(spawn(move || {
                for rss_url in &div_urls {
                    let content = reqwest::blocking::get(rss_url);
                    if let Ok(res) = content {
                        let mut webdata = webdata.lock().expect("Lock Error");
                        let rss_url = rss_url.to_string();
                        webdata.push((res.bytes().unwrap(), rss_url));
                    }
                }
            }));
        }

        // 並列でスレッドのjoin処理
        threads.into_iter().for_each(|th| {
            th.join().unwrap();
        });
        // 獲得したコンテンツをVecに格納
        let contents = webdata.lock().unwrap().to_vec();
        // Rss形式のデータを整形して self.feedsに格納
        self.rss_channel_read(contents)?;

        // 獲得したFeedを日付でソート
        self.feeds.sort_by_cached_key(|k| k.date.to_owned());
        self.feeds.reverse();

        Ok(())
    }

    fn rss_channel_read(&mut self, contents: Vec<(Bytes, String)>) -> Result<(), Box<dyn Error>> {
        // rss形式のcontentをFeedItem形式に落とし込み、self.feeds:Vec<FeedItem>に格納
        contents.into_iter().for_each(|(content, _rss_url)| {
            let rss_channel;

            match Channel::read_from(content.as_ref()) {
                Ok(c) => {
                    rss_channel = c;

                    let rss_channel_iter = rss_channel
                        .items()
                        .iter()
                        .filter(|x| x.title().is_some() && x.link().is_some());

                    for i in rss_channel_iter {
                        // 日付を文字列にパース

                        let parsedate = if i.pub_date().is_some() {
                            DateTime::parse_from_rfc2822(i.pub_date().expect("trance date error"))
                                .expect("parse date Error")
                                .to_rfc3339()
                        } else if i.dublin_core_ext().is_some() {
                            i.dublin_core_ext().unwrap().dates()[0].to_string()
                        } else {
                            let today = Local::now();
                            today.format("%Y-%m-%d").to_string()
                        };

                        let getitem = FeedItem::new(
                            i.title().expect("failed to get title").to_string(),
                            i.link().expect("failed to get link").to_string(),
                            parsedate,
                        );

                        if self.feeds.iter().any(|x| x.title == getitem.title)
                            || self.is_skip_feed(&getitem, true)
                        {
                            continue;
                        } else {
                            self.feeds.push(getitem);
                        }
                    }
                }
                Err(e) => {
                    self.status_message += &e.to_string();
                }
            };
        });

        Ok(())
    }

    fn is_skip_feed(&self, feeditem: &FeedItem, is_link: bool) -> bool {
        let check_items = if is_link {
            &self.setting_item.skip_link
        } else {
            &self.setting_item.skip_words
        };
        // スキップする単語が含まれるか判定
        if is_link {
            for s in check_items.iter() {
                if s.is_empty() {
                    continue;
                } else if feeditem.link.contains(s.trim()) {
                    return true;
                }
            }
        } else {
            for s in check_items.iter() {
                if s.is_empty() {
                    continue;
                } else if feeditem.title.contains(s) {
                    return true;
                }
            }
        }
        false
    }
}

fn save_to_json(
    filename: &PathBuf,
    feeds: Vec<FeedItem>,
    maxsize: usize,
) -> Result<(), Box<dyn Error>> {
    // 上限maxsize個のアイテムをjson化して保存
    let mut feeds = feeds.to_owned();

    feeds.truncate(maxsize);

    let file = fs::File::create(filename)?;

    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, &feeds)?;

    Ok(())
}

fn read_from_json(filename: &PathBuf) -> Result<Vec<FeedItem>, Box<dyn Error>> {
    let file = fs::File::open(filename)?;
    let bufreader = BufReader::new(file);
    let value = serde_json::from_reader(bufreader)?;
    Ok(value)
}

#[test]
fn get_feedtest() {
    let mut rss = RssReader::new().unwrap();
    let myurls = rss.geturls().unwrap();
    rss.getfeed(&myurls).unwrap();
    println!("{:?}", rss);
}

#[test]
fn jsontest() {
    use std::str::FromStr;
    let anitem = FeedItem::new(
        "testtitle".to_string(),
        "testlink".to_string(),
        "2023/7/8".to_string(),
    );
    let filename = PathBuf::from_str("./test.json").unwrap();
    let feeds = vec![anitem];
    save_to_json(&filename, feeds, 5).unwrap();
    let items = read_from_json(&filename).unwrap();
    println!("{:?}", items);
}
