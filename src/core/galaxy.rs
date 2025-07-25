////////////////////////////////////////////////////////////////////////////
// The MIT License (MIT)                                                  //
//                                                                        //
// Copyright (c) 2025 Jacob Long                                          //
//                                                                        // Permission is hereby granted, free of charge, to any person obtaining  // a copy of this software and associated documentation files (the        //
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
 * Module containing the Galaxy implementation as well as all logic for
 * interacting with databases.
 */

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                  IMPORTS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

use std::{
    collections::HashMap,
    env, fmt, fs, io,
    path::{Path, PathBuf},
};

use log::info;
use serde::{Deserialize, Serialize};

use super::{CelestialBodyKind, Comet, Planet, Star, ID};

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                   TYPES                                    //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

type Result<T> = std::result::Result<T, DatabaseError>;

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                   ENUMS                                    //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

/// Possible errors when loading / saving a database
#[derive(Debug)]
pub enum DatabaseError {
    /// The specified database could not be found
    DatabaseNotFound(String),
    /// The specified database already exists
    DatabaseAlreadyExists(String),
    /// An error occurred while performing an filesystem operation
    FileSystemError(io::Error),
    /// An error occurrd while parsing the database
    ParsingError(serde_json::Error),
}

impl std::error::Error for DatabaseError {}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatabaseError::DatabaseNotFound(name) => {
                write!(f, "Database not found: {name}")
            }
            DatabaseError::DatabaseAlreadyExists(name) => {
                write!(f, "Database already exists: {name}")
            }
            DatabaseError::FileSystemError(io_error) => {
                write!(f, "Database file system error: {io_error}")
            }
            DatabaseError::ParsingError(json_error) => {
                write!(f, "Database parsing error: {json_error}")
            }
        }
    }
}

impl From<io::Error> for DatabaseError {
    fn from(value: io::Error) -> Self {
        Self::FileSystemError(value)
    }
}

impl From<serde_json::Error> for DatabaseError {
    fn from(value: serde_json::Error) -> Self {
        Self::ParsingError(value)
    }
}

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                  STRUCTS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

/// An struct representing the type of the celestial body and the index in the
/// corresponding vector for said celestial body
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CelestialBodyIndex {
    /// The kind of celestial body it is
    kind: CelestialBodyKind,
    /// The index into the associated vector
    index: usize,
}

impl CelestialBodyIndex {
    pub fn new(kind: CelestialBodyKind, index: usize) -> Self {
        Self { kind, index }
    }
}

/// The representation of the database. This is an internal struct that should
/// only be used by the `Galaxy` when loading / saving.
/// NOTE: If this struct (or any structs it contains) is changed in any way,
/// `SCHEMA_VERSION` needs to be incremented
#[derive(Debug, Deserialize, Serialize)]
struct Database {
    /// The current schema version. This field should ALWAYS exist.
    #[serde(deserialize_with = "ensure_database_version")]
    version: u64,
    /// Comment explaining what the database file is for
    comment: String,

    title: String,
    description: String,
    next_id: ID,

    comets: Vec<Comet>,
    planets: Vec<Planet>,
    stars: Vec<Star>,
}

impl Database {
    const SCHEMA_VERSION: u64 = 2;
    const DEFAULT_FILENAME: &str = ".planit.json";

    /// Finds the location for the database file
    ///
    /// # Errors
    /// Errors will occur in the following situations:
    /// - The specified database cannot be found. This includes if the default
    ///   database cannot be found.
    pub fn location() -> Result<PathBuf> {
        let mut path: PathBuf = env::current_dir()?;
        let file = Path::new(Database::DEFAULT_FILENAME);

        loop {
            path.push(file);
            if path.exists() {
                break Ok(path);
            }
            // Remove the file and go up one directory
            if !(path.pop() && path.pop()) {
                break Err(DatabaseError::DatabaseNotFound(
                    Database::DEFAULT_FILENAME.into(),
                ));
            }
        }
    }

    /// Sets the `title` field and returns `self`
    pub fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    /// Sets the `description` field and returns `self`
    pub fn description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    /// Sets the `next_id` field and returns `self`
    pub fn next_id(mut self, next_id: ID) -> Self {
        self.next_id = next_id;
        self
    }

    /// Sets the `comets` field and returns `self`
    pub fn comets(mut self, comets: Vec<Comet>) -> Self {
        self.comets = comets;
        self
    }

    /// Sets the `planets` field and returns `self`
    pub fn planets(mut self, planets: Vec<Planet>) -> Self {
        self.planets = planets;
        self
    }

