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
 * Module containing the Planet implementation.
 */

use std::{fmt, time::SystemTime};

use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::ListItem,
};
use serde::{Deserialize, Serialize};

/// An enum representing the status of a Planet
///
/// Planet's status should follow the pattern `Todo -> Next -> Start ->
/// `Done`. There are also `Block`, `Hold` and `Cancel` states if they are
/// needed.
///
/// Only `Done` and `Cancel` are final states.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum PlanetStatus {
    /// Planet's that are `Todo` are in the "backlog"
    Todo,
    // Planet's that are `Block` cannot be started due to a pre-requesite
    Block,
    /// Planet's that are `Next` are staged to be started
    Next,
    /// Planet's that are `Start` are currently being worked on
    Start,
    /// Planet's that are `Hold` are on hold for some reason
    Hold,
    /// Planet's that are `Done` are completed
    Done,
    /// Planet's that are `Cancel` have been canceled for some reason
    Cancel,
}

impl fmt::Display for PlanetStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

/// Planets are the basic unit of work. Everything else is made of them. Planets
/// have a unique identifier, so no two planets will ever be the same. They also
/// have a name (or title), description, creation date, and a history of all
/// changes.
///
/// These features are all built-in and cannot be disabled. Other features (like
/// dates, pre/co-requesites, etc.) are enabled, but can safely be ignored.
#[derive(Deserialize, Serialize, Clone)]
pub struct Planet {
    /// The `name` (or title) for the planet
    name: String,
    /// A more detailed description of the planet
    description: String,
    /// The date the planet was created
    created: SystemTime,
    /// The current state of the planet
    status: PlanetStatus,
}

impl Planet {
    /// This function creates a new Planet from all the parameters given. This
    /// function should only be used to create a brand new Planet. That is, it
    /// should not be used when initializing a Planet read from a file.
    ///
    /// # Arguments
    /// - `name`: The name field of the new Planet
    /// - `description`: The description field of the new Planet
    ///
    /// # Returns
    /// A new Planet with all fields initialized appropriately
    pub fn new(name: String, description: String) -> Planet {
        Planet {
            name,
            description,
            created: SystemTime::now(),
            status: PlanetStatus::Todo,
        }
    }
}

/// Used when rendering the Planet as a list along with all the other planets
impl From<&Planet> for ListItem<'_> {
    fn from(value: &Planet) -> Self {
        let line = Line::from(vec![
            Span::styled(
                value.status.to_string(),
                Style::default()
                    .fg(Color::LightYellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" "),
            Span::styled(value.name.clone(), Style::default().fg(Color::Magenta)),
            Span::raw(" "),
            Span::raw(value.description.clone()),
        ]);
        ListItem::new(line)
    }
}
