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

mod keybindings;
mod statusline;
mod view;

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                  IMPORTS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

use std::{sync::mpsc, thread};

use log::debug;
use ratatui::{
    layout::{Constraint, Layout},
    DefaultTerminal,
};
use statusline::StatusLine;
use view::View;

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
    /// Move the focused view in the specified direction
    MoveFocus(Direction),
    /// Move the cursor in the specified direction
    MoveCursor(Direction),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Horizontal,
}

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                  STRUCTS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

/// Contains the TUI application.
#[derive(Debug)]
struct App {
    /// All Stars, Planets, and Comets.
    galaxy: Galaxy,

    /// The current mode of the application.
    mode: AppMode,

    /// The root view of the application.
    view: Box<dyn view::View>,
    /// The status line displayed at the bottom of the screen.
    status: statusline::StatusLine,
}

impl App {
    /// Creates a new application struct for the TUI
    ///
    /// # Errors
    /// - Will error if loading the `Galaxy` fails
    fn new() -> Result<Self> {
        Ok(Self {
            galaxy: Galaxy::load()?,
            mode: AppMode::default(),
            view: Box::new(view::OpeningView),
            status: StatusLine::default(),
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
    fn run(mut self, mut terminal: DefaultTerminal, rx: mpsc::Receiver<AppCommand>) -> Result<()> {
        loop {
            terminal.draw(|frame| {
                let layout = Layout::default()
                    .direction(ratatui::layout::Direction::Vertical)
                    .constraints([Constraint::Fill(0), Constraint::Length(2)])
                    .split(frame.area());
                self.view.render(&self, layout[0], frame.buffer_mut());
                self.status.render(&self, layout[1], frame.buffer_mut());
            })?;

            let command = rx.recv()?;
            debug!("Application received command from channel: {command:?}");
            match command {
                AppCommand::Quit => break Ok(()),
                AppCommand::Redraw => {} // will redraw on next iteration of loop
                AppCommand::UpdateMode(mode) => self.mode = mode,
                AppCommand::MoveCursor(direction) => todo!(),
                AppCommand::MoveFocus(direction) => todo!(),
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
