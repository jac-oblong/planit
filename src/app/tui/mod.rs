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
 * Contains the implementation for the terminal user interface.
 */

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                  MODULES                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

pub mod keybindings;

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                  IMPORTS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

use std::{sync::mpsc, thread};

use log::debug;
use ratatui::{text::Line, DefaultTerminal};

use crate::core::Galaxy;

use super::Result;

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                   ENUMS                                    //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

/// Current mode of the application. This is primarily based around Vim's modes.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum AppMode {
    #[default]
    Normal,
    Command,
    Insert,
}

/// Commands that the application recognizes. Commands are not necessarily
/// guaranteed to succeed. Errors will not be communicated back to the sender of
/// the command.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AppCommand {
    /// Attempt to exit immediately
    Quit,
    /// Redraw the screen, no other operations are necessary
    Redraw,
    /// Update the application's mode to be the mode provided
    UpdateMode(AppMode),
}

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                  STRUCTS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

/// Contains the TUI application.
#[derive(Debug)]
struct App {
    galaxy: Galaxy,
}

impl App {
    /// Creates a new application struct for the TUI
    ///
    /// # Errors
    /// - Will error if loading the `Galaxy` fails
    fn new() -> Result<Self> {
        Ok(Self {
            galaxy: Galaxy::load()?,
        })
    }

    /// Runs the application. Will return any errors that are encountered.
    ///
    /// # Arguments
    /// - `terminal`: The default terminal to be drawn to. Should be the result
    ///   of `ratatui::init()`
    /// - `rx`: The channel that app commands will be sent through.
    ///
    /// # Errors
    /// - Error while drawing to terminal
    /// - Error while receiving commands from the channel
    fn run(self, mut terminal: DefaultTerminal, rx: mpsc::Receiver<AppCommand>) -> Result<()> {
        loop {
            terminal
                .draw(|frame| frame.render_widget(Line::from("Planit").centered(), frame.area()))?;

            let command = rx.recv()?;
            debug!("Application received command from channel: {command:?}");
            match command {
                AppCommand::Quit => break Ok(()),
                AppCommand::Redraw => {} // will redraw on next iteration of loop
                AppCommand::UpdateMode(mode) => todo!(),
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                 FUNCTIONS                                  //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

/// Runs the TUI application. Will return when the application should exit.
///
/// # Errors
/// - Any errors encountered during creation of the main application structure
/// - Any errors encountered during running of the TUI application
pub fn run() -> Result<()> {
    let terminal = ratatui::init();
    let (tx, rx) = mpsc::channel::<AppCommand>();
    let app = App::new()?;

    thread::spawn(move || keybindings::handle_keyboard_input(tx));
    app.run(terminal, rx)?;
    ratatui::restore();

    Ok(())
}
