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
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Clear, List, ListItem, Padding, Paragraph},
    DefaultTerminal, Frame,
};

use crate::core::Planet;

/// Determines the current mode of the application.
/// * `Normal` - This is the primary mode. It includes a list view of the
///   `Planets`, and nothing else.
/// * `Insert` - This is a secondary mode. It allows for adding a new `Planet`
#[derive(Debug, PartialEq, Eq)]
enum AppMode {
    Normal,
    Insert,
}

/// Determines what is currently being inserted
/// * `Name` - The planet name is currently being typed.
/// * `Description` - The planet description is currently being typed.
#[derive(Debug, PartialEq, Eq)]
enum AppInsert {
    Name,
    Description,
}

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
    /// Current mode of the application
    mode: AppMode,

    /// What is currently being typed
    insert: AppInsert,
    /// Planet name that is currently being typed
    planet_name: String,
    /// Planet description that is currently being typed
    planet_desc: String,
}

impl App {
    /// Creates a new App
    pub fn new() -> Self {
        App {
            planets: Vec::new(),
            should_quit: false,
            mode: AppMode::Normal,
            insert: AppInsert::Name,
            planet_name: String::new(),
            planet_desc: String::new(),
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

        if self.mode == AppMode::Insert {
            let popup_block = Block::bordered()
                .title(" Enter New Planet ")
                .border_type(BorderType::Rounded);

            let popup_area = centered_rect(75, 25, frame.area());
            frame.render_widget(Clear, popup_area);
            frame.render_widget(popup_block, popup_area);

            let popup_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
                .split(popup_area);

            let mut name_block = Block::bordered()
                .title(" Name ")
                .border_type(BorderType::Rounded);
            let mut desc_block = Block::bordered()
                .title(" Description ")
                .border_type(BorderType::Rounded);

            let active_style = Style::default().fg(Color::Magenta);

            match self.insert {
                AppInsert::Name => {
                    name_block = name_block
                        .border_style(active_style)
                        .title_style(active_style)
                }
                AppInsert::Description => {
                    desc_block = desc_block
                        .border_style(active_style)
                        .title_style(active_style)
                }
            };

            let name_text = Paragraph::new(self.planet_name.clone()).block(name_block);
            frame.render_widget(name_text, popup_chunks[0]);

            let desc_text = Paragraph::new(self.planet_desc.clone()).block(desc_block);
            frame.render_widget(desc_text, popup_chunks[1]);
        }
    }

    /// This function handles all events from the user, etc.
    ///
    /// # Errors
    /// Will produce errors when `crossterm::event::read` errors
    fn handle_events(&mut self) -> io::Result<()> {
        if let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            match self.mode {
                AppMode::Normal => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
                    KeyCode::Char('i') => self.mode = AppMode::Insert,
                    _ => {}
                },
                AppMode::Insert => match key.code {
                    KeyCode::Esc => {
                        self.mode = AppMode::Normal;
                        self.insert = AppInsert::Name;
                        self.planet_name = String::new();
                        self.planet_desc = String::new();
                    }
                    KeyCode::Enter => {
                        self.mode = AppMode::Normal;
                        self.insert = AppInsert::Name;
                        self.planets.push(Planet::new(
                            self.planet_name.clone(),
                            self.planet_desc.clone(),
                        ));
                        self.planet_name = String::new();
                        self.planet_desc = String::new();
                    }
                    KeyCode::Tab => match self.insert {
                        AppInsert::Name => self.insert = AppInsert::Description,
                        AppInsert::Description => self.insert = AppInsert::Name,
                    },
                    KeyCode::Char(c) => match self.insert {
                        AppInsert::Name => self.planet_name.push(c),
                        AppInsert::Description => self.planet_desc.push(c),
                    },
                    _ => {}
                },
            }
        }
        Ok(())
    }
}

/// Helper function to create a centered rectangle using a certain percentage
/// of the rectange `r`.
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
