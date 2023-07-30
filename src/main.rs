use enigo::KeyboardControllable;

pub mod morse;

fn main() {
    let config = std::fs::read("other/config.toml").unwrap();
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
        InputEvent::SequenceParsed(seq, event_key) => {
            use morse::ConfigKey;

            println!("Sequence parsed: {:?} -> {:?}", seq, event_key);
            // remove morse keys
            for _ in 0..seq.len() {
                enigo.key_click(enigo::Key::Backspace);
            }
            match event_key.key {
                ConfigKey::Code(code) => {
                    enigo.key_click(code);
                }
                ConfigKey::Layout(layout) => {
                    if event_key.is_upper {
                        enigo.key_down(enigo::Key::Shift);
                        enigo.key_click(layout.lower);
                        enigo.key_up(enigo::Key::Shift);
                    } else {
                        enigo.key_click(layout.lower);
                    }
                }
                ConfigKey::Sequence(seq) => {
                    for layout in seq {
                        if event_key.is_upper {
                            enigo.key_down(enigo::Key::Shift);
                            enigo.key_click(layout.lower);
                            enigo.key_up(enigo::Key::Shift);
                        } else {
                            enigo.key_click(layout.lower);
                        }
                    }
                }
            }
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
