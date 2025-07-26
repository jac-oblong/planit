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

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                  IMPORTS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

use std::collections::{BTreeMap, HashMap};

use chrono::Local;
use log::info;
use serde::{Deserialize, Serialize, Serializer};

use super::{CelestialBody, Status, StatusHistory, ID};

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                  STRUCTS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

/// Planets are the basic unit of work.
///
/// In addition to the core features that all celestial bodies have, Planets
/// have custom tags and custom fields. These can all be safely ignored.
#[derive(Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct Planet {
    pub(super) id: ID,
    pub(super) parent: Option<ID>,
    pub(super) title: String,
    pub(super) description: String,
    pub(super) status: Status,
    pub(super) history: Vec<StatusHistory>,

    /// User defined tags. These can be used for searching, filtering, labeling,
    /// etc. They will not affect the Planet otherwise.
    pub(super) tags: Vec<String>,
    /// User defined fields. These can be used for searching, filtering,
    /// labeling, etc. They consist of a key and an associated value. They will
    /// not affect the Planet otherwise.
    #[serde(serialize_with = "ordered_map")]
    pub(super) fields: HashMap<String, String>,
}

/// Helper function to ensure that HashMaps are serialized in order
fn ordered_map<S>(value: &HashMap<String, String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}

impl CelestialBody<'_> for Planet {
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
            time: Local::now(),
        });
        info!(
            "Planet ({}) changed status from {} to {}",
            self.id, self.status, status
        );
        self.status = status;
        self
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
        let mut planet = Planet::default();
        let t1 = Local::now();
        planet.status(Status::Start, "1".to_string());
        let t2 = Local::now();
        planet.status(Status::Done, "2".to_string());

        assert_eq!(planet.history.len(), 2);

        assert_eq!(planet.history[0].comment, "1");
        assert_eq!(planet.history[0].old, Status::Todo);
        assert_eq!(planet.history[0].new, Status::Start);
        assert!(planet.history[0].time - t1 < TimeDelta::milliseconds(1));

        assert_eq!(planet.history[1].comment, "2");
        assert_eq!(planet.history[1].old, Status::Start);
        assert_eq!(planet.history[1].new, Status::Done);
        assert!(planet.history[1].time - t2 < TimeDelta::milliseconds(1));
    }
}
