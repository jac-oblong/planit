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
 * Module containing the Comet implementation.
 */

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                  IMPORTS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

use chrono::Local;
use log::info;
use serde::{Deserialize, Serialize};

use super::{Status, StatusHistory, ID};

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                  STRUCTS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

/// Comets are interrupting tasks / bugs. They should be small and compact. They
/// only contain the core features required by all celestial bodies because they
/// are meant to quickly go from `Todo` to `Done`.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Comet {
    pub(super) id: ID,
    pub(super) parent: Option<ID>,
    pub(super) title: String,
    pub(super) description: String,
    pub(super) status: Status,
    pub(super) history: Vec<StatusHistory>,
}

impl Comet {
    /// Sets the `parent` field and returns `self`
    pub fn parent(&mut self, parent: ID) -> &mut Self {
        self.parent = Some(parent);
        self
    }

    /// Sets the `title` field and returns `self`
    pub fn title(&mut self, title: String) -> &mut Self {
        self.title = title;
        self
    }

    /// Sets the `description` field and returns `self`
    pub fn description(&mut self, description: String) -> &mut Self {
        self.description = description;
        self
    }

    /// Sets the `status` field. `comment` should be an explanation of why the
    /// status has changed
    pub fn status(&mut self, status: Status, comment: String) {
        self.history.push(StatusHistory {
            old: self.status,
            new: status,
            comment,
            time: Local::now(),
        });
        info!(
            "Comet ({}) changed status from {} to {}",
            self.id, self.status, status
        );
        self.status = status;
    }
}

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                   TESTS                                    //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod test {
    use chrono::TimeDelta;

    use super::*;

    #[test]
    fn changing_status_adds_to_history() {
        let mut comet = Comet::default();
        let t1 = Local::now();
        comet.status(Status::Start, "1".to_string());
        let t2 = Local::now();
        comet.status(Status::Done, "2".to_string());

        assert_eq!(comet.history.len(), 2);

        assert_eq!(comet.history[0].comment, "1");
        assert_eq!(comet.history[0].old, Status::Todo);
        assert_eq!(comet.history[0].new, Status::Start);
        assert!(comet.history[0].time - t1 < TimeDelta::milliseconds(1));

        assert_eq!(comet.history[1].comment, "2");
        assert_eq!(comet.history[1].old, Status::Start);
        assert_eq!(comet.history[1].new, Status::Done);
        assert!(comet.history[1].time - t2 < TimeDelta::milliseconds(1));
    }
}
