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
 * The statusline. This displays the current mode of the application, the title
 * of the loaded galaxy, etc. This also provides the area that commands are
 * typed.
 */

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                  IMPORTS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Widget,
};

use crate::app::tui::Mode;

use super::{view::View, App};

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                  STRUCTS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

/// Handles all functionality related to the statusline at the bottom of the
/// application screen.
#[derive(Debug, Default)]
pub struct StatusLine;

impl View for StatusLine {
    fn render(&self, app: &App, area: Rect, buf: &mut Buffer) {
        let mode = match app.mode {
            Mode::Normal => "[NORMAL]  ",
            Mode::Insert => "[INSERT]  ",
            Mode::Command => "[COMMAND]  ",
        };
        let mode = match app.mode {
            Mode::Normal => Span::from(mode).style(Style::default().fg(Color::Green)),
            Mode::Insert => Span::from(mode).style(Style::default().fg(Color::Magenta)),
            Mode::Command => Span::from(mode).style(Style::default().fg(Color::Blue)),
        };
        let title = Span::from(app.galaxy.get_title_copy());
        let line = Line::from(vec![mode, title]).style(Style::default().bg(Color::Black));
        line.render(area, buf);
    }
}
