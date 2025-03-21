////////////////////////////////////////////////////////////////////////////
//                                                                        //
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
//                                                                        //
////////////////////////////////////////////////////////////////////////////

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{text::Text, Frame};

/// Handles Drawing of TUI
///
/// This function takes the frame and renders all widgets that should be shown
/// based on the app's state
///
/// # Arguments
/// - `frame`: The ratatui::Frame object used to render widgets
///
/// # Returns
/// - None
fn draw(frame: &mut Frame) {
    let text = Text::raw("Hello world!");
    frame.render_widget(text, frame.area());
}

/// Handles Events
///
/// This function handles all events from the user, etc.
///
/// # Arguments
/// - None
///
/// # Returns
/// - Result containing `true` if app should quit, `false` otherwise
fn handle_events() -> std::io::Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') => Ok(true),
            _ => Ok(false),
        },
        _ => Ok(false),
    }
}

/// Runs the terminal application
///
/// This function contains the super loop that handles rendering and events. It
/// will not exit until the application should close
///
/// # Arguments
/// - `terminal`: the terminal to use for drawing
///
/// # Returns
/// - Any errors encountered
fn run(terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(draw)?;
        if handle_events()? {
            break Ok(());
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();
    result
}
