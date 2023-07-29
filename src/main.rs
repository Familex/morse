pub mod morse;
use enigo::Key;
use std::{collections::HashMap, time::Duration};

fn main() {
    // test config
    let config = morse::ConfigSerde {
        langs: {
            let mut langs = HashMap::new();
            langs.insert("en".to_string(), {
                let mut en = Vec::new();
                en.push(morse::ConfigKeySerde {
                    sequence: ".-".to_string(),
                    lower: Key::Layout('a'),
                    upper: Some(Key::Layout('A')),
                });
                en.push(morse::ConfigKeySerde {
                    sequence: "-...".to_string(),
                    lower: Key::Layout('b'),
                    upper: Some(Key::Layout('B')),
                });
                en
            });
            langs.insert("ru".to_string(), {
                let mut ru = Vec::new();
                ru.push(morse::ConfigKeySerde {
                    sequence: ".-".to_string(),
                    lower: Key::Layout('а'),
                    upper: Some(Key::Layout('А')),
                });
                ru.push(morse::ConfigKeySerde {
                    sequence: "-...".to_string(),
                    lower: Key::Layout('б'),
                    upper: Some(Key::Layout('Б')),
                });
                ru
            });
            langs
        },
        functional: {
            let mut functional = Vec::new();
            functional.push(morse::ConfigKeySerde {
                sequence: "---.-".to_string(),
                lower: Key::Space,
                upper: None,
            });
            functional.push(morse::ConfigKeySerde {
                sequence: ".-.--".to_string(),
                lower: Key::Backspace,
                upper: None,
            });
            functional.push(morse::ConfigKeySerde {
                sequence: ".---.-".to_string(),
                lower: Key::Accept,
                upper: None,
            });
            functional
        },
        main: Key::Space,
        exit: Key::Backspace,
        pause: Key::Accept,
        change_lang: Key::Control,
        change_case: Key::Shift,
        time_erase: Duration::from_millis(1000),
        time_to_long_press: Duration::from_millis(1000),
    };

    let config_ser = toml::to_string_pretty(&config).unwrap();
    std::fs::write("config.toml", config_ser).unwrap();

    // test config
    let config = std::fs::read("config.toml").unwrap();
    let config = std::str::from_utf8(&config).unwrap();
    let config: morse::ConfigSerde = toml::from_str(config).unwrap();
    let config: morse::Config = config.try_into().unwrap();
    dbg!(config);
}
