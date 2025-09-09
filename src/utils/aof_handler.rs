use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::{
    helper::file_helper::{open_file_read, open_file_write},
    types::{CleanCmd, DangerCmd, SharedDB},
    utils::cmd_handler::proc_cmd,
};

pub async fn append_cmd<'a>(argv: CleanCmd<'a>, cmd_type: DangerCmd) {
    let mut file = open_file_write("db.aof", false).await;

    let prefix_argv = match cmd_type {
        DangerCmd::Set => std::iter::once("SET").chain(argv.0),
        DangerCmd::Del => std::iter::once("DEL").chain(argv.0),
    };
    let mut cmd: String = prefix_argv.collect::<Vec<_>>().join(" ");
    cmd.push('\n');

    file.write_all(cmd.as_bytes())
        .await
        .expect("Failed to write");
}

pub async fn startup_load(storage: &mut SharedDB) {
    if let Some(mut file) = open_file_read("db.aof").await {
        let mut stored = String::new();

        if let Err(e) = file.read_to_string(&mut stored).await {
            eprintln!("Failed to read db.aof: {}", e);
        }

        for line in stored.lines() {
            if let Err(e) = proc_cmd(line, storage, true).await {
                eprintln!("Failed to proc_cmd during startup_load: {}", e);
            }
        }
        println!("startup_aof_load succeed");
    }
}
