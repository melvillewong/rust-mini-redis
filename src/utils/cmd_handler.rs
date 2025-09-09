use std::io::{
    Error,
    ErrorKind::{InvalidInput, Other},
};

use crate::{helper::cmd_helper, utils::snapshot_handler::snapshot_save};
use crate::{
    types::{CleanCmd, DangerCmd, SharedDB},
    utils::aof_handler,
};

pub async fn proc_cmd(cmd: &str, storage: &mut SharedDB, replay: bool) -> Result<String, Error> {
    let mut argv = cmd_helper::split_cmd(cmd);
    match argv.0.next() {
        Some("SET") => set_cmd(&mut argv, storage, replay).await,
        Some("GET") => get_cmd(&mut argv, storage).await,
        Some("DEL") => del_cmd(&mut argv, storage, replay).await,
        Some("SAVE") => save_cmd(&mut argv, storage).await,
        Some(other) => Err(Error::new(
            InvalidInput,
            format!("Invalid command: {}", other),
        )),
        None => Err(Error::new(InvalidInput, "Empty command")),
    }
}

async fn save_cmd<'a>(argv: &mut CleanCmd<'a>, storage: &SharedDB) -> Result<String, Error> {
    cmd_helper::validate_save(argv)?;

    match snapshot_save(storage).await {
        Ok(_) => Ok(String::from("OK")),
        Err(e) => Err(Error::new(Other, format!("ERR {}", e))),
    }
}

async fn set_cmd<'a>(
    argv: &mut CleanCmd<'a>,
    storage: &SharedDB,
    replay: bool,
) -> Result<String, Error> {
    cmd_helper::validate_set(argv)?;

    if !replay {
        aof_handler::append_cmd(argv.clone(), DangerCmd::Set).await;
    }

    let mut storage = storage.write().await;
    let key = argv.0.next().unwrap().to_string();
    let value = argv.0.next().unwrap().to_string();

    storage.insert(key, value);
    Ok(String::from("OK"))
}

async fn get_cmd<'a>(argv: &mut CleanCmd<'a>, storage: &SharedDB) -> Result<String, Error> {
    cmd_helper::validate_get(argv)?;

    let storage = storage.read().await;
    let key = argv.0.next().unwrap().to_string();

    match storage.get(&key) {
        Some(value) => Ok(value.clone()),
        None => Ok("(nil)".to_string()),
    }
}

async fn del_cmd<'a>(
    argv: &mut CleanCmd<'a>,
    storage: &SharedDB,
    replay: bool,
) -> Result<String, Error> {
    cmd_helper::validate_del(argv)?;
    let argv_clone = argv.clone();

    let mut storage = storage.write().await;
    let key = argv.0.next().unwrap().to_string();

    match storage.remove(&key) {
        Some(_) => {
            if !replay {
                aof_handler::append_cmd(argv_clone, DangerCmd::Del).await;
            }
            Ok("(integer) 1".to_string())
        }
        None => Ok("(integer) 0".to_string()),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{collections::HashMap, sync::Arc};
    use tokio::sync::RwLock;

    fn setup_storage() -> SharedDB {
        Arc::new(RwLock::new(HashMap::new()))
    }

    #[tokio::test]
    async fn test_set_new_key() {
        let mut storage = setup_storage();
        let result = proc_cmd("SET foo bar", &mut storage, false).await.unwrap();
        assert_eq!(result, "OK");
        let guard = storage.read().await;
        assert_eq!(guard.get("foo"), Some(&"bar".to_string()));
    }

    #[tokio::test]
    async fn test_set_overwrite_existing() {
        let mut storage = setup_storage();
        proc_cmd("SET foo bar", &mut storage, false).await.unwrap();
        let result = proc_cmd("SET foo baz", &mut storage, false).await.unwrap();
        assert_eq!(result, "OK");
        let guard = storage.read().await;
        assert_eq!(guard.get("foo"), Some(&"baz".to_string())); // overwritten
    }

    #[tokio::test]
    async fn test_get_existing_key() {
        let mut storage = setup_storage();
        proc_cmd("SET name Alice", &mut storage, false)
            .await
            .unwrap();
        let result = proc_cmd("GET name", &mut storage, false).await.unwrap();
        assert_eq!(result, "Alice");
    }

    #[tokio::test]
    async fn test_get_nonexistent_key() {
        let mut storage = setup_storage();
        let result = proc_cmd("GET missing", &mut storage, false).await.unwrap();
        assert_eq!(result, "(nil)");
    }

    #[tokio::test]
    async fn test_del_existing_key() {
        let mut storage = setup_storage();
        proc_cmd("SET mykey hello", &mut storage, false)
            .await
            .unwrap();
        let result = proc_cmd("DEL mykey", &mut storage, false).await.unwrap();
        assert_eq!(result, "(integer) 1"); // key existed and was deleted
        let guard = storage.read().await;
        assert!(!guard.contains_key("mykey"));
    }

    #[tokio::test]
    async fn test_del_nonexistent_key() {
        let mut storage = setup_storage();
        let result = proc_cmd("DEL not_here", &mut storage, false).await.unwrap();
        assert_eq!(result, "(integer) 0"); // no key deleted
    }

    #[tokio::test]
    async fn test_get_after_del() {
        let mut storage = setup_storage();
        proc_cmd("SET foo bar", &mut storage, false).await.unwrap();
        proc_cmd("DEL foo", &mut storage, false).await.unwrap();
        let result = proc_cmd("GET foo", &mut storage, false).await.unwrap();
        assert_eq!(result, "(nil)");
    }
}
