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

use std::{cell::RefCell, collections::VecDeque, rc::Rc, sync::mpsc, thread};

use log::debug;
use ratatui::{
    layout::{Constraint, Layout},
    DefaultTerminal,
};
use statusline::StatusLine;
use view::{PaneView, View};

use crate::{core::Galaxy, util::queue::PushQueue};

use super::Result;

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                   ENUMS                                    //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

/// Current mode of the application. This is primarily based around Vim's modes.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum Mode {
    #[default]
    Normal,
    Command,
    Insert,
}

/// Commands that the application recognizes. Commands are not necessarily
/// guaranteed to succeed. Errors will not be communicated back to the sender of
/// the command.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Command {
    /// Attempt to exit immediately.
    Quit,
    /// Redraw the screen, no other operations are necessary.
    Redraw,
    /// Update the application's mode to be the mode provided.
    UpdateMode(Mode),
    /// Move the focused view in the specified direction.
    MoveFocus(MovementDirection),
    /// Move the cursor in the specified direction.
    MoveCursor(MovementDirection),
    /// Split the currently focused view into two using the given direction. The
    /// newly created view should be focused.
    SplitView(SplitDirection),
}

/// A direction that things can move in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MovementDirection {
    Up,
    Down,
    Left,
    Right,
}

/// A direction that things can be split in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitDirection {
    Horizontal,
    Vertical,
}

impl Into<ratatui::layout::Direction> for SplitDirection {
    fn into(self) -> ratatui::layout::Direction {
        // NOTE: ratatui defines its layout direction in terms of stacking, not
        // in terms of where the split is. Thus for a horizontal split, elements
        // are stacked vertically, and vice versa. I find it easier to think
        // about things based on how they split, not on how they stack, so I
        // have decided to use a separate type with inverted values.
        match self {
            SplitDirection::Horizontal => ratatui::layout::Direction::Vertical,
            SplitDirection::Vertical => ratatui::layout::Direction::Horizontal,
        }
    }
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
    #[allow(dead_code)]
    galaxy: Rc<RefCell<Galaxy>>,

    /// The current mode of the application.
    mode: Mode,

    /// The root view of the application.
    view: PaneView,
    /// The status line displayed at the bottom of the screen.
    status: statusline::StatusLine,
}

impl App {
    /// Creates a new application struct for the TUI
    ///
    /// # Errors
    /// - Will error if loading the `Galaxy` fails
    fn new() -> Result<Self> {
        let galaxy = Rc::new(RefCell::new(Galaxy::load()?));
        Ok(Self {
            galaxy: galaxy.clone(),
            mode: Mode::default(),
            view: PaneView::new(galaxy.clone()),
            status: StatusLine::new(galaxy),
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
    fn run(mut self, mut terminal: DefaultTerminal, rx: mpsc::Receiver<Command>) -> Result<()> {
        'outer: loop {
            terminal.draw(|frame| {
                let layout = Layout::default()
                    .direction(ratatui::layout::Direction::Vertical)
                    .constraints([Constraint::Fill(0), Constraint::Length(2)])
                    .split(frame.area());
                self.view.render(layout[0], frame.buffer_mut());
                self.status.render(layout[1], frame.buffer_mut());
            })?;

            let command = rx.recv()?;
            debug!("Application received command from channel: {command:?}");
            let mut commands = VecDeque::from([command]);

            loop {
                let command = commands.pop_front().unwrap();
                debug!("Application processing command: {command:?}");

                let mut queue = PushQueue::from(commands);
                match command {
                    Command::Quit => break 'outer Ok(()),
                    Command::Redraw => {} // will redraw on next iteration of loop
                    Command::UpdateMode(mode) => {
                        self.mode = mode;
                        self.status.update(Command::UpdateMode(mode), &mut queue);
                    }
                    command => self.view.update(command, &mut queue),
                }
                commands = queue.into();

                if commands.len() == 0 {
                    break;
                }
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
    let (tx, rx) = mpsc::channel::<Command>();
    let app = App::new()?;

    thread::spawn(move || keybindings::handle_keyboard_input(tx));
    app.run(terminal, rx)?;
    ratatui::restore();

    Ok(())
}
