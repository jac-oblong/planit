////////////////////////////////////////////////////////////////////////////
// The MIT License (MIT)                                                  //
//                                                                        //
// Copyright (c) 2025 Jacob Long                                          //
//                                                                        //
// Permission is hereby granted, free of charge, to any person obtaining  //
// a copy of this software and associated documentation files (the        //
// "Software"), to deal in the Software without restriction, including    //
// without limitation the rights to use, copy, modify, merge, publish,    //
// distribute, sublicense, and/or sell copies of the Software, and to     //
// permit persons to whom the Software is furnished to do so, subject to  //
// the following conditions:                                              //
//                                                                        //
// The above copyright notice and this permission notice shall be         //
// included in all copies or substantial portions of the Software.        //
//                                                                        //
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,        //
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF     //
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. //
// IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY   //
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,   //
// TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE      //
// SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.                 //
////////////////////////////////////////////////////////////////////////////

/*!
 * Contains the implementation for receiving key inputs and translating them to
 * app commands. The function is blocking, so should only be called in a
 * separate thread.
 */

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                  IMPORTS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

use std::{
    collections::{HashMap, VecDeque},
    sync::mpsc,
    time::Duration,
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use log::{debug, error, warn};

use super::{Command, Mode};

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                   CONSTS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

const KEY_SEQUENCE_TIMEOUT_MS: u64 = 500;
const KEY_SEQUENCE_RESOLUTION_MS: u64 = 10;
const KEY_SEQUENCE_ITERATIONS: u64 = KEY_SEQUENCE_TIMEOUT_MS / KEY_SEQUENCE_RESOLUTION_MS;

const NORMAL_MODE_KEYBINDINGS: &[(&'static str, Command)] = &[
    ("i", Command::UpdateMode(Mode::Insert)),
    (":", Command::UpdateMode(Mode::Command)),
    ("q", Command::Quit),
];
const COMMAND_MODE_KEYBINDINGS: &[(&'static str, Command)] =
    &[("j k", Command::UpdateMode(Mode::Normal))];
const INSERT_MODE_KEYBINDINGS: &[(&'static str, Command)] =
    &[("j k", Command::UpdateMode(Mode::Normal))];

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                  STRUCTS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

/// A single key in a key binding sequence. Uses `crossterm`'s `KeyCode` and
/// `KeyModifiers` types.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Key {
    /// The code of the key.
    code: KeyCode,
    /// The modifiers for the key.
    modifiers: KeyModifiers,
}

impl From<&'static str> for Key {
    fn from(value: &'static str) -> Self {
        let mut value: Vec<_> = value.split("+").collect();
        let code = value.pop().unwrap();
        let code = match code {
            "Backspace" => KeyCode::Backspace,
            "Delete" => KeyCode::Delete,
            "Enter" => KeyCode::Enter,
            "Left" => KeyCode::Left,
            "Right" => KeyCode::Right,
            "Up" => KeyCode::Up,
            "Down" => KeyCode::Down,
            "Home" => KeyCode::Home,
            "End" => KeyCode::End,
            "PageUp" => KeyCode::PageUp,
            "PageDown" => KeyCode::PageDown,
            "Tab" => KeyCode::Tab,
            "BackTab" => KeyCode::BackTab,
            "Insert" => KeyCode::Insert,
            "Null" => KeyCode::Null,
            "Escape" => KeyCode::Esc,
            "CapsLock" => KeyCode::CapsLock,
            "ScrollLock" => KeyCode::ScrollLock,
            "NumLock" => KeyCode::NumLock,
            "PrintScreen" => KeyCode::PrintScreen,
            "Pause" => KeyCode::Pause,
            "Menu" => KeyCode::Menu,
            "Begin" => KeyCode::KeypadBegin,
            "Space" => KeyCode::Char(' '),
            code => match code.len() {
                1 => KeyCode::Char(code.chars().next().unwrap()),
                _ => panic!("Unrecognized key code: {code}"),
            },
        };

        let mut modifiers = KeyModifiers::empty();
        for modifier in value {
            match modifier {
                "Shift" => modifiers.insert(KeyModifiers::SHIFT),
                "Control" => modifiers.insert(KeyModifiers::CONTROL),
                "Alt" => modifiers.insert(KeyModifiers::ALT),
                "Super" => modifiers.insert(KeyModifiers::SUPER),
                "Hyper" => modifiers.insert(KeyModifiers::HYPER),
                "Meta" => modifiers.insert(KeyModifiers::META),
                modifier => panic!("Unrecognized modifeir: {modifier}"),
            }
        }

        Self { code, modifiers }
    }
}

impl From<KeyEvent> for Key {
    fn from(value: KeyEvent) -> Self {
        Self {
            code: value.code,
            modifiers: value.modifiers,
        }
    }
}

impl From<&KeyEvent> for Key {
    fn from(value: &KeyEvent) -> Self {
        Self {
            code: value.code,
            modifiers: value.modifiers,
        }
    }
}

/// A "node" in a key binding sequence. It can end with a app command. It can
/// also continue with more keys in the sequence.
#[derive(Debug, Default, Clone, PartialEq)]
struct KeyBind {
    /// The command to use, if this is the end of a key sequence.
    command: Option<Command>,
    /// Other keys that are accepted in the continuation of this sequence.
    sequence: Option<KeyBinds>,
}

impl KeyBind {
    /// Inserts a command or sequence at this point
    pub fn insert(&mut self, keys: VecDeque<Key>, command: Command) {
        if keys.len() == 0 {
            if self.command.is_some() {
                warn!(
                    "Overriding keybinding for command {:?} with command {:?}",
                    self.command, command
                );
            }
            self.command = Some(command);
        } else {
            match &mut self.sequence {
                Some(sequence) => sequence.insert(keys, command),
                None => {
                    let mut sequence = KeyBinds::default();
                    sequence.insert(keys, command);
                    self.sequence = Some(sequence);
                }
            }
        }
    }
}

/// The bindings from a key to a command and / or a key sequence. The key
/// sequence must eventually end in a command.
#[derive(Debug, Default, Clone, PartialEq)]
struct KeyBinds(HashMap<Key, KeyBind>);

impl KeyBinds {
    /// Inserts the keybinding
    pub fn insert(&mut self, mut keys: VecDeque<Key>, command: Command) {
        let first = keys.pop_front().unwrap();
        let sequence = self.0.entry(first).or_insert(KeyBind::default());
        sequence.insert(keys, command);
    }

    /// Returns the key bind associated with the keys, if any exists. The key
    /// bind will be found by following the keys provided recursively through
    /// the data structure.
    ///
    /// # Arguments
    /// - `keys`: The keys with which to find the sequence.
    ///
    /// # Returns
    /// A key bind, if any exists for the keys.
    pub fn get_key_bind(&self, keys: &[KeyEvent]) -> Option<&KeyBind> {
        let mut binds = self;
        let mut sequence = None;
        let mut itr = keys.iter().peekable();

        while let Some(key) = itr.next() {
            match binds.0.get(&Key::from(key)) {
                // found a continuation of the key sequence
                Some(key_sequence) => {
                    sequence = Some(key_sequence);
                    // need to verify that the key bind has possibility for more
                    // keys in the sequence
                    match &key_sequence.sequence {
                        Some(key_binds) => binds = &key_binds,
                        None => {
                            // found a key bind that has no continuation, but
                            // the input sequence of keys is not expired
                            if itr.peek().is_some() {
                                sequence = None;
                                break;
                            }
                        }
                    }
                }
                // the key sequence has ended, return `None`
                None => {
                    sequence = None;
                    break;
                }
            }
        }

        sequence
    }

    /// Matches the sequence of keys to key bindings. Any matched keys will be
    /// removed from the queue. Keys will only be matched if there is no
    /// possibilty for future keys to match a different key binding.
    ///
    /// # Arguments
    /// - `keys`: The queue of keys to match to key bindings. This queue is
    ///   guaranteed to be empty when this function returns.
    ///
    /// # Returns
    /// A vector containing all commands that were found.
    pub fn try_match(&self, keys: &mut Vec<KeyEvent>) -> Vec<Command> {
        let mut commands = Vec::new();
        let mut previous_command: Option<Command> = None;
        let mut previous_end: usize = 0;
        let mut previous_bind_had_continuation: bool = false;

        let mut current_end: usize = 1;
        while !keys.is_empty() {
            if current_end <= keys.len()
                && let Some(bind) = self.get_key_bind(&keys[..current_end])
            {
                // Key sequence continues. Save the previous command (if
                // applicable), and increment the current end.
                if let Some(command) = bind.command {
                    previous_command = Some(command);
                    previous_end = current_end;
                }
                previous_bind_had_continuation = bind.sequence.is_some();
                current_end += 1;
            } else {
                // Key sequence has been broken. If there is potentially more in
                // the key sequence and it has been broken because there are no
                // more keys left, we do not consume the keys because future key
                // presses can complete the key sequence. Otherwise, use the
                // previous command (if applicable), drain the keys, and reset
                // the ends
                if current_end > keys.len() && previous_bind_had_continuation {
                    break;
                } else if let Some(command) = previous_command {
                    commands.push(command);
                    keys.drain(..previous_end);
                    // If we change modes, do not match more keys, as the key
                    // bindings have changed.
                    match command {
                        Command::UpdateMode(_) => break,
                        _ => {}
                    }
                } else {
                    keys.drain(..current_end);
                }
                previous_command = None;
                previous_end = 0;
                current_end = 1;
            }
        }

        commands
    }

    /// Matches the sequence of keys to key bindings. Any matched keys will be
    /// removed from the queue. All keys will either be matched to a keybinding
    /// or discarded. The only exception to this is if a
    /// `Command::UpdateMode` command is matched. In this case, all keys
    /// after this command will remain.
    ///
    /// # Arguments
    /// - `keys`: The queue of keys to match to key bindings.
    ///
    /// # Returns
    /// A vector containing all commands that were found.
    pub fn force_match(&self, keys: &mut Vec<KeyEvent>) -> Vec<Command> {
        let mut commands = Vec::new();
        let mut previous_command: Option<Command> = None;
        let mut previous_end: usize = 0;

        let mut current_end: usize = 1;
        while !keys.is_empty() {
            if current_end <= keys.len()
                && let Some(bind) = self.get_key_bind(&keys[..current_end])
            {
                // Key sequence continues. Save the previous command (if
                // applicable), and increment the current end.
                if let Some(command) = bind.command {
                    previous_command = Some(command);
                    previous_end = current_end;
                }
                current_end += 1;
            } else {
                // Key sequence has been broken. Use the previous command
                // (if applicable), drain the keys, and reset the ends
                if let Some(command) = previous_command {
                    commands.push(command);
                    keys.drain(..previous_end);
                    // If we change modes, do not match more keys, as the key
                    // bindings have changed. Do not drain the keys either.
                    match command {
                        Command::UpdateMode(_) => break,
                        _ => {}
                    }
                } else if current_end <= keys.len() {
                    keys.drain(..current_end);
                } else {
                    keys.clear();
                }
                previous_command = None;
                previous_end = 0;
                current_end = 1;
            }
        }

        commands
    }
}

impl From<&[(&'static str, Command)]> for KeyBinds {
    fn from(value: &[(&'static str, Command)]) -> Self {
        let mut bindings = Self::default();

        for (sequence, command) in value {
            let keys: VecDeque<Key> = sequence.split(" ").map(Key::from).collect();
            match keys.len() {
                0 => error!("Empty string in key binding"),
                _ => bindings.insert(keys, command.clone()),
            }
        }

        bindings
    }
}

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                 FUNCTIONS                                  //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

/// Handles keyboard input and translates them to app commands using the key
/// bindings. NOTE: This function is intended to be launched in a separate
/// thread as it does not return.
///
/// # Arguments
/// - `tx`: The channel to send app commands through.
///
/// # Panics
/// - Will panic if there is a error while polling / reading events.
///   `crossterm::event::poll` and `crossterm::event::read` are the two
///   functions that may return errors that cause panics.
pub fn handle_keyboard_input(tx: mpsc::Sender<Command>) -> ! {
    /// Controls how the input queue is matched to bindings. This is reset to
    /// `NoMatch` at every iteration of the loop and it is up to the logic
    /// within the loop to try to match or force matches.
    #[derive(PartialEq)]
    enum ShouldMatch {
        NoMatch,
        TryMatch,
        ForceMatch,
    }

    // The mode of the application. Anytime this is changed, the change should\
    // also be sent through the channel.
    let mut mode = Mode::default();
    // Queue of key inputs. This is built up to match key sequences. Once a
    // sequence is matched (and no other sequences have partial matches) or the
    // sequence has timed out, the keys will be processed.
    let mut input_queue = Vec::new();
    // Counter for how long since the last key input.
    let mut iterations_since_last_key: u64 = 0;
    // key bindings for different modes
    let normal_binds = KeyBinds::from(NORMAL_MODE_KEYBINDINGS);
    let command_binds = KeyBinds::from(COMMAND_MODE_KEYBINDINGS);
    let insert_binds = KeyBinds::from(INSERT_MODE_KEYBINDINGS);

    loop {
        let res = event::poll(Duration::from_millis(KEY_SEQUENCE_RESOLUTION_MS)).unwrap();
        let mut should_match = ShouldMatch::NoMatch;

        if res {
            let input = event::read().unwrap();
            debug!("Keyboard Input Handler read key: {input:?}");
            if let Event::Key(key) = input
                && key.kind == KeyEventKind::Press
            {
                input_queue.push(key);
                iterations_since_last_key = 0;
                should_match = ShouldMatch::TryMatch;
            }
        } else {
            iterations_since_last_key += 1;
            if iterations_since_last_key >= KEY_SEQUENCE_ITERATIONS {
                iterations_since_last_key = 0;
                should_match = ShouldMatch::ForceMatch;
            }
        }

        if should_match == ShouldMatch::NoMatch {
            continue;
        }

        loop {
            let mut mode_has_not_changed = true;
            let binds = match mode {
                Mode::Normal => &normal_binds,
                Mode::Command => &command_binds,
                Mode::Insert => &insert_binds,
            };

            let commands = match should_match {
                ShouldMatch::TryMatch => binds.try_match(&mut input_queue),
                ShouldMatch::ForceMatch => binds.force_match(&mut input_queue),
                ShouldMatch::NoMatch => {
                    panic!("Keyboard Input Handler matching keys when not supposed to")
                }
            };

            for command in commands {
                match command {
                    Command::UpdateMode(new_mode) => {
                        mode = new_mode;
                        mode_has_not_changed = false;
                    }
                    _ => {}
                }
                debug!("Keyboard Input Handler sending command: {command:?}");
                tx.send(command).unwrap();
            }

            if mode_has_not_changed {
                break;
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                   TESTS                                    //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod test {
    use super::*;

    const KEY_BINDS: &[(&'static str, Command)] = &[
        ("a b", Command::Quit),
        ("a b c", Command::Redraw),
        ("x y", Command::UpdateMode(Mode::Command)),
    ];

    #[test]
    fn key_from_string_works_correctly() {
        assert_eq!(
            Key::from("a"),
            Key {
                code: KeyCode::Char('a'),
                modifiers: KeyModifiers::empty()
            }
        );
        assert_eq!(
            Key::from("Control+a"),
            Key {
                code: KeyCode::Char('a'),
                modifiers: KeyModifiers::CONTROL,
            }
        );
        assert_eq!(
            Key::from("Alt+Control+!"),
            Key {
                code: KeyCode::Char('!'),
                modifiers: KeyModifiers::CONTROL | KeyModifiers::ALT,
            }
        );
        assert_eq!(
            Key::from("Backspace"),
            Key {
                code: KeyCode::Backspace,
                modifiers: KeyModifiers::empty()
            }
        );
        assert_eq!(
            Key::from("Alt+Space"),
            Key {
                code: KeyCode::Char(' '),
                modifiers: KeyModifiers::ALT,
            }
        );
        assert_eq!(
            Key::from("Control+Alt+Enter"),
            Key {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::CONTROL | KeyModifiers::ALT,
            }
        );
    }

    #[test]
    #[should_panic]
    fn key_from_string_panics_when_code_not_recognized() {
        let _ = Key::from("ab");
    }

    #[test]
    #[should_panic]
    fn key_from_string_panics_when_modifier_not_recognized() {
        let _ = Key::from("Modifier+a");
    }

    #[test]
    fn key_binds_from_slice_works_correctly() {
        let binds = KeyBinds::from(KEY_BINDS);
        assert_eq!(
            binds,
            KeyBinds(HashMap::from([
                (
                    Key::from("a"),
                    KeyBind {
                        command: None,
                        sequence: Some(KeyBinds(HashMap::from([(
                            Key::from("b"),
                            KeyBind {
                                command: Some(Command::Quit),
                                sequence: Some(KeyBinds(HashMap::from([(
                                    Key::from("c"),
                                    KeyBind {
                                        command: Some(Command::Redraw),
                                        sequence: None
                                    }
                                )])))
                            }
                        )])))
                    }
                ),
                (
                    Key::from("x"),
                    KeyBind {
                        command: None,
                        sequence: Some(KeyBinds(HashMap::from([(
                            Key::from("y"),
                            KeyBind {
                                command: Some(Command::UpdateMode(Mode::Command)),
                                sequence: None
                            }
                        )])))
                    }
                ),
            ]))
        );
    }

    #[test]
    fn try_match_perfect_match_returns_command() {
        let binds = KeyBinds::from(KEY_BINDS);
        let mut keys = vec![
            KeyEvent::from(KeyCode::Char('a')),
            KeyEvent::from(KeyCode::Char('b')),
            KeyEvent::from(KeyCode::Char('c')),
        ];
        let commands = binds.try_match(&mut keys);
        assert_eq!(keys, vec![]);
        assert_eq!(commands, vec![Command::Redraw]);
    }

    #[test]
    fn try_match_no_match_returns_nothing_and_consumes_keys() {
        let binds = KeyBinds::from(KEY_BINDS);
        let mut keys = vec![
            KeyEvent::from(KeyCode::Char('j')),
            KeyEvent::from(KeyCode::Char('k')),
        ];
        let commands = binds.try_match(&mut keys);
        assert_eq!(keys, vec![]);
        assert_eq!(commands, vec![]);
    }

    #[test]
    fn try_match_partial_match_returns_nothing() {
        let binds = KeyBinds::from(KEY_BINDS);
        let mut keys = vec![KeyEvent::from(KeyCode::Char('a'))];
        let commands = binds.try_match(&mut keys);
        assert_eq!(keys, vec![KeyEvent::from(KeyCode::Char('a'))]);
        assert_eq!(commands, vec![]);
    }

    #[test]
    fn try_match_perfect_and_partial_match_returns_nothing() {
        let binds = KeyBinds::from(KEY_BINDS);
        let mut keys = vec![
            KeyEvent::from(KeyCode::Char('a')),
            KeyEvent::from(KeyCode::Char('b')),
        ];
        let commands = binds.try_match(&mut keys);
        assert_eq!(
            keys,
            vec![
                KeyEvent::from(KeyCode::Char('a')),
                KeyEvent::from(KeyCode::Char('b')),
            ]
        );
        assert_eq!(commands, vec![]);
    }

    #[test]
    fn try_match_perfect_match_with_extra_returns_command() {
        let binds = KeyBinds::from(KEY_BINDS);
        let mut keys = vec![
            KeyEvent::from(KeyCode::Char('a')),
            KeyEvent::from(KeyCode::Char('b')),
            KeyEvent::from(KeyCode::Char('x')),
        ];
        let commands = binds.try_match(&mut keys);
        assert_eq!(keys, vec![KeyEvent::from(KeyCode::Char('x'))]);
        assert_eq!(commands, vec![Command::Quit]);
    }

    #[test]
    fn try_match_stops_when_mode_chages() {
        let binds = KeyBinds::from(KEY_BINDS);
        let mut keys = vec![
            KeyEvent::from(KeyCode::Char('a')),
            KeyEvent::from(KeyCode::Char('b')),
            KeyEvent::from(KeyCode::Char('x')),
            KeyEvent::from(KeyCode::Char('y')),
            KeyEvent::from(KeyCode::Char('a')),
            KeyEvent::from(KeyCode::Char('b')),
            KeyEvent::from(KeyCode::Char('c')),
        ];
        let commands = binds.try_match(&mut keys);
        assert_eq!(
            keys,
            vec![
                KeyEvent::from(KeyCode::Char('a')),
                KeyEvent::from(KeyCode::Char('b')),
                KeyEvent::from(KeyCode::Char('c')),
            ]
        );
        assert_eq!(
            commands,
            vec![Command::Quit, Command::UpdateMode(Mode::Command)]
        );
    }

    #[test]
    fn force_match_perfect_match_returns_command() {
        let binds = KeyBinds::from(KEY_BINDS);
        let mut keys = vec![
            KeyEvent::from(KeyCode::Char('a')),
            KeyEvent::from(KeyCode::Char('b')),
            KeyEvent::from(KeyCode::Char('c')),
        ];
        let commands = binds.force_match(&mut keys);
        assert_eq!(keys, vec![]);
        assert_eq!(commands, vec![Command::Redraw]);
    }

    #[test]
    fn force_match_no_match_returns_nothing_and_consumes_keys() {
        let binds = KeyBinds::from(KEY_BINDS);
        let mut keys = vec![
            KeyEvent::from(KeyCode::Char('j')),
            KeyEvent::from(KeyCode::Char('k')),
        ];
        let commands = binds.force_match(&mut keys);
        assert_eq!(keys, vec![]);
        assert_eq!(commands, vec![]);
    }

    #[test]
    fn force_match_partial_match_returns_nothing_and_consumes_keys() {
        let binds = KeyBinds::from(KEY_BINDS);
        let mut keys = vec![KeyEvent::from(KeyCode::Char('a'))];
        let commands = binds.force_match(&mut keys);
        assert_eq!(keys, vec![]);
        assert_eq!(commands, vec![]);
    }

    #[test]
    fn force_match_perfect_and_partial_match_returns_command_and_consumes_keys() {
        let binds = KeyBinds::from(KEY_BINDS);
        let mut keys = vec![
            KeyEvent::from(KeyCode::Char('a')),
            KeyEvent::from(KeyCode::Char('b')),
        ];
        let commands = binds.force_match(&mut keys);
        assert_eq!(keys, vec![]);
        assert_eq!(commands, vec![Command::Quit]);
    }

    #[test]
    fn force_match_perfect_match_with_extra_returns_command() {
        let binds = KeyBinds::from(KEY_BINDS);
        let mut keys = vec![
            KeyEvent::from(KeyCode::Char('a')),
            KeyEvent::from(KeyCode::Char('b')),
            KeyEvent::from(KeyCode::Char('x')),
        ];
        let commands = binds.force_match(&mut keys);
        assert_eq!(keys, vec![]);
        assert_eq!(commands, vec![Command::Quit]);
    }

    #[test]
    fn force_match_stops_when_mode_chages() {
        let binds = KeyBinds::from(KEY_BINDS);
        let mut keys = vec![
            KeyEvent::from(KeyCode::Char('a')),
            KeyEvent::from(KeyCode::Char('b')),
            KeyEvent::from(KeyCode::Char('x')),
            KeyEvent::from(KeyCode::Char('y')),
            KeyEvent::from(KeyCode::Char('a')),
            KeyEvent::from(KeyCode::Char('b')),
            KeyEvent::from(KeyCode::Char('c')),
        ];
        let commands = binds.force_match(&mut keys);
        assert_eq!(
            keys,
            vec![
                KeyEvent::from(KeyCode::Char('a')),
                KeyEvent::from(KeyCode::Char('b')),
                KeyEvent::from(KeyCode::Char('c')),
            ]
        );
        assert_eq!(
            commands,
            vec![Command::Quit, Command::UpdateMode(Mode::Command)]
        );
    }
}
