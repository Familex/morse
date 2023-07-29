use enigo::keycodes::Key;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug)]
pub enum MorseKey {
    Dot,
    Slash,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigKeySerde {
    pub sequence: String,
    pub lower: Key,
    pub upper: Option<Key>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigSerde {
    pub langs: HashMap<String, Vec<ConfigKeySerde>>,
    pub functional: Vec<ConfigKeySerde>,
    pub main_key: String,
    pub exit_key: String,
    pub pause_key: String,
    pub change_lang_key: String,
    pub change_case_key: String,
    pub time_erase_ms: u32,
    pub time_to_long_press_ms: u32,
}

pub struct ConfigKey {
    pub sequence: Vec<MorseKey>,
    pub lower: Key,
    pub upper: Option<Key>,
}

pub struct Config {
    pub langs: HashMap<String, HashMap<Key, ConfigKey>>,
    pub functional: HashMap<Key, ConfigKey>,
    pub main_key: String,
    pub exit_key: String,
    pub pause_key: String,
    pub change_lang_key: String,
    pub change_case_key: String,
    pub time_erase_ms: u32,
    pub time_to_long_press_ms: u32,
}

impl TryFrom<ConfigKeySerde> for ConfigKey {
    type Error = ();
    fn try_from(value: ConfigKeySerde) -> Result<Self, Self::Error> {
        let mut sequence = Vec::new();
        for c in value.sequence.chars() {
            match c {
                '.' => sequence.push(MorseKey::Dot),
                '-' => sequence.push(MorseKey::Slash),
                _ => return Err(()),
            }
        }
        Ok(ConfigKey {
            sequence,
            lower: value.lower,
            upper: value.upper,
        })
    }
}

impl TryInto<Config> for ConfigSerde {
    type Error = ();

    fn try_into(self) -> Result<Config, Self::Error> {
        let mut langs = HashMap::new();
        for (lang, keys) in self.langs {
            let mut keys_map = HashMap::new();
            for key_serde in keys {
                keys_map.insert(key_serde.lower.clone(), key_serde.try_into()?);
            }
            langs.insert(lang, keys_map);
        }
        let mut functional = HashMap::new();
        for key_serde in self.functional {
            functional.insert(key_serde.lower.clone(), key_serde.try_into()?);
        }
        Ok(Config {
            langs,
            functional,
            main_key: self.main_key,
            exit_key: self.exit_key,
            pause_key: self.pause_key,
            change_lang_key: self.change_lang_key,
            change_case_key: self.change_case_key,
            time_erase_ms: self.time_erase_ms,
            time_to_long_press_ms: self.time_to_long_press_ms,
        })
    }
}
