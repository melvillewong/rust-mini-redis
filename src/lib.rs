use std::{
    collections::HashMap,
    io::{Error, ErrorKind::InvalidInput},
    str::SplitWhitespace,
};

type KeyValue = HashMap<String, String>;
type CleanCmd<'a> = (SplitWhitespace<'a>, usize);

pub fn proc_cmd(cmd: &str, storage: &mut KeyValue) -> Result<String, Error> {
    let mut argv = clean_cmd(cmd);
    match argv.0.next() {
        Some("SET") => set_cmd(&mut argv, storage),
        Some("GET") => get_cmd(&mut argv, storage),
        Some("DEL") => del_cmd(&mut argv, storage),
        Some(other) => Err(Error::new(
            InvalidInput,
            format!("Invalid command: {}", other),
        )),
        None => Err(Error::new(InvalidInput, "Empty command")),
    }
}

fn set_cmd(argv: &mut CleanCmd, storage: &mut KeyValue) -> Result<String, Error> {
    if argv.1 != 3 {
        return Err(Error::new(
            InvalidInput,
            "Invalid arguments for SET command",
        ));
    }
    let key = argv.0.next().unwrap().to_string();
    let value = argv.0.next().unwrap().to_string();

    storage.insert(key, value);
    Ok(String::from("OK"))
}

fn get_cmd(argv: &mut CleanCmd, storage: &mut KeyValue) -> Result<String, Error> {
    if argv.1 != 2 {
        return Err(Error::new(
            InvalidInput,
            "Invalid arguments for GET command",
        ));
    }
    let key = argv.0.next().unwrap().to_string();

    match storage.get(&key) {
        Some(value) => Ok(value.clone()),
        None => Ok("(nil)".to_string()),
    }
}

fn del_cmd(argv: &mut CleanCmd, storage: &mut KeyValue) -> Result<String, Error> {
    if argv.1 != 2 {
        return Err(Error::new(
            InvalidInput,
            "Invalid arguments for DEL command",
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
    use std::collections::HashMap;

    fn setup_storage() -> KeyValue {
        HashMap::new()
    }

    #[test]
    fn test_set_new_key() {
        let mut storage = setup_storage();
        let result = proc_cmd("SET foo bar", &mut storage).unwrap();
        assert_eq!(result, "OK");
        assert_eq!(storage.get("foo"), Some(&"bar".to_string()));
    }

    #[test]
    fn test_set_overwrite_existing() {
        let mut storage = setup_storage();
        proc_cmd("SET foo bar", &mut storage).unwrap();
        let result = proc_cmd("SET foo baz", &mut storage).unwrap();
        assert_eq!(result, "OK");
        assert_eq!(storage.get("foo"), Some(&"baz".to_string())); // overwritten
    }

    #[test]
    fn test_get_existing_key() {
        let mut storage = setup_storage();
        proc_cmd("SET name Alice", &mut storage).unwrap();
        let result = proc_cmd("GET name", &mut storage).unwrap();
        assert_eq!(result, "Alice");
    }

    #[test]
    fn test_get_nonexistent_key() {
        let mut storage = setup_storage();
        let result = proc_cmd("GET missing", &mut storage).unwrap();
        assert_eq!(result, "(nil)");
    }

    #[test]
    fn test_del_existing_key() {
        let mut storage = setup_storage();
        proc_cmd("SET mykey hello", &mut storage).unwrap();
        let result = proc_cmd("DEL mykey", &mut storage).unwrap();
        assert_eq!(result, "(integer) 1"); // key existed and was deleted
        assert!(!storage.contains_key("mykey"));
    }

    #[test]
    fn test_del_nonexistent_key() {
        let mut storage = setup_storage();
        let result = proc_cmd("DEL not_here", &mut storage).unwrap();
        assert_eq!(result, "(integer) 0"); // no key deleted
    }

    #[test]
    fn test_get_after_del() {
        let mut storage = setup_storage();
        proc_cmd("SET foo bar", &mut storage).unwrap();
        proc_cmd("DEL foo", &mut storage).unwrap();
        let result = proc_cmd("GET foo", &mut storage).unwrap();
        assert_eq!(result, "(nil)");
    }
}