    /// Sets the `planets` field and returns `self`
    pub fn stars(mut self, stars: Vec<Star>) -> Self {
        self.stars = stars;
        self
    }
}

impl Default for Database {
    fn default() -> Self {
        Database {
            version: Database::SCHEMA_VERSION,
            comment: format!(
                "Database for Planit project. See {}",
                env!("CARGO_PKG_REPOSITORY")
            ),
            title: String::default(),
            description: String::default(),
            next_id: ID::default(),
            comets: Vec::default(),
            planets: Vec::default(),
            stars: Vec::default(),
        }
    }
}

fn ensure_database_version<'de, D: serde::Deserializer<'de>>(
    d: D,
) -> std::result::Result<u64, D::Error> {
    let version = u64::deserialize(d)?;
    match version {
        Database::SCHEMA_VERSION => Ok(version),
        _ => Err(serde::de::Error::custom(format!(
            "Version mismatch for database. Expected {} got {}",
            Database::SCHEMA_VERSION,
            version
        ))),
    }
}

/// The Galaxy is the top-level structure. It contains all celestial bodies
/// within the project.
#[derive(Debug, Default)]
pub struct Galaxy {
    pub title: String,
    pub description: String,

    /// The ID of the next created celestial body
    pub(super) next_id: ID,

    /// Vector of all comets that exist within the Galaxy (even those that are
    /// "owned" by a star). No elements should ever be removed from this vector.
    pub comets: Vec<Comet>,
    /// Vector of all planets that exist within the Galaxy (even those that are
    /// "owned" by a star). No elements should ever be removed from this vector.
    pub planets: Vec<Planet>,
    /// Vector of all stars that exist within the Galaxy (even those that are
    /// "owned" by a star). No elements should ever be removed from this vector.
    pub stars: Vec<Star>,

    /// A map from the celestial body's id to the index within the corresponding
    /// vector (`comets`, `planets`, or `stars`)
    pub(super) id_to_index: HashMap<ID, CelestialBodyIndex>,
}

impl Galaxy {
    /// Loads a `Galaxy` from a database. The database will be found by
    /// searching in parent directories for `Database::DEFAULT_FILENAME`.
    ///
    /// # Returns
    /// A new `Galaxy` object.
    ///
    /// # Errors
    /// Errors will occur in the following situations:
    /// - The specified database cannot be found or the default database cannot
    ///   be found when `name` is `None`
    /// - There is an error while doing a filesystem operation
    /// - There is an error while parsing the database
    pub fn load() -> Result<Self> {
        let path = Database::location()?;
        let file = fs::File::open(path)?;
        let reader = io::BufReader::new(file);
        Self::load_from_reader(reader)
    }

    /// A helper function that reads the `Database` and uses it to create a
    /// `Galaxy`. This is factored into a separate function primarily for ease
    /// of testing the loading functionality without interacting with IO.
    fn load_from_reader<R: io::Read>(reader: R) -> Result<Self> {
        let value: Database = serde_json::from_reader(reader)?;

        let mut id_to_index: HashMap<ID, CelestialBodyIndex> = HashMap::new();
        for (i, comet) in value.comets.iter().enumerate() {
            id_to_index.insert(
                comet.id,
                CelestialBodyIndex::new(CelestialBodyKind::Comet, i),
            );
        }
        for (i, planet) in value.planets.iter().enumerate() {
            id_to_index.insert(
                planet.id,
                CelestialBodyIndex::new(CelestialBodyKind::Planet, i),
            );
        }
        for (i, star) in value.stars.iter().enumerate() {
            id_to_index.insert(star.id, CelestialBodyIndex::new(CelestialBodyKind::Star, i));
        }

        Ok(Galaxy {
            title: value.title,
            description: value.description,
            next_id: value.next_id,
            comets: value.comets,
            planets: value.planets,
            stars: value.stars,
            id_to_index,
        })
    }

    /// Initializes a new database for `Galaxy` to be saved in. The new database
    /// will be placed in the directory `dir`.
    ///
    /// **WARNING**: This action is destructive. The old database will be
    /// overwritten.
    ///
    /// # Errors
    /// Errors will occur in the following situations:
    /// - There is an error while doing a filesystem operation
    /// - There is an error while parsing the database
    pub fn init(self, mut dir: PathBuf) -> Result<()> {
        dir.push(Database::DEFAULT_FILENAME);
        if dir.exists() {
            return Err(DatabaseError::DatabaseAlreadyExists(
                dir.to_string_lossy().to_string(),
            ));
        }

        let file = fs::File::create(dir)?;
        let writer = io::BufWriter::new(file);
        self.save_to_writer(writer)
    }

