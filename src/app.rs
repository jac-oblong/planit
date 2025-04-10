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

use std::io::Result;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{text::Text, DefaultTerminal, Frame};

use crate::core::planet::Planet;

/// The planit application
///
/// This struct encapsulates everything in the planit app. It should only be
/// necessary to create and run
pub struct App {
    /// Vector of planets that the App currently knows about
    planets: Vec<Planet>,

    /// Whether or not the application should close
    should_quit: bool,
}

impl App {
    /// Creates a new App
    pub fn new() -> Self {
        App {
            planets: Vec::new(),
            should_quit: false,
        }
    }

    /// Runs the application
    ///
    /// This function contains the super loop that handles drawing to the screen
    /// and handling all events. It will not exit until the application should
    /// close
    ///
    /// # Arguments
    /// - `terminal`: the terminal to use for drawing
    ///
    /// # Returns
    /// - Any errors encountered
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.should_quit {
            let draw_callable = |frame: &mut Frame| self.draw(frame);
            terminal.draw(&draw_callable)?;
            self.handle_events()?;
        }
        Ok(())
    }

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
    fn draw(&self, frame: &mut Frame) {
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
    /// - Any errors encountered
    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Char('q') => {
                    self.should_quit = true;
                    Ok(())
                }
                _ => Ok(()),
            },
            _ => Ok(()),
        }
    }
}
