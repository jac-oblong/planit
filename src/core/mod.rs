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
 * The core structures representing the work to be done for the project.
 *
 * All celestial bodies have shared core features: namely a title and a
 * description. This will be used for display purposes. Additionally, with the
 * exception of the `Galaxy`, everything has a unique ID, a `Status`, a
 * parent, and a history. The parent can optionally be `None` if the celestial
 * body is in the root of the `Galaxy`. The history will keep track of all
 * changes to the status of the celestial body.
 */

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                  MODULES                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

pub mod comet;
pub mod galaxy;
pub mod planet;
pub mod star;

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                  IMPORTS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

use std::fmt::Display;

use chrono::{DateTime, Local};
use colored::Colorize;
use serde::{Deserialize, Serialize};

use crate::app::cli;
pub use crate::core::comet::Comet;
pub use crate::core::galaxy::{CelestialBodyIndex, Galaxy};
pub use crate::core::planet::Planet;
pub use crate::core::star::Star;
use crate::util;

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                   TYPES                                    //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

type ID = u64;

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                   TRAITS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

/// Trait that all celestial bodies must implement
pub trait CelestialBody<'a>:
    std::fmt::Debug + Deserialize<'a> + Serialize + PartialEq + Eq + util::tree::PrintTreeNode<Galaxy>
{
    /// Constructor that uses `id` for the new celestial body
    fn new(id: ID) -> Self;

    /// Setter for celestial body's parent
    fn parent(&mut self, parent: ID) -> &mut Self;
    /// Setter for celestial body's title
    fn title(&mut self, title: String) -> &mut Self;
    /// Setter for celestial body's description
    fn description(&mut self, description: String) -> &mut Self;
    /// Setter for celestial body's status. `commet` should be an explanation of
    /// why the status has changed
    fn status(&mut self, status: Status, comment: String) -> &mut Self;
}

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                   ENUMS                                    //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

/// Represents the different types of celestial bodies
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, cli::ValueEnum)]
pub enum CelestialBodyKind {
    /// An interrupting task / bug
    Comet,
    /// A basic unit of work
    Planet,
    /// A collection of other celestial bodies
    Star,
}

/// Represents the status of the `Planet` / `Comet` / `Star`
///
/// The status should follow the pattern `Todo` -> `Next` -> `Start` -> `Done`.
/// There are also `Block`, `Hold`, and `Cancel` states if they are needed.
///
/// Only `Done` and `Cancel` are considered to be final states. Parents cannot
/// move to a final state unless all children are in a final state.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Status {
    /// No work has been started, still in the "backlog"
    Todo,
    /// Cannot be started due to a pre-requisite or some other reason
    Block,
    /// Staged to be started after current tasks are completed
    Next,
    /// Currently being worked on
    Start,
    /// Paused for some reason
    Hold,
    /// Completed
    Done,
    /// Canceled for some reason
    Cancel,
}

impl Default for Status {
    fn default() -> Self {
        Self::Todo
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Todo => write!(f, "Todo"),
            Self::Block => write!(f, "Block"),
            Self::Next => write!(f, "Next"),
            Self::Start => write!(f, "Start"),
            Self::Hold => write!(f, "Hold"),
            Self::Done => write!(f, "Done"),
            Self::Cancel => write!(f, "Cancel"),
        }
    }
}

impl From<Status> for colored::ColoredString {
    fn from(value: Status) -> Self {
        match value {
            Status::Todo => "Todo ".bright_yellow(),
            Status::Next => "Next ".purple(),
            Status::Start => "Start ".green(),
            Status::Hold => "Hold ".bright_black(),
            Status::Block => "Block ".red(),
            Status::Done => "Done  ".bright_black(),
            Status::Cancel => "Cancel".bright_black(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                  STRUCTS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

/// A single change to the celestial body's status that occurred in history
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct StatusHistory {
    old: Status,
    new: Status,
    comment: String,
    time: DateTime<Local>,
}
