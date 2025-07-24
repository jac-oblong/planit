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
use serde::{Deserialize, Serialize};

pub use crate::core::comet::Comet;
pub use crate::core::galaxy::{CelestialBodyIndex, Galaxy};
pub use crate::core::planet::Planet;
pub use crate::core::star::Star;

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                   TYPES                                    //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

type ID = u64;

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                   ENUMS                                    //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

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

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                  STRUCTS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

/// A single change to the celestial body's status that occurred in history
#[derive(Debug, Deserialize, Serialize)]
pub struct StatusHistory {
    old: Status,
    new: Status,
    comment: String,
    time: DateTime<Local>,
}