    /// Saves `Galaxy` to a database. The database will be found by searching
    /// parent directories for `Database::DEFAULT_FILENAME`.
    ///
    /// **WARNING**: This action is destructive. The old database will be
    /// overwritten.
    ///
    /// # Errors
    /// Errors will occur in the following situations:
    /// - The specified database cannot be found or the default database cannot
    ///   be found when `name` is `None`
    /// - There is an error while doing a filesystem operation
    /// - There is an error while parsing the database
    pub fn save(self) -> Result<()> {
        let path = Database::location()?;
        let file = fs::File::create(path)?;
        let writer = io::BufWriter::new(file);
        self.save_to_writer(writer)
    }

    /// Saves `Galaxy` to the database in `path`. Will create a new database if
    /// one does not exist.
    ///
    /// **WARNING**: This action is destructive. The old database will be
    /// overwritten.
    ///
    /// # Errors
    /// Errors will occur in the following situations:
    /// - There is an error while doing a filesystem operation
    /// - There is an error while parsing the database
    pub fn save_to(self, path: PathBuf) -> Result<()> {
        let file = fs::File::create(path)?;
        let writer = io::BufWriter::new(file);
        self.save_to_writer(writer)
    }

    /// A helper function that creates a `Database` from the `Galaxy` and writes
    /// it to the writer. This is factored into a separate function primarily
    /// for ease of testing the saving functionality without interacting with IO.
    fn save_to_writer<W: io::Write>(self, writer: W) -> Result<()> {
        let db = Database::default()
            .title(self.title)
            .description(self.description)
            .next_id(self.next_id)
            .comets(self.comets)
            .planets(self.planets)
            .stars(self.stars);

        match serde_json::to_writer_pretty(writer, &db) {
            Ok(_) => Ok(()),
            Err(e) => Err(DatabaseError::ParsingError(e)),
        }
    }

    /// Sets the `title` field and returns `self`
    pub fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    /// Sets the `description` field and returns `self`
    pub fn description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    /// Creates a new `Comet` object and registers it with the `Galaxy`
    ///
    /// # Returns
    /// The new `Comet` object
    pub fn comet(&mut self) -> &mut Comet {
        let id = self.next_id();
        let index = self.comets.len();
        info!("Creating new Comet with id {id}");
        // Create new comet and set the id
        let comet = Comet {
            id,
            ..Default::default()
        };
        // put the comet into the vector of comets
        self.comets.push(comet);
        // associate the id with the index
        self.id_to_index
            .insert(id, CelestialBodyIndex::new(CelestialBodyKind::Comet, index));

        &mut self.comets[index]
    }

    /// Creates a new `Planet` object and registers it with the `Galaxy`
    ///
    /// # Returns
    /// The new `Planet` object
    pub fn planet(&mut self) -> &mut Planet {
        let id = self.next_id();
        let index = self.planets.len();
        info!("Creating new Planet with id {id}");
        // Create new planet and set the id
        let planet = Planet {
            id,
            ..Default::default()
        };
        // put the planet into the vector of planets
        self.planets.push(planet);
        // associate the id with the index
        self.id_to_index.insert(
            id,
            CelestialBodyIndex::new(CelestialBodyKind::Planet, index),
        );

        &mut self.planets[index]
    }

    /// Creates a new `Star` object and registers it with the `Galaxy`
    ///
    /// # Returns
    /// The new `Star` object
    pub fn star(&mut self) -> &mut Star {
        let id = self.next_id();
        let index = self.stars.len();
        info!("Creating new Star with id {id}");
        // Create new star and set the id
        let star = Star {
            id,
            ..Default::default()
        };
        // put the star into the vector of stars
        self.stars.push(star);
        // associate the id with the index
        self.id_to_index
            .insert(id, CelestialBodyIndex::new(CelestialBodyKind::Star, index));

        &mut self.stars[index]
    }

    /// Returns the index associated with the celestial body ID if it exists
    pub fn index(&self, id: ID) -> Option<CelestialBodyIndex> {
        self.id_to_index.get(&id).cloned()
    }

