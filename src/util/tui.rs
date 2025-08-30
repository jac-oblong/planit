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
 * Helper utilities related to TUI operations
 */

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                  IMPORTS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

use ratatui::layout::{Constraint, Direction, Flex, Layout, Rect};

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                 FUNCTIONS                                  //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

/// Helper function to create a centered rectangle using a certain percentage
/// of the rectange `area`.
///
/// # Arguments
/// - `area`: Rectangle within which the new rectangle should be centered.
/// - `percent_x`: Percentage in range (0, 100]. The center `percent_x` percent
///   of the horizontal part of `area` will be used for the new rectangle.
/// - `percent_y`: Percentage in range (0, 100]. The center `percent_y` percent
///   of the vertical part of `area` will be used for the new rectangle.
///
/// # Returns
/// A new rectangle that is centered within `area`
pub fn center_with_percent(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    debug_assert!(percent_x < 100);
    debug_assert!(percent_y < 100);

    // Cut the given rectangle into three vertical pieces
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(layout[1])[1] // Return the middle chunk
}

/// Helper function to create a centered rectangle using constraints on the
/// rectangle `area`.
///
/// # Arguments
/// - `area`: Rectangle within which the new rectangle should be centered.
/// - `horizontal`: The horizontal constraint that should be used.
/// - `percent_y`: The vertical constraint that should be used.
///
/// # Returns
/// A new rectangle that is centered within `area`
pub fn center_with_constraints(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
