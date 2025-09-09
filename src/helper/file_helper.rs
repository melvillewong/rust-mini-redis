use tokio::fs::{File, OpenOptions, metadata};

pub async fn open_file_write(path: &str, overwrite: bool) -> File {
    let mut opts = OpenOptions::new();
    opts.create(true).write(true).read(true);

    if overwrite {
        opts.truncate(true);
    } else {
        opts.append(true);
    }

    opts.open(path)
        .await
        .expect("Failed to Create or Open files: open_file_write")
}

pub async fn open_file_read(path: &str) -> Option<File> {
    if metadata(path).await.is_ok() {
        match OpenOptions::new().read(true).open(path).await {
            Ok(file) => Some(file),
            Err(_) => {
                println!("Failed to Create or Open files: open_file_read");
                None
            }
        }
    } else {
        println!("Non-existing path: {}", path);
        None
    }
}
