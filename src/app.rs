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

use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    widgets::{Block, BorderType, List, ListItem, Padding},
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
    pub fn run(&mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
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
        let planets: Vec<ListItem> = self
            .planets
            .iter()
            .map(|planet| ListItem::from(planet))
            .collect();

        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .padding(Padding::proportional(1))
            .title("Planets");

        let list = List::new(planets).block(block);
        frame.render_widget(list, frame.area());
    }

    /// This function handles all events from the user, etc.
    ///
    /// # Errors
    /// Will produce errors when `crossterm::event::read` errors
    fn handle_events(&mut self) -> io::Result<()> {
        if let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    self.should_quit = true;
                }

                KeyCode::Char('i') => {
                    self.planets
                        .push(Planet::new("A".to_string(), "B".to_string()));
                }

                _ => {}
            }
        }
        Ok(())
    }
}
