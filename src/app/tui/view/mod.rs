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
 * Contains implementations for app views. These views directly control what the
 * user sees.
 */

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                  IMPORTS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

use std::fmt::Debug;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Clear, Paragraph, Widget},
};

use super::App;

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                   TRAITS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

/// The interface that all views must implement.
pub trait View: Debug {
    /// Renders self into the given buffer. A reference to the app itself is
    /// given if the view relies on the state of the application.
    ///
    /// # Arguments
    /// - `app`: The current state of the application.
    /// - `area`: The area within the buffer that is owned by this view.
    /// - `buf`: The buffer to render into.
    fn render(&self, app: &App, area: Rect, buf: &mut Buffer);
}

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                  STRUCTS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

/// The default view. This is the view that is shown when no view has been
/// selected. It will display nothing (just a blank screen).
#[derive(Debug)]
pub struct DefaultView;

impl View for DefaultView {
    fn render(&self, _: &App, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);
    }
}

/// The opening view. This is the view that is shown when the app is first
/// opened. This includes things like version, helpful keybinds, etc.
#[derive(Debug)]
pub struct OpeningView;

impl View for OpeningView {
    fn render(&self, _: &App, area: Rect, buf: &mut Buffer) {
        let lines = vec![
            Line::from(env!("CARGO_PKG_NAME"))
                .centered()
                .style(Style::default().fg(Color::Magenta)),
            Line::from(env!("CARGO_PKG_DESCRIPTION")).centered(),
            Line::from("").centered(),
            Line::from(format!("version: {}", env!("CARGO_PKG_VERSION"))).centered(),
            Line::from(format!("repo: {}", env!("CARGO_PKG_REPOSITORY"))).centered(),
        ];
        let paragraph = Paragraph::new(lines);
        paragraph.render(area, buf);
    }
}

/// A view that is split into two halves. The two halves are assigned to child
/// views. The split can be either horizontal or vertical.
#[derive(Debug)]
struct SplitView {
    direction: Direction,
    children: [Box<dyn View>; 2],
}

impl View for SplitView {
    fn render(&self, app: &App, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(self.direction)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);
        self.children[0].render(app, layout[0], buf);
        self.children[1].render(app, layout[1], buf);
    }
}
