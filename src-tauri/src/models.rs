use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct MusicItem {
    pub hash: String,
    pub size: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MusicTable {
    pub overworld_old: HashMap<String, MusicItem>,
    pub overworld_new: HashMap<String, MusicItem>,
    pub underwater: HashMap<String, MusicItem>,
    pub deep_dark: HashMap<String, MusicItem>,
    pub creative: HashMap<String, MusicItem>,
    pub nether_old: HashMap<String, MusicItem>,
    pub nether_new: HashMap<String, MusicItem>,
    pub end: HashMap<String, MusicItem>,
    pub menu: HashMap<String, MusicItem>,
}
