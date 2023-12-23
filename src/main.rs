use std::process::exit;

use threading::runner::run;
use utils::{
    cmd::{pause, Input, Menu},
    proxy::{pick_file, FileRes, ProxyPath, ProxyType},
    Res,
};

mod api;
mod io;
mod threading;
mod utils;

#[tokio::main]
async fn main() {
    loop {
        let _ = lobby().await;
    }
}

async fn lobby() -> Res<()> {
    loop {
        let mode_selector = Menu::new(
            "Lobby",
            vec![
                "Start".to_string(),
                "Exit".to_string(),
            ],
        )
        .show();

        match mode_selector {
            1 => {
                break;
            }
            2 => {
                exit(0);
            }
            _ => {
                continue;
            }
        }
    }
    let threads: usize;
    loop {
        Menu::title();
        let num = Menu::input(Input::String("Threads".to_string()));
        match num.parse::<usize>() {
            Ok(num) => {
                threads = num;
                break;
            }
            Err(_) => {
                continue;
            }
        }
    }

    let max_gens: usize;
    loop {
        Menu::title();
        let num = Menu::input(Input::String("Max codes to gen".to_string()));
        match num.parse::<usize>() {
            Ok(num) => {
                max_gens = num;
                break;
            }
            Err(_) => {
                continue;
            }
        }
    }

    let proxy_type;
    let proxy_path;
    loop {
        let mode_selector = Menu::new(
            "Select Proxies",
            vec![
                "Http/s".to_string(),
                "Socks5".to_string(),
                "Proxyless".to_string(),
                "Back".to_string(),
            ],
        )
        .show();

        match mode_selector {
            1 => {
                proxy_type = ProxyType::Http;
                break;
            }
            2 => {
                proxy_type = ProxyType::Socks5;
                break;
            }
            3 => {
                proxy_type = ProxyType::ProxyLess;
                break;
            }
            4 => return Ok(()),
            _ => {
                continue;
            }
        }
    }

    if proxy_type != ProxyType::ProxyLess {
        let path = match pick_file("Proxy").await {
            FileRes::Success(file) => file,
            FileRes::Fail => return Ok(()),
        }
        .path()
        .to_path_buf();
        proxy_path = ProxyPath::Path(path);
    } else {
        proxy_path = ProxyPath::ProxyLess;
    }

    Menu::title();

    let _ = run(threads, max_gens, proxy_path, proxy_type).await;
    pause();
    Ok(())
}
