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

/// A struct representing the a task
///
/// Tasks are the basic unit of work. Everything else is made of tasks
pub struct Task {
    /// The unique identifier for each task. This `id` is guaranteed to be constant
    id: u64,
    /// The `name` (or title) for the task
    name: String,
    /// A more detailed description of the task
    description: String,
    /// The date the task was created
    created: Option<SystemTime>,
    /// The date that work for the task is expected to start
    start: Option<SystemTime>,
    /// The date that work for the task is expected to be complete
    end: Option<SystemTime>,
    /// How often the task should repeat
    repeat: Option<String>, // TODO: This should be a more specific value
    /// Any user defined tags the task has
    tags: Vec<String>,
}
