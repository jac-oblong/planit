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
 * Contains the implementation for the opening view. This is the view shown when
 * the application is originally opened. It shows some information about the
 * application and some initial guidance on how to use the app.
 */

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                  IMPORTS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Widget},
};

use crate::util::{queue::PushQueue, tui::center_with_constraints};

use super::{super::Command, View};

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                   CONSTS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

/// The padding between the text inside the box and the box itself
const VERTICAL_BOX_PADDING: u16 = 2;
const HORIZONTAL_BOX_PADDING: u16 = 2 * VERTICAL_BOX_PADDING;
/// The size of the box (1 for left / top plus 1 for right / bottom)
const BOX_SIZE: u16 = 2;

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                  STRUCTS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

/// The opening view. This is the view that is shown when the app is first
/// opened. This includes things like version, helpful keybinds, etc.
#[derive(Debug)]
pub struct OpeningView;

impl View for OpeningView {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let lines = vec![
            Line::from(env!("CARGO_PKG_NAME")).style(Style::default().fg(Color::Magenta)),
            Line::from(env!("CARGO_PKG_DESCRIPTION")),
            Line::from(""),
            Line::from(format!("version: {}", env!("CARGO_PKG_VERSION"))),
            Line::from(format!("repo: {}", env!("CARGO_PKG_REPOSITORY"))),
        ];

        let height = lines.len();
        let width = lines
            .iter()
            .map(|x| match x.spans.get(0) {
                Some(span) => span.content.len(),
                None => 0,
            })
            .max()
            .unwrap();
        let area = center_with_constraints(
            area,
            Constraint::Length((width as u16) + 2 * HORIZONTAL_BOX_PADDING + BOX_SIZE),
            Constraint::Length((height as u16) + 2 * VERTICAL_BOX_PADDING + BOX_SIZE),
        );

        let paragraph = Paragraph::new(lines).alignment(Alignment::Center).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::symmetric(
                    HORIZONTAL_BOX_PADDING,
                    VERTICAL_BOX_PADDING,
                )),
        );
        paragraph.render(area, buf);
    }

    fn update(&mut self, _: Command, _: &mut PushQueue<Command>) {}
}
