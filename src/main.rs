use enigo::KeyboardControllable;

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
    use morse::{InputEvent, MorseKey};
    use std::thread::spawn;
    use windows::Win32::System::Diagnostics::Debug::Beep;

    const DOT_KEY: &str = ".";
    const DOT_KEY_UPPER: &str = ">";
    const DASH_KEY: &str = "-";
    const DASH_KEY_UPPER: &str = "_";
    const DOT_BEEP_FREQ: u32 = 500;
    const DASH_BEEP_FREQ: u32 = 800;
    const DOT_BEEP_DURATION: u32 = 60;
    const DASH_BEEP_DURATION: u32 = 400;

    let mut enigo = enigo::Enigo::new();

    match event {
        InputEvent::MorseKey(key) => {
            enigo.key_click(enigo::Key::Backspace); // remove main key
            match key {
                MorseKey::Dot => {
                    unsafe {
                        spawn(|| Beep(DOT_BEEP_FREQ, DOT_BEEP_DURATION));
                    }
                    if state.is_upper_case {
                        enigo.key_sequence(DOT_KEY_UPPER);
                    } else {
                        enigo.key_sequence(DOT_KEY);
                    }
                }
                MorseKey::Dash => {
                    unsafe {
                        spawn(|| Beep(DASH_BEEP_FREQ, DASH_BEEP_DURATION));
                    }
                    if state.is_upper_case {
                        enigo.key_sequence(DASH_KEY_UPPER);
                    } else {
                        enigo.key_sequence(DASH_KEY);
                    }
                }
            }
        }
        InputEvent::SequenceParsed(seq, key) => {
            println!("Sequence parsed: {:?} -> {:?}", seq, key);
            // remove morse keys
            for _ in 0..seq.len() {
                enigo.key_click(enigo::Key::Backspace);
            }
            enigo.key_click(key); // click parsed key
        }
        InputEvent::SeqRejected(seq, reason) => {
            println!("Sequence rejected: {:?} -> {:?}", seq, reason);
            // remove morse keys
            for _ in 0..seq.len() {
                enigo.key_click(enigo::Key::Backspace);
            }
        }
        InputEvent::LangChange(lang) => {
            println!("Lang changed: {:?}", lang);
        }
        InputEvent::CaseChange(is_upper) => {
            println!("Case changed: {:?}", is_upper);
        }
        InputEvent::PauseToggle(is_pause) => {
            println!("Pause changed: {:?}", is_pause);
        }
        InputEvent::Exit => {
            println!("Exit");
        }
    }
}
