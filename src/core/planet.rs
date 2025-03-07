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

use std::time::SystemTime;

/// An enum representing the status of a Planet
///
/// Planet's status should follow the pattern `Todo -> Next -> Start -> Done`.
/// There are also `Hold` and `Cancel` states if they are needed.
///
/// Only `Done` and `Cancel` are final states.
pub enum PlanetStatus {
    /// Planet's that are `Todo` are in the "backlog"
    Todo,
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

/// A struct representing the basic unit of work
///
/// Planets are the basic unit of work. Everything else is made of them. Planets
/// have a unique identifier that is guaranteed to be unique, so no two planets
/// will ever be the same. They also have a name (or title), description,
/// creation date, and a history of all changes.
///
/// Optionally, they can have start and end dates, repetition, and user defined
/// tags.
pub struct Planet {
    /// The unique & constant identifier for each planet
    id: u64,
    /// The `name` (or title) for the planet
    name: String,
    /// A more detailed description of the planet
    description: String,
    /// The date the planet was created
    created: SystemTime,
    /// The current state of the planet
    status: PlanetStatus,
    /// The date that work for the planet is expected to start
    start: Option<SystemTime>,
    /// The date that work for the planet is expected to be complete
    end: Option<SystemTime>,
    /// How often the planet should repeat
    repeat: Option<String>, // TODO: This should be a more specific value
    /// Any user defined tags the planet has
    tags: Vec<String>,
}