    /// Helper function for retrieving and increment the next id
    fn next_id(&mut self) -> ID {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                   TESTS                                    //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod test {
    use chrono::DateTime;

    use crate::core::{Status, StatusHistory};

    use super::*;

    const DB_STRING: &str = r#"{
  "version": 2,
  "comment": "Database for Planit project. See https://github.com/jac-oblong/planit",
  "title": "Test",
  "description": "This is a test",
  "next_id": 4,
  "comets": [
    {
      "id": 0,
      "parent": null,
      "title": "Test Comet",
      "description": "This is a test comet",
      "status": "Todo",
      "history": []
    }
  ],
  "planets": [
    {
      "id": 1,
      "parent": 3,
      "title": "Test Planet 1",
      "description": "This is a test planet",
      "status": "Hold",
      "history": [
        {
          "old": "Todo",
          "new": "Hold",
          "comment": "No",
          "time": "2020-12-25T19:33:51-05:00"
        }
      ],
      "tags": [],
      "fields": {}
    },
    {
      "id": 2,
      "parent": 3,
      "title": "Test Planet 2",
      "description": "This is a test planet",
      "status": "Done",
      "history": [],
      "tags": [
        "tag1",
        "tag2"
      ],
      "fields": {
        "key1": "value1",
        "key2": "value2"
      }
    }
  ],
  "stars": [
    {
      "id": 3,
      "parent": null,
      "title": "Test Star",
      "description": "This is a test star",
      "status": "Todo",
      "history": [],
      "children": [
        1,
        2
      ]
    }
  ]
}"#;

    #[test]
    fn created_comet_added_to_vec_and_map() {
        let mut galaxy = Galaxy::default();
        let _ = galaxy.comet();

        assert_eq!(galaxy.comets.len(), 1);
        assert_eq!(galaxy.id_to_index.len(), 1);

        let id = galaxy.comets.first().unwrap().id;
        assert_eq!(
            galaxy.index(id),
            Some(CelestialBodyIndex::new(CelestialBodyKind::Comet, 0))
        );
    }

    #[test]
    fn created_planet_added_to_vec_and_map() {
        let mut galaxy = Galaxy::default();
        let _ = galaxy.planet();

        assert_eq!(galaxy.planets.len(), 1);
        assert_eq!(galaxy.id_to_index.len(), 1);

        let id = galaxy.planets.first().unwrap().id;
        assert_eq!(
            galaxy.index(id),
            Some(CelestialBodyIndex::new(CelestialBodyKind::Planet, 0))
        );
    }

    #[test]
    fn created_star_added_to_vec_and_map() {
        let mut galaxy = Galaxy::default();
        let _ = galaxy.star();

        assert_eq!(galaxy.stars.len(), 1);
        assert_eq!(galaxy.id_to_index.len(), 1);

        let id = galaxy.stars.first().unwrap().id;
        assert_eq!(
            galaxy.index(id),
            Some(CelestialBodyIndex::new(CelestialBodyKind::Star, 0))
        );
    }

    #[test]
    fn loading_galaxy_produces_correct_object() {
        let reader = io::Cursor::new(DB_STRING);
        let galaxy = Galaxy::load_from_reader(reader).unwrap();

        assert_eq!(galaxy.title, "Test");
        assert_eq!(galaxy.description, "This is a test");
        assert_eq!(galaxy.next_id, 4);

        assert_eq!(galaxy.comets.len(), 1);
        assert_eq!(galaxy.comets[0].id, 0);
        assert_eq!(galaxy.comets[0].parent, None);
        assert_eq!(galaxy.comets[0].title, "Test Comet");
        assert_eq!(galaxy.comets[0].description, "This is a test comet");
        assert_eq!(galaxy.comets[0].status, Status::Todo);
        assert_eq!(galaxy.comets[0].history.len(), 0);

        assert_eq!(galaxy.planets.len(), 2);
        assert_eq!(galaxy.planets[0].id, 1);
        assert_eq!(galaxy.planets[0].parent, Some(3));
        assert_eq!(galaxy.planets[0].title, "Test Planet 1");
        assert_eq!(galaxy.planets[0].description, "This is a test planet");
        assert_eq!(galaxy.planets[0].status, Status::Hold);
        assert_eq!(galaxy.planets[0].history.len(), 1);
        assert_eq!(galaxy.planets[0].history[0].old, Status::Todo);
        assert_eq!(galaxy.planets[0].history[0].new, Status::Hold);
        assert_eq!(galaxy.planets[0].history[0].comment, "No");
        assert_eq!(
            galaxy.planets[0].history[0].time,
            DateTime::parse_from_rfc3339("2020-12-25T19:33:51-05:00").unwrap()
        );
        assert_eq!(galaxy.planets[0].tags.len(), 0);
        assert_eq!(galaxy.planets[0].fields.len(), 0);
        assert_eq!(galaxy.planets[1].id, 2);
        assert_eq!(galaxy.planets[1].parent, Some(3));
        assert_eq!(galaxy.planets[1].title, "Test Planet 2");
        assert_eq!(galaxy.planets[1].description, "This is a test planet");
        assert_eq!(galaxy.planets[1].status, Status::Done);
        assert_eq!(galaxy.planets[1].history.len(), 0);
        assert_eq!(galaxy.planets[1].tags.len(), 2);
        assert_eq!(galaxy.planets[1].tags[0], "tag1");
        assert_eq!(galaxy.planets[1].tags[1], "tag2");
        assert_eq!(galaxy.planets[1].fields.len(), 2);
        assert_eq!(
            galaxy.planets[1].fields.get("key1"),
            Some(&"value1".to_string())
        );
        assert_eq!(
            galaxy.planets[1].fields.get("key2"),
            Some(&"value2".to_string())
        );

        assert_eq!(galaxy.stars.len(), 1);
        assert_eq!(galaxy.stars[0].id, 3);
        assert_eq!(galaxy.stars[0].parent, None);
        assert_eq!(galaxy.stars[0].title, "Test Star");
        assert_eq!(galaxy.stars[0].description, "This is a test star");
        assert_eq!(galaxy.stars[0].status, Status::Todo);
        assert_eq!(galaxy.stars[0].history.len(), 0);
        assert_eq!(galaxy.stars[0].children.len(), 2);
        assert_eq!(galaxy.stars[0].children[0], 1);
        assert_eq!(galaxy.stars[0].children[1], 2);

        assert_eq!(galaxy.id_to_index.len(), 4);
        assert_eq!(
            galaxy.index(0).unwrap(),
            CelestialBodyIndex::new(CelestialBodyKind::Comet, 0)
        );
        assert_eq!(
            galaxy.index(1).unwrap(),
            CelestialBodyIndex::new(CelestialBodyKind::Planet, 0)
        );
        assert_eq!(
            galaxy.index(2).unwrap(),
            CelestialBodyIndex::new(CelestialBodyKind::Planet, 1)
        );
        assert_eq!(
            galaxy.index(3).unwrap(),
            CelestialBodyIndex::new(CelestialBodyKind::Star, 0)
        );
    }

