use std::sync::Arc;

use async_channel::{Receiver, Sender};
use atomic_counter::AtomicCounter;
use reqwest::{header::HeaderMap, Proxy};

use crate::{
    api::api::get_code,
    utils::{
        proxy::{proxy_builder, ProxyType},
        Res,
    },
};

use super::stats::Stats;

pub async fn worker(
    stats: Arc<Stats>,
    writer: Sender<String>,
    reader: Receiver<Option<String>>,
    pt: ProxyType,
    max_gens: usize,
) -> Res<()> {
    let mut headers = HeaderMap::new();
    headers.append("accept", "*/*".parse().unwrap());
    headers.append("accept-language", "es-ES,es;q=0.9".parse().unwrap());
    headers.append("content-type", "application/json".parse().unwrap());
    headers.append(
        "sec-ch-ua",
        "\"Opera GX\";v=\"105\", \"Chromium\";v=\"119\", \"Not?A_Brand\";v=\"24\""
            .parse()
            .unwrap(),
    );
    headers.append("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.append("sec-ch-ua-platform", "\"Windows\"".parse().unwrap());
    headers.append("sec-fetch-dest", "empty".parse().unwrap());
    headers.append("sec-fetch-mode", "cors".parse().unwrap());
    headers.append("sec-fetch-site", "cross-site".parse().unwrap());
    headers.append("Referer", "https://www.opera.com/".parse().unwrap());
    headers.append(
        "Referrer-Policy",
        "strict-origin-when-cross-origin".parse().unwrap(),
    );
    headers.append("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36 OPR/105.0.0.0".parse().unwrap());
    headers.append("Origin", "https://www.opera.com".parse().unwrap());

    while let Ok(proxy) = reader.recv().await {
        let headers = headers.clone();
        let mut proxy_less = false;

        let proxy: Proxy = match proxy_builder(proxy.clone(), &mut proxy_less, pt.clone()) {
            Ok(proxy) => proxy,
            Err(_e) => {
                stats.errors.inc();
                continue;
            }
        };

        let client = match proxy_less {
            true => reqwest::Client::builder()
                .default_headers(headers)
                .build()?,
            false => reqwest::Client::builder()
                .default_headers(headers)
                .proxy(proxy)
                .build()?,
        };

        if stats.total_gens.get() >= max_gens {
            stats.stop.inc();
        } else {
            let code = match get_code(&client).await {
                Ok(code) => code,
                Err(_e) => {
                    println!("{:?}", _e);
                    stats.errors.inc();
                    continue;
                }
            };
            stats.gens.inc();
            stats.total_gens.inc();
            writer.send(code).await.expect("Couldnt send code");
        }
    }
    Ok(())
}
