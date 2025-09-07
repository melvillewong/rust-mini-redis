use std::{collections::HashMap, str::SplitWhitespace, sync::Arc};

use tokio::sync::RwLock;

pub enum DangerCmd {
    Set,
    Del,
}

pub type CleanCmd<'a> = (SplitWhitespace<'a>, usize);
pub type KeyValue = HashMap<String, String>;
pub type SharedDB = Arc<RwLock<KeyValue>>;
