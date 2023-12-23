use std::{
    sync::Arc,
    thread::{self, sleep},
    time::{Duration, Instant},
};

use atomic_counter::AtomicCounter;
use tokio::task::JoinHandle;

use crate::{
    io::writer::writer,
    utils::{
        proxy::{reader, ProxyPath, ProxyType},
        Res,
    },
};

use super::{
    stats::{set_title, Stats},
    worker::worker,
};

pub async fn run(
    threads: usize,
    max_gens: usize,
    proxy_path: ProxyPath,
    proxy_type: ProxyType,
) -> Res<()> {
    let stats: Arc<Stats> = Arc::new(Stats::default());

    let (writer_tx, writer_rx) = async_channel::bounded::<String>(100_000);
    let writer_handle = tokio::spawn(writer(writer_rx));

    let c = Arc::clone(&stats);
    let (reader_tx, reader_rx) = async_channel::bounded::<Option<String>>(100_000);
    let reader_handle = tokio::spawn(reader(c, proxy_path, reader_tx));

    thread::spawn({
        let stats = Arc::clone(&stats);
        move || {
            let start_time = Instant::now();
            let duration = Instant::now();
            while stats.stop.get() <= 1 {
                sleep(Duration::from_millis(120));

                let check_time = Instant::now();
                let total_average = stats.total_gens.get() as f64
                    / check_time.duration_since(start_time).as_secs_f64();
                stats.gens.reset();

                let seconds = duration.elapsed().as_secs();
                let minutes = seconds / 60;
                let hours = minutes / 60;
                let seconds = seconds % 60;
                let minutes = minutes % 60;
                let dur = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
                set_title(
                    dur,
                    stats.total_gens.get(),
                    total_average * 60f64,
                    stats.retries.get(),
                    stats.errors.get(),
                );
            }
        }
    });

    let handles: Vec<JoinHandle<()>> = (0..threads)
        .map(|_| {
            let stats: Arc<Stats> = Arc::clone(&stats);
            let writer_tx = writer_tx.clone();
            let reader_rx = reader_rx.clone();
            let proxy_type = proxy_type.clone();
            tokio::spawn(async move {
                let _ = worker(stats, writer_tx, reader_rx, proxy_type, max_gens).await;
            })
        })
        .collect();

    for handle in handles {
        handle.await.expect("Failed to join handle");
    }

    tokio::time::sleep(Duration::from_secs(1)).await;

    stats.stop.inc();

    writer_tx.close();
    reader_rx.close();

    writer_handle.await.expect("Failed to join writer");
    reader_handle.await.expect("Failed to join reader");

    println!("   Done!");
    Ok(())
}
