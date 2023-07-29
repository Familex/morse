pub mod morse;

fn main() {
    {
        use enigo::Key;
        use std::collections::HashMap;
        use std::time::Duration;

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
            exit: Key::Escape,
            pause: Key::Backspace,
            change_lang: Key::Control,
            change_case: Key::Shift,
            time_to_long_press: Duration::from_millis(100),
            listen_delay: Duration::from_millis(5),
            accept_sequence_delay: Duration::from_millis(750),
        };

        let config = toml::to_string_pretty(&config).unwrap();
        std::fs::write("config.toml", config).unwrap();
    }

    let config = std::fs::read("config.toml").unwrap();
    let config = std::str::from_utf8(&config).unwrap();
    let config = toml::from_str::<morse::ConfigSerde>(config).unwrap();
    let config: morse::Config = config.try_into().unwrap();
    morse::listen_loop(&config, event_handler);
}

fn event_handler(event: morse::InputEvent, state: &mut morse::InputState) {
    println!("event = {:?}; state = {:?}", event, state);
}
