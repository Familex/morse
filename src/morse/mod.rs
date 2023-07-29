use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use std::{collections::HashMap, time::Duration};
use windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY;

pub type KeyCode = enigo::Key;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MorseKey {
    Dot,
    Slash,
}

pub type MorseSequence = Vec<MorseKey>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigKeySerde {
    pub sequence: String,
    pub lower: KeyCode,
    pub upper: Option<KeyCode>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigSerde {
    pub langs: HashMap<String, Vec<ConfigKeySerde>>,
    pub functional: Vec<ConfigKeySerde>,
    pub main: KeyCode,
    pub exit: KeyCode,
    pub pause: KeyCode,
    pub change_lang: KeyCode,
    pub change_case: KeyCode,
    pub time_to_long_press: Duration,
    pub listen_delay: Duration,
    /// time to wait for transform sequence to event
    pub accept_sequence_delay: Duration,
}

#[derive(Debug, Clone)]
pub struct ConfigKey {
    pub sequence: MorseSequence,
    pub lower: KeyCode,
    pub upper: Option<KeyCode>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub langs: HashMap<String, HashMap<MorseSequence, ConfigKey>>,
    pub functional: HashMap<MorseSequence, ConfigKey>,
    pub main: KeyCode,
    pub exit: KeyCode,
    pub pause: KeyCode,
    pub change_lang: KeyCode,
    pub change_case: KeyCode,
    pub time_to_long_press: Duration,
    pub listen_delay: Duration,
    pub accept_sequence_delay: Duration,
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
            time_to_long_press: self.time_to_long_press,
            listen_delay: self.listen_delay,
            accept_sequence_delay: self.accept_sequence_delay,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyState {
    Down(std::time::SystemTime),
    NotPressed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputWorkState {
    Pause,
    Exit,
    Work,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputState {
    main_key_state: KeyState,
    lang_key_state: KeyState,
    change_case_key_state: KeyState,
    change_lang_key_state: KeyState,
    pause_key_state: KeyState,
    exit_key_state: KeyState,
    last_main_key_press: Option<SystemTime>,
    sequence: MorseSequence,
    is_upper_case: bool,
    lang: Option<String>,
    work_state: InputWorkState,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SequenceRejectReason {
    NoLangsLoaded,
    InvalidSequence,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InputEvent {
    SequenceParsed(MorseSequence, KeyCode),
    SeqRejected(MorseSequence, SequenceRejectReason),
    LangChange,
    CaseChange,
    Exit,
    PauseToggle,
}

impl InputState {
    fn new(config: &Config) -> Self {
        InputState {
            main_key_state: KeyState::NotPressed,
            lang_key_state: KeyState::NotPressed,
            change_case_key_state: KeyState::NotPressed,
            change_lang_key_state: KeyState::NotPressed,
            pause_key_state: KeyState::NotPressed,
            exit_key_state: KeyState::NotPressed,
            last_main_key_press: None,
            sequence: Vec::new(),
            is_upper_case: false,
            lang: config.langs.keys().next().map(|s| s.clone()),
            work_state: InputWorkState::Work,
        }
    }
}

/// uses global mutable state
pub fn listen_loop(config: &Config, event_handler: fn(InputEvent, &mut InputState) -> ()) {
    let mut state: InputState = InputState::new(config);

    while state.work_state != InputWorkState::Exit {
        // handle main key press
        if let Some(main_key_hold_duration) = key_hold_duration(config.main, state.main_key_state) {
            if main_key_hold_duration < config.time_to_long_press {
                state.sequence.push(MorseKey::Dot);
            } else {
                state.sequence.push(MorseKey::Slash);
            }
            state.last_main_key_press = Some(SystemTime::now());
        }

        // handle other keys
        {
            if let Some(_) = key_hold_duration(config.change_lang, state.lang_key_state) {
                // if lang is None, then config not contains any lang
                if state.lang.is_some() {
                    // cyclically find next lang in config.langs HashMap
                    state.lang = config
                        .langs
                        .keys()
                        .skip_while(|&s| s != state.lang.as_ref().unwrap())
                        .cycle()
                        .next()
                        .map(|s| s.clone());
                } else {
                    state.lang = config.langs.keys().next().map(|s| s.clone());
                }
                event_handler(InputEvent::LangChange, &mut state);
            }

            if let Some(_) = key_hold_duration(config.change_case, state.change_case_key_state) {
                state.is_upper_case = !state.is_upper_case;
                event_handler(InputEvent::CaseChange, &mut state);
            }

            if let Some(_) = key_hold_duration(config.pause, state.pause_key_state) {
                state.work_state = match state.work_state {
                    InputWorkState::Pause => InputWorkState::Work,
                    InputWorkState::Work => InputWorkState::Pause,
                    InputWorkState::Exit => InputWorkState::Exit,
                };
                event_handler(InputEvent::PauseToggle, &mut state);
            }

            if let Some(_) = key_hold_duration(config.exit, state.exit_key_state) {
                state.work_state = InputWorkState::Exit;
                event_handler(InputEvent::Exit, &mut state);
            }
        }

        // handle morse sequence
        // main key is up && sequence is not empty && last main key press was long enough ago
        if let KeyState::NotPressed = state.main_key_state {
            if let Some(true) = state.last_main_key_press.map(|t| {
                t.elapsed().unwrap() > config.accept_sequence_delay && !state.sequence.is_empty()
            }) {
                match config.langs.get(state.lang.as_ref().unwrap()) {
                    Some(keys) => {
                        if let Some(config_key) = keys.get(&state.sequence) {
                            let curr_config_key = if state.is_upper_case {
                                config_key.upper.unwrap_or(config_key.lower)
                            } else {
                                config_key.lower
                            };
                            event_handler(
                                InputEvent::SequenceParsed(state.sequence.clone(), curr_config_key),
                                &mut state,
                            );
                        } else {
                            event_handler(
                                InputEvent::SeqRejected(
                                    state.sequence.clone(),
                                    SequenceRejectReason::InvalidSequence,
                                ),
                                &mut state,
                            );
                        }
                    }
                    None => {
                        event_handler(
                            InputEvent::SeqRejected(
                                state.sequence.clone(),
                                SequenceRejectReason::NoLangsLoaded,
                            ),
                            &mut state,
                        );
                    }
                }
                state.last_main_key_press = None;
                state.sequence.clear();
            }
        }

        // update key states
        update_key_state(&mut state.main_key_state, config.main);
        update_key_state(&mut state.lang_key_state, config.change_lang);
        update_key_state(&mut state.change_case_key_state, config.change_case);
        update_key_state(&mut state.pause_key_state, config.pause);
        update_key_state(&mut state.exit_key_state, config.exit);

        std::thread::sleep(config.listen_delay);
    }
}

fn update_key_state(state: &mut KeyState, key: enigo::Key) {
    match (is_key_down(key), &state) {
        (true, &KeyState::NotPressed) => {
            *state = KeyState::Down(std::time::SystemTime::now());
        }
        (false, _) => {
            *state = KeyState::NotPressed;
        }
        _ => {}
    };
}

/// returns none if key is not pressed or previous state was not pressed
fn key_hold_duration(key: enigo::Key, state: KeyState) -> Option<Duration> {
    match (is_key_down(key), state) {
        (false, KeyState::Down(time)) => Some(time.elapsed().unwrap()),
        _ => None,
    }
}

#[cfg(target_os = "windows")]
fn is_key_down(key: enigo::Key) -> bool {
    use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;

    // get most significant bit of return value
    // https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getasynckeystate#return-value
    unsafe { GetAsyncKeyState(key_code_into_virtual_key(key).0 as i32) < 0 }
}

#[cfg(target_os = "windows")]
fn key_code_into_virtual_key(key_code: enigo::Key) -> VIRTUAL_KEY {
    use enigo::Key;
    use windows::Win32::UI::Input::KeyboardAndMouse::*;

    match key_code {
        Key::Num0 => VK_NUMPAD0,
        Key::Num1 => VK_NUMPAD1,
        Key::Num2 => VK_NUMPAD2,
        Key::Num3 => VK_NUMPAD3,
        Key::Num4 => VK_NUMPAD4,
        Key::Num5 => VK_NUMPAD5,
        Key::Num6 => VK_NUMPAD6,
        Key::Num7 => VK_NUMPAD7,
        Key::Num8 => VK_NUMPAD8,
        Key::Num9 => VK_NUMPAD9,
        Key::A => VK_A,
        Key::B => VK_B,
        Key::C => VK_C,
        Key::D => VK_D,
        Key::E => VK_E,
        Key::F => VK_F,
        Key::G => VK_G,
        Key::H => VK_H,
        Key::I => VK_I,
        Key::J => VK_J,
        Key::K => VK_K,
        Key::L => VK_L,
        Key::M => VK_M,
        Key::N => VK_N,
        Key::O => VK_O,
        Key::P => VK_P,
        Key::Q => VK_Q,
        Key::R => VK_R,
        Key::S => VK_S,
        Key::T => VK_T,
        Key::U => VK_U,
        Key::V => VK_V,
        Key::W => VK_W,
        Key::X => VK_X,
        Key::Y => VK_Y,
        Key::Z => VK_Z,
        Key::AbntC1 => VK_ABNT_C1,
        Key::AbntC2 => VK_ABNT_C2,
        Key::Accept => VK_ACCEPT,
        Key::Add => VK_ADD,
        Key::Alt => VK_MENU,
        Key::Apps => VK_APPS,
        Key::Attn => VK_ATTN,
        Key::Backspace => VK_BACK,
        Key::BrowserBack => VK_BROWSER_BACK,
        Key::BrowserFavorites => VK_BROWSER_FAVORITES,
        Key::BrowserForward => VK_BROWSER_FORWARD,
        Key::BrowserHome => VK_BROWSER_HOME,
        Key::BrowserRefresh => VK_BROWSER_REFRESH,
        Key::BrowserSearch => VK_BROWSER_SEARCH,
        Key::BrowserStop => VK_BROWSER_STOP,
        Key::Cancel => VK_CANCEL,
        Key::CapsLock => VK_CAPITAL,
        Key::Clear => VK_CLEAR,
        Key::Control => VK_CONTROL,
        Key::Convert => VK_CONVERT,
        Key::Crsel => VK_CRSEL,
        Key::DBEAlphanumeric => VK_DBE_ALPHANUMERIC,
        Key::DBECodeinput => VK_DBE_CODEINPUT,
        Key::DBEDetermineString => VK_DBE_DETERMINESTRING,
        Key::DBEEnterDLGConversionMode => VK_DBE_ENTERDLGCONVERSIONMODE,
        Key::DBEEnterIMEConfigMode => VK_DBE_ENTERIMECONFIGMODE,
        Key::DBEEnterWordRegisterMode => VK_DBE_ENTERWORDREGISTERMODE,
        Key::DBEFlushString => VK_DBE_FLUSHSTRING,
        Key::DBEHiragana => VK_DBE_HIRAGANA,
        Key::DBEKatakana => VK_DBE_KATAKANA,
        Key::DBENoCodepoint => VK_DBE_NOCODEINPUT,
        Key::DBENoRoman => VK_DBE_NOROMAN,
        Key::DBERoman => VK_DBE_ROMAN,
        Key::DBESBCSChar => VK_DBE_SBCSCHAR,
        Key::DBESChar => {
            unimplemented!();
        }
        Key::Decimal => VK_DECIMAL,
        Key::Delete => VK_DELETE,
        Key::Divide => VK_DIVIDE,
        Key::DownArrow => VK_DOWN,
        Key::End => VK_END,
        Key::Ereof => VK_EREOF,
        Key::Escape => VK_ESCAPE,
        Key::Execute => VK_EXECUTE,
        Key::Exsel => VK_EXSEL,
        Key::F1 => VK_F1,
        Key::F2 => VK_F2,
        Key::F3 => VK_F3,
        Key::F4 => VK_F4,
        Key::F5 => VK_F5,
        Key::F6 => VK_F6,
        Key::F7 => VK_F7,
        Key::F8 => VK_F8,
        Key::F9 => VK_F9,
        Key::F10 => VK_F10,
        Key::F11 => VK_F11,
        Key::F12 => VK_F12,
        Key::F13 => VK_F13,
        Key::F14 => VK_F14,
        Key::F15 => VK_F15,
        Key::F16 => VK_F16,
        Key::F17 => VK_F17,
        Key::F18 => VK_F18,
        Key::F19 => VK_F19,
        Key::F20 => VK_F20,
        Key::F21 => VK_F21,
        Key::F22 => VK_F22,
        Key::F23 => VK_F23,
        Key::F24 => VK_F24,
        Key::Final => VK_FINAL,
        // Find => VK_FIND,
        Key::GamepadA => VK_GAMEPAD_A,
        Key::GamepadB => VK_GAMEPAD_B,
        Key::GamepadDPadDown => VK_GAMEPAD_DPAD_DOWN,
        Key::GamepadDPadLeft => VK_GAMEPAD_DPAD_LEFT,
        Key::GamepadDPadRight => VK_GAMEPAD_DPAD_RIGHT,
        Key::GamepadDPadUp => VK_GAMEPAD_DPAD_UP,
        Key::GamepadLeftShoulder => VK_GAMEPAD_LEFT_SHOULDER,
        Key::GamepadLeftThumbstickButton => VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON,
        Key::GamepadLeftThumbstickDown => VK_GAMEPAD_LEFT_THUMBSTICK_DOWN,
        Key::GamepadLeftThumbstickLeft => VK_GAMEPAD_LEFT_THUMBSTICK_LEFT,
        Key::GamepadLeftThumbstickRight => VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT,
        Key::GamepadLeftThumbstickUp => VK_GAMEPAD_LEFT_THUMBSTICK_UP,
        Key::GamepadLeftTrigger => VK_GAMEPAD_LEFT_TRIGGER,
        Key::GamepadMenu => VK_GAMEPAD_MENU,
        Key::GamepadRightShoulder => VK_GAMEPAD_RIGHT_SHOULDER,
        Key::GamepadRightThumbstickButton => VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON,
        Key::GamepadRightThumbstickDown => VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN,
        Key::GamepadRightThumbstickLeft => VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT,
        Key::GamepadRightThumbstickRight => VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT,
        Key::GamepadRightThumbstickUp => VK_GAMEPAD_RIGHT_THUMBSTICK_UP,
        Key::GamepadRightTrigger => VK_GAMEPAD_RIGHT_TRIGGER,
        Key::GamepadView => VK_GAMEPAD_VIEW,
        Key::GamepadX => VK_GAMEPAD_X,
        Key::GamepadY => VK_GAMEPAD_Y,
        Key::Hangeul => VK_HANGEUL,
        Key::Hangul => VK_HANGUL,
        Key::Hanja => VK_HANJA,
        Key::Help => VK_HELP,
        Key::Home => VK_HOME,
        Key::Ico00 => VK_ICO_00,
        Key::IcoClear => VK_ICO_CLEAR,
        Key::IcoHelp => VK_ICO_HELP,
        Key::IMEOff => VK_OEM_FINISH,
        Key::IMEOn => VK_OEM_COPY,
        Key::Insert => VK_INSERT,
        Key::Junja => VK_JUNJA,
        Key::Kana => VK_KANA,
        Key::Kanji => VK_KANJI,
        Key::LaunchApp1 => VK_LAUNCH_APP1,
        Key::LaunchApp2 => VK_LAUNCH_APP2,
        Key::LaunchMail => VK_LAUNCH_MAIL,
        Key::LaunchMediaSelect => VK_LAUNCH_MEDIA_SELECT,
        // Launchpad =>
        Key::LButton => VK_LBUTTON,
        Key::LControl => VK_LCONTROL,
        Key::LeftArrow => VK_LEFT,
        Key::LMenu => VK_LMENU,
        Key::LShift => VK_LSHIFT,
        Key::LWin => VK_LWIN,
        Key::MButton => VK_MBUTTON,
        Key::MediaNextTrack => VK_MEDIA_NEXT_TRACK,
        Key::MediaPlayPause => VK_MEDIA_PLAY_PAUSE,
        Key::MediaPrevTrack => VK_MEDIA_PREV_TRACK,
        Key::MediaStop => VK_MEDIA_STOP,
        Key::Meta => VK_LWIN,
        Key::ModeChange => VK_MODECHANGE,
        Key::Multiply => VK_MULTIPLY,
        Key::NavigationAccept => VK_NAVIGATION_ACCEPT,
        Key::NavigationCancel => VK_NAVIGATION_CANCEL,
        Key::NavigationDown => VK_NAVIGATION_DOWN,
        Key::NavigationLeft => VK_NAVIGATION_LEFT,
        Key::NavigationMenu => VK_NAVIGATION_MENU,
        Key::NavigationRight => VK_NAVIGATION_RIGHT,
        Key::NavigationUp => VK_NAVIGATION_UP,
        Key::NavigationView => VK_NAVIGATION_VIEW,
        Key::NoName => unimplemented!(),
        Key::NonConvert => VK_NONCONVERT,
        Key::None => unimplemented!(),
        Key::Numlock => VK_NUMLOCK,
        Key::Numpad0 => VK_NUMPAD0,
        Key::Numpad1 => VK_NUMPAD1,
        Key::Numpad2 => VK_NUMPAD2,
        Key::Numpad3 => VK_NUMPAD3,
        Key::Numpad4 => VK_NUMPAD4,
        Key::Numpad5 => VK_NUMPAD5,
        Key::Numpad6 => VK_NUMPAD6,
        Key::Numpad7 => VK_NUMPAD7,
        Key::Numpad8 => VK_NUMPAD8,
        Key::Numpad9 => VK_NUMPAD9,
        Key::OEM1 => VK_OEM_1,
        Key::OEM102 => VK_OEM_102,
        Key::OEM2 => VK_OEM_2,
        Key::OEM3 => VK_OEM_3,
        Key::OEM4 => VK_OEM_4,
        Key::OEM5 => VK_OEM_5,
        Key::OEM6 => VK_OEM_6,
        Key::OEM7 => VK_OEM_7,
        Key::OEM8 => VK_OEM_8,
        Key::OEMAttn => VK_OEM_ATTN,
        Key::OEMAuto => VK_OEM_AUTO,
        Key::OEMAx => VK_OEM_AX,
        Key::OEMBacktab => VK_OEM_BACKTAB,
        Key::OEMClear => VK_OEM_CLEAR,
        Key::OEMComma => VK_OEM_COMMA,
        Key::OEMCopy => VK_OEM_COPY,
        Key::OEMCusel => VK_OEM_CUSEL,
        Key::OEMEnlw => VK_OEM_ENLW,
        Key::OEMFinish => VK_OEM_FINISH,
        Key::OEMFJJisho => VK_OEM_FJ_JISHO,
        Key::OEMFJLoya => VK_OEM_FJ_LOYA,
        Key::OEMFJMasshou => VK_OEM_FJ_MASSHOU,
        Key::OEMFJRoya => VK_OEM_FJ_ROYA,
        Key::OEMFJTouroku => VK_OEM_FJ_TOUROKU,
        Key::OEMJump => VK_OEM_JUMP,
        Key::OEMMinus => VK_OEM_MINUS,
        Key::OEMNECEqual => VK_OEM_NEC_EQUAL,
        Key::OEMPA1 => VK_OEM_PA1,
        Key::OEMPA2 => VK_OEM_PA2,
        Key::OEMPA3 => VK_OEM_PA3,
        Key::OEMPeriod => VK_OEM_PERIOD,
        Key::OEMPlus => VK_OEM_PLUS,
        Key::OEMReset => VK_OEM_RESET,
        Key::OEMWsctrl => VK_OEM_WSCTRL,
        Key::Option => VK_LMENU,
        Key::PA1 => VK_PA1,
        Key::Packet => VK_PACKET,
        Key::PageDown => VK_NEXT,
        Key::PageUp => VK_PRIOR,
        Key::Pause => VK_PAUSE,
        Key::Play => VK_PLAY,
        Key::Print => VK_PRINT,
        Key::Processkey => VK_PROCESSKEY,
        Key::RButton => VK_RBUTTON,
        Key::RControl => VK_RCONTROL,
        Key::Return => VK_RETURN,
        Key::RightArrow => VK_RIGHT,
        Key::RMenu => VK_RMENU,
        Key::RShift => VK_RSHIFT,
        Key::RWin => VK_RWIN,
        Key::Scroll => VK_SCROLL,
        Key::Select => VK_SELECT,
        Key::Separator => VK_SEPARATOR,
        Key::Shift => VK_SHIFT,
        Key::Sleep => VK_SLEEP,
        Key::Snapshot => VK_SNAPSHOT,
        Key::Space => VK_SPACE,
        Key::Subtract => VK_SUBTRACT,
        Key::Tab => VK_TAB,
        Key::UpArrow => VK_UP,
        Key::VolumeDown => VK_VOLUME_DOWN,
        Key::VolumeMute => VK_VOLUME_MUTE,
        Key::VolumeUp => VK_VOLUME_UP,
        Key::XButton1 => VK_XBUTTON1,
        Key::XButton2 => VK_XBUTTON2,
        Key::Zoom => VK_ZOOM,
        Key::Layout(_character) => unimplemented!(),
        Key::Raw(_) => unimplemented!(),
        _ => unimplemented!(),
    }
}
