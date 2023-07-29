pub mod morse;
use std::collections::HashMap;
use enigo::Key;

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
        main_key: "space".to_string(),
        exit_key: "backspace".to_string(),
        pause_key: "enter".to_string(),
        change_lang_key: "ctrl".to_string(),
        change_case_key: "shift".to_string(),
        time_erase_ms: 1000,
        time_to_long_press_ms: 1000,
    };

    let config_ser = toml::to_string_pretty(&config).unwrap();
    std::fs::write("config.toml", config_ser).unwrap();
}
