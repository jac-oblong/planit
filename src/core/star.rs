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
 * Module containing the Star implementation.
 */

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                  IMPORTS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

use chrono::Utc;
use colored::Colorize;
use log::info;
use serde::{Deserialize, Serialize};

use crate::util;

use super::{CelestialBody, Galaxy, Status, StatusHistory, ID};

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                  STRUCTS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

/// Stars are collections of other celestial bodies. They can contain Planets,
/// Comets, and even other Stars. They are meant to be used to separate elements
/// into organized groups.
#[derive(Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct Star {
    pub(super) id: ID,
    pub(super) parent: Option<ID>,
    pub(super) title: String,
    pub(super) description: String,
    pub(super) status: Status,
    pub(super) history: Vec<StatusHistory>,

    /// Contains the ids of all the celestial bodies that are directly owned by
    /// this star
    pub(super) children: Vec<ID>,
}

impl CelestialBody<'_> for Star {
    fn new(id: ID) -> Self {
        Self {
            id,
            ..Self::default()
        }
    }

    fn parent(&mut self, parent: ID) -> &mut Self {
        self.parent = Some(parent);
        self
    }

    fn title(&mut self, title: String) -> &mut Self {
        self.title = title;
        self
    }

    fn description(&mut self, description: String) -> &mut Self {
        self.description = description;
        self
    }

    fn status(&mut self, status: Status, comment: String) -> &mut Self {
        self.history.push(StatusHistory {
            old: self.status,
            new: status,
            comment,
            time: Utc::now(),
        });
        info!(
            "Star ({}) changed status from {} to {}",
            self.id, self.status, status
        );
        self.status = status;
        self
    }
}

impl util::tree::PrintTreeNode<Galaxy> for Star {
    fn icon(&self) -> colored::ColoredString {
        "".yellow()
    }

    fn label(&self) -> colored::ColoredString {
        "[STAR]  ".yellow() // Added spaces line it up with planet
    }

    fn status(&self) -> colored::ColoredString {
        self.status.into()
    }

    fn title(&self) -> colored::ColoredString {
        colored::ColoredString::from(self.title.clone())
    }

    fn description(&self) -> colored::ColoredString {
        self.description.bright_black()
    }

    fn children<'a>(
        &self,
        root: &'a Galaxy,
    ) -> Vec<Box<&'a dyn util::tree::PrintTreeNode<Galaxy>>> {
        let children = self
            .children
            .iter()
            .map(|child| match root.index(*child) {
                Some(index) => match index.kind {
                    super::CelestialBodyKind::Comet => Box::new(
                        &root.comets[index.index] as &dyn util::tree::PrintTreeNode<Galaxy>,
                    ),
                    super::CelestialBodyKind::Planet => Box::new(
                        &root.planets[index.index] as &dyn util::tree::PrintTreeNode<Galaxy>,
                    ),
                    super::CelestialBodyKind::Star => {
                        Box::new(&root.stars[index.index] as &dyn util::tree::PrintTreeNode<Galaxy>)
                    }
                },
                None => panic!(
                    "Child with id {} of star {} did not match any index",
                    child, self.id
                ),
            })
            .collect();
        children
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
        let mut star = Star::default();
        let t1 = Utc::now();
        star.status(Status::Start, "1".to_string());
        let t2 = Utc::now();
        star.status(Status::Done, "2".to_string());

        assert_eq!(star.history.len(), 2);

        assert_eq!(star.history[0].comment, "1");
        assert_eq!(star.history[0].old, Status::Todo);
        assert_eq!(star.history[0].new, Status::Start);
        assert!(star.history[0].time - t1 < TimeDelta::milliseconds(1));

        assert_eq!(star.history[1].comment, "2");
        assert_eq!(star.history[1].old, Status::Start);
        assert_eq!(star.history[1].new, Status::Done);
        assert!(star.history[1].time - t2 < TimeDelta::milliseconds(1));
    }
}
