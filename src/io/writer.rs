use std::fs;

use async_channel::Receiver;
use tokio::io::AsyncWriteExt;

pub async fn writer(rx: Receiver<String>) {
    let timestamp = chrono::Local::now().format("%Y;%m;%d_%H;%M;%S").to_string();
    let results_dir = format!("results/{}", timestamp);
    fs::create_dir_all(&results_dir).expect("Cannot create folder");

    let file_path = format!("{}/generated.txt", results_dir);
    let mut file = tokio::fs::File::create(file_path).await.unwrap();

    while let Ok(message) = rx.recv().await {
        file.write(format!("{}\n", message).as_bytes())
            .await
            .expect("Cannot save");
        file.flush().await.expect("Couldnt flush");
    }
}
