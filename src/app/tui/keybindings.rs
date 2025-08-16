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
//                                  MODULES                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

use std::{sync::mpsc, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use super::AppCommand;

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                   CONSTS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

const KEY_SEQUENCE_TIMEOUT_MS: u64 = 500;
const KEY_SEQUENCE_RESOLUTION_MS: u64 = 10;

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                   ENUMS                                    //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

/// Current keybindings mode
#[derive(Debug, Default)]
enum Mode {
    #[default]
    Normal,
    Command,
    Insert,
}

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                  STRUCTS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

struct KeyBind {
    keys: &'static str,
    command: AppCommand,
}

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                 FUNCTIONS                                  //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

pub fn handle_keyboard_input(tx: mpsc::Sender<AppCommand>) -> ! {
    let mut mode = Mode::default();

    loop {
        let res = event::poll(Duration::from_millis(KEY_SEQUENCE_RESOLUTION_MS)).unwrap();
        if !res {
            continue;
        }

        let input = event::read().unwrap();
        match input {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                match key.code {
                    KeyCode::Char('q') => {
                        tx.send(AppCommand::Quit).unwrap();
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }
}
