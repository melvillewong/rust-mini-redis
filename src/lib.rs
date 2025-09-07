use std::{
    collections::HashMap,
    io::{Error, ErrorKind::InvalidInput},
    str::SplitWhitespace,
    sync::Arc,
};

use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

type KeyValue = HashMap<String, String>;
type SharedDB = Arc<RwLock<KeyValue>>;
type CleanCmd<'a> = (SplitWhitespace<'a>, usize);

pub async fn proc_cmd(cmd: &str, storage: &mut SharedDB) -> Result<String, Error> {
    let mut argv = clean_cmd(cmd);
    match argv.0.next() {
        Some("SET") => set_cmd(&mut argv, &mut storage.write().await),
        Some("GET") => get_cmd(&mut argv, &mut storage.read().await),
        Some("DEL") => del_cmd(&mut argv, &mut storage.write().await),
        Some(other) => Err(Error::new(
            InvalidInput,
            format!("Invalid command: {}", other),
        )),
        None => Err(Error::new(InvalidInput, "Empty command")),
    }
}

fn set_cmd(argv: &mut CleanCmd, storage: &mut RwLockWriteGuard<KeyValue>) -> Result<String, Error> {
    if argv.1 != 3 {
        return Err(Error::new(
            InvalidInput,
            format!(
                "Invalid arguments for SET command: expected 2 args, found {}",
                argv.1 - 1
            ),
        ));
    }
    let key = argv.0.next().unwrap().to_string();
    let value = argv.0.next().unwrap().to_string();

    storage.insert(key, value);
    Ok(String::from("OK"))
}

fn get_cmd(argv: &mut CleanCmd, storage: &mut RwLockReadGuard<KeyValue>) -> Result<String, Error> {
    if argv.1 != 2 {
        return Err(Error::new(
            InvalidInput,
            format!(
                "Invalid arguments for GET command: expected 1 args, found {}",
                argv.1 - 1
            ),
        ));
    }
    let key = argv.0.next().unwrap().to_string();

    match storage.get(&key) {
        Some(value) => Ok(value.clone()),
        None => Ok("(nil)".to_string()),
    }
}

fn del_cmd(argv: &mut CleanCmd, storage: &mut RwLockWriteGuard<KeyValue>) -> Result<String, Error> {
    if argv.1 != 2 {
        return Err(Error::new(
            InvalidInput,
            format!(
                "Invalid arguments for DEL command: expected 1 args, found {}",
                argv.1 - 1
            ),
        ));
    }
    let key = argv.0.next().unwrap().to_string();

    match storage.remove(&key) {
        Some(_) => Ok("(integer) 1".to_string()),
        None => Ok("(integer) 0".to_string()),
    }
}

fn clean_cmd(cmd: &'_ str) -> CleanCmd<'_> {
    let argv = cmd.split_whitespace();
    let len = argv.clone().count();

    (argv, len)
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup_storage() -> SharedDB {
        Arc::new(RwLock::new(HashMap::new()))
    }

    #[tokio::test]
    async fn test_set_new_key() {
        let mut storage = setup_storage();
        let result = proc_cmd("SET foo bar", &mut storage).await.unwrap();
        assert_eq!(result, "OK");
        let guard = storage.read().await;
        assert_eq!(guard.get("foo"), Some(&"bar".to_string()));
    }

    #[tokio::test]
    async fn test_set_overwrite_existing() {
        let mut storage = setup_storage();
        proc_cmd("SET foo bar", &mut storage).await.unwrap();
        let result = proc_cmd("SET foo baz", &mut storage).await.unwrap();
        assert_eq!(result, "OK");
        let guard = storage.read().await;
        assert_eq!(guard.get("foo"), Some(&"baz".to_string())); // overwritten
    }

    #[tokio::test]
    async fn test_get_existing_key() {
        let mut storage = setup_storage();
        proc_cmd("SET name Alice", &mut storage).await.unwrap();
        let result = proc_cmd("GET name", &mut storage).await.unwrap();
        assert_eq!(result, "Alice");
    }

    #[tokio::test]
    async fn test_get_nonexistent_key() {
        let mut storage = setup_storage();
        let result = proc_cmd("GET missing", &mut storage).await.unwrap();
        assert_eq!(result, "(nil)");
    }

    #[tokio::test]
    async fn test_del_existing_key() {
        let mut storage = setup_storage();
        proc_cmd("SET mykey hello", &mut storage).await.unwrap();
        let result = proc_cmd("DEL mykey", &mut storage).await.unwrap();
        assert_eq!(result, "(integer) 1"); // key existed and was deleted
        let guard = storage.read().await;
        assert!(!guard.contains_key("mykey"));
    }

    #[tokio::test]
    async fn test_del_nonexistent_key() {
        let mut storage = setup_storage();
        let result = proc_cmd("DEL not_here", &mut storage).await.unwrap();
        assert_eq!(result, "(integer) 0"); // no key deleted
    }

    #[tokio::test]
    async fn test_get_after_del() {
        let mut storage = setup_storage();
        proc_cmd("SET foo bar", &mut storage).await.unwrap();
        proc_cmd("DEL foo", &mut storage).await.unwrap();
        let result = proc_cmd("GET foo", &mut storage).await.unwrap();
        assert_eq!(result, "(nil)");
    }
}
