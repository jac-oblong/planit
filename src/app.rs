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

/*!
 * The interface for interacting with the TUI application.
 */

use std::io::Result;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Text,
    widgets::{Paragraph, Widget},
    DefaultTerminal, Frame,
};

use crate::core::Planet;

/// This struct encapsulates everything in the planit app. It should only be
/// necessary to create the struct and then run it
///
/// # Examples
///
/// ```
/// let mut terminal = ratatui::init();
/// let mut app = App::new();
/// let result = app.run(&mut terminal);
/// ```
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

    /// This function contains the super loop that handles drawing to the screen
    /// and handling all events. It will not exit until the application should
    /// close
    ///
    /// # Arguments
    /// - `terminal`: the terminal to use for drawing
    ///
    /// # Errors
    /// Will produce errors when there is an error drawing to the screen or
    /// handling events. This can happend when:
    /// * `crossterm::event::read` produces an error
    /// * `ratatui::DefaultTerminal::draw` produces an error
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    /// This function takes the frame and renders all widgets that should be shown
    /// based on the app's state
    ///
    /// # Arguments
    /// - `frame`: The ratatui::Frame object used to render widgets
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    /// This function handles all events from the user, etc.
    ///
    /// # Errors
    /// Will produce errors when `crossterm::event::read` errors
    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Char('q') => {
                    self.should_quit = true;
                    Ok(())
                }

                KeyCode::Char('a') => {
                    self.planets
                        .push(Planet::new("A".to_string(), "B".to_string()));
                    Ok(())
                }

                _ => Ok(()),
            },
            _ => Ok(()),
        }
    }
}

impl Widget for &App {
    /// Implements rendering of the app
    ///
    /// # Arguments
    /// - `area`: The rectangle within which to render the app
    /// - `buf`: The buffer to render the app into
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = Text::raw("Hello world!");
        Paragraph::new(text).centered().render(area, buf);
    }
}
