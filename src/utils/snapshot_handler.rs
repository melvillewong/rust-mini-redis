use std::{collections::HashMap, sync::Arc};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::RwLock,
};

use crate::{
    helper::file_helper::{open_file_read, open_file_write},
    types::{MAX_BUFFER, SharedDB},
};

pub async fn snapshot_save(storage: &SharedDB) -> Result<(), serde_json::Error> {
    let json = serde_json::to_string(&*storage.read().await)?;
    let mut file = open_file_write("db.json", true).await;

    file.write_all(format!("{}\n", json).as_bytes())
        .await
        .expect("Failed to write json");

    Ok(())
}

pub async fn snapshot_load() -> SharedDB {
    if let Some(mut file) = open_file_read("db.json").await {
        let mut buf = [0u8; MAX_BUFFER];

        match file.read(&mut buf).await {
            Ok(0) => {
                println!("db.json is empty");
            }
            Ok(byte_read) => {
                let json = String::from_utf8_lossy(&buf[..byte_read]);
                let snapshot_hm =
                    serde_json::from_str(&json).expect("Failed to parse json to hashmap");
                println!("snapshot_load succeed");
                return Arc::new(RwLock::new(snapshot_hm));
            }
            Err(e) => {
                eprintln!("Failed to read json: {}", e);
            }
        }
    }
    Arc::new(RwLock::new(HashMap::new()))
}
