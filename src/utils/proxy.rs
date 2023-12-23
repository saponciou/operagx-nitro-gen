use std::{fmt::Formatter, path::PathBuf, sync::Arc};

use async_channel::Sender;
use atomic_counter::AtomicCounter;
use reqwest::Proxy;
use rfd::{AsyncFileDialog, FileHandle};
use termcolor::Color;
use tokio::io::{AsyncBufReadExt, BufReader};

use crate::threading::stats::Stats;

use super::{
    cmd::Text,
    errors::{ErrorT, ErrorType},
    Res,
};

#[derive(PartialEq, Clone)]
pub enum ProxyPath {
    Path(PathBuf),
    ProxyLess,
}

#[derive(PartialEq, Clone)]
pub enum ProxyType {
    Http,
    Socks5,
    ProxyLess,
}

impl std::fmt::Display for ProxyType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProxyType::Http => write!(f, "http"),
            ProxyType::Socks5 => write!(f, "socks5"),
            ProxyType::ProxyLess => write!(f, ""),
        }
    }
}

pub fn proxy_builder(proxy: Option<String>, proxy_less: &mut bool, pt: ProxyType) -> Res<Proxy> {
    if pt != ProxyType::ProxyLess {
        match proxy {
            Some(px) => {
                *proxy_less = false;
                let proxy_vec: Vec<String> = px.split(':').map(|part| part.to_string()).collect();

                let url = format!(
                    "{}://{}:{}",
                    pt,
                    proxy_vec
                        .get(0)
                        .ok_or(ErrorT::new(ErrorType::InvalidProxy))?,
                    proxy_vec
                        .get(1)
                        .ok_or(ErrorT::new(ErrorType::InvalidProxy))?,
                );

                if proxy_vec.len() == 2 || proxy_vec.len() == 4 {
                    if proxy_vec.len() == 2 {
                        Ok(Proxy::all(url)?)
                    } else if proxy_vec.len() == 4 {
                        let user = proxy_vec
                            .get(2)
                            .ok_or(ErrorT::new(ErrorType::InvalidProxy))?
                            .to_string();
                        let pass = proxy_vec
                            .get(3)
                            .ok_or(ErrorT::new(ErrorType::InvalidProxy))?
                            .to_string();
                        Ok(Proxy::all(url)?.basic_auth(&user, &pass))
                    } else {
                        Err(ErrorT::new(ErrorType::InvalidProxy))
                    }
                } else {
                    Err(ErrorT::new(ErrorType::InvalidProxy))
                }
            }
            None => {
                *proxy_less = true;
                Ok(Proxy::all("http://127.0.0.1:8080")?)
            }
        }
    } else {
        *proxy_less = true;
        Ok(Proxy::all("http://127.0.0.1:8080")?)
    }
}

pub enum FileRes {
    Success(FileHandle),
    Fail,
}

pub async fn pick_file(name: &str) -> FileRes {
    match AsyncFileDialog::new()
        .set_title(format!("Choose {}", name).as_str())
        .add_filter(name, &["txt"])
        .set_directory("/")
        .pick_file()
        .await
    {
        Some(file) => FileRes::Success(file),
        None => FileRes::Fail,
    }
}

pub async fn reader(stats: Arc<Stats>, proxy_file: ProxyPath, tx: Sender<Option<String>>) {
    match proxy_file {
        ProxyPath::Path(proxy_file) => {
            let proxies = tokio::fs::File::open(proxy_file)
                .await
                .expect("Failed to open input file");

            let preader = BufReader::new(proxies);
            let mut proxies_file = preader.lines();
            let mut proxies: Vec<String> = Vec::new();

            while let Ok(Some(proxy)) = proxies_file.next_line().await {
                proxies.push(proxy);
            }

            if proxies.is_empty() {
                Text::new(
                    super::cmd::TextType::LN("   No proxies found in proxy file".to_string()),
                    Color::Red,
                )
                .w();
                tx.close();
                return;
            }

            while stats.stop.get() <= 1 {
                let i = fastrand::usize(..proxies.len());
                if let Err(e) = tx.send(proxies.get(i).cloned()).await {
                    println!("Failed to send line: {:#?}", e);
                }
            }
            tx.close();
        }
        ProxyPath::ProxyLess => {
            while stats.stop.get() <= 1 {
                if let Err(e) = tx.send(None).await {
                    println!("Failed to send line: {:#?}", e);
                }
            }
            tx.close();
        }
    };
}