    #[test]
    fn saving_galaxy_produces_correct_string() {
        let galaxy = Galaxy {
            title: "Test".to_string(),
            description: "This is a test".to_string(),
            next_id: 4,
            comets: vec![Comet {
                id: 0,
                parent: None,
                title: "Test Comet".to_string(),
                description: "This is a test comet".to_string(),
                status: Status::Todo,
                history: vec![],
            }],
            planets: vec![
                Planet {
                    id: 1,
                    parent: Some(3),
                    title: "Test Planet 1".to_string(),
                    description: "This is a test planet".to_string(),
                    status: Status::Hold,
                    history: vec![StatusHistory {
                        old: Status::Todo,
                        new: Status::Hold,
                        comment: "No".to_string(),
                        time: DateTime::parse_from_rfc3339("2020-12-25T19:33:51-05:00")
                            .unwrap()
                            .into(),
                    }],
                    tags: vec![],
                    fields: HashMap::default(),
                },
                Planet {
                    id: 2,
                    parent: Some(3),
                    title: "Test Planet 2".to_string(),
                    description: "This is a test planet".to_string(),
                    status: Status::Done,
                    history: vec![],
                    tags: vec!["tag1".to_string(), "tag2".to_string()],
                    fields: HashMap::from([
                        ("key1".to_string(), "value1".to_string()),
                        ("key2".to_string(), "value2".to_string()),
                    ]),
                },
            ],
            stars: vec![Star {
                id: 3,
                parent: None,
                title: "Test Star".to_string(),
                description: "This is a test star".to_string(),
                status: Status::Todo,
                history: vec![],
                children: vec![1, 2],
            }],
            id_to_index: HashMap::from([
                (0, CelestialBodyIndex::new(CelestialBodyKind::Comet, 0)),
                (1, CelestialBodyIndex::new(CelestialBodyKind::Planet, 0)),
                (2, CelestialBodyIndex::new(CelestialBodyKind::Planet, 1)),
                (3, CelestialBodyIndex::new(CelestialBodyKind::Star, 0)),
            ]),
        };

        let mut writer = Vec::new();
        galaxy.save_to_writer(&mut writer).unwrap();
        assert_eq!(writer, DB_STRING.as_bytes());
    }

    #[test]
    fn loaded_galaxy_can_be_saved_without_changes() {
        let reader = io::Cursor::new(DB_STRING);
        let mut writer = Vec::new();
        let galaxy = Galaxy::load_from_reader(reader).unwrap();
        galaxy.save_to_writer(&mut writer).unwrap();
        assert_eq!(String::from_utf8(writer).unwrap(), DB_STRING);
    }
}
