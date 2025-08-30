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
 * Contains the implementation of the `PushQueue` data structure. This data
 * structure is a queue that can only be pushed into using `push_back`. It
 * supports `From<VecDeque>` and `Into<VecDeque>`. Its primary purpose is to
 * allow others to push into the queue without fear of the items already in the
 * queue being altered.
 */

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                  IMPORTS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

use std::collections::VecDeque;

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                  STRUCTS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

/// A queue that only allows new items to be pushed into the back of it.
pub struct PushQueue<T>(VecDeque<T>);

impl<T> PushQueue<T> {
    /// Appends the provided element into the back of the queue.
    pub fn push_back(&mut self, value: T) {
        self.0.push_back(value);
    }
}

impl<T> From<VecDeque<T>> for PushQueue<T> {
    fn from(value: VecDeque<T>) -> Self {
        Self(value)
    }
}

impl<T> Into<VecDeque<T>> for PushQueue<T> {
    fn into(self) -> VecDeque<T> {
        self.0
    }
}
