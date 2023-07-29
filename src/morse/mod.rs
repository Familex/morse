use enigo::keycodes::Key;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MorseKey {
    Dot,
    Slash,
}

pub type MorseSequence = Vec<MorseKey>;

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
    pub main: Key,
    pub exit: Key,
    pub pause: Key,
    pub change_lang: Key,
    pub change_case: Key,
    pub time_erase: Duration,
    pub time_to_long_press: Duration,
}

#[derive(Debug)]
pub struct ConfigKey {
    pub sequence: MorseSequence,
    pub lower: Key,
    pub upper: Option<Key>,
}

#[derive(Debug)]
pub struct Config {
    pub langs: HashMap<String, HashMap<MorseSequence, ConfigKey>>,
    pub functional: HashMap<MorseSequence, ConfigKey>,
    pub main: Key,
    pub exit: Key,
    pub pause: Key,
    pub change_lang: Key,
    pub change_case: Key,
    pub time_erase: Duration,
    pub time_to_long_press: Duration,
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
                let key: ConfigKey = key_serde.try_into()?;
                keys_map.insert(key.sequence.clone(), key);
            }
            langs.insert(lang, keys_map);
        }
        let mut functional = HashMap::new();
        for key_serde in self.functional {
            let key: ConfigKey = key_serde.try_into()?;
            functional.insert(key.sequence.clone(), key);
        }
        Ok(Config {
            langs,
            functional,
            main: self.main,
            exit: self.exit,
            pause: self.pause,
            change_lang: self.change_lang,
            change_case: self.change_case,
            time_erase: self.time_erase,
            time_to_long_press: self.time_to_long_press,
        })
    }
}
