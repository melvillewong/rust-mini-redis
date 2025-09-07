use tokio::{
    fs::{File, OpenOptions},
    io::AsyncWriteExt,
};

use crate::helper::types::{CleanCmd, DangerCmd};

pub async fn append_cmd<'a>(argv: CleanCmd<'a>, cmd_type: DangerCmd) {
    let mut file = open_file().await;

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

async fn open_file() -> File {
    OpenOptions::new()
        .create(true)
        .append(true)
        .open("db.aof")
        .await
        .expect("Failed to Create or Open files: open_file")
}
