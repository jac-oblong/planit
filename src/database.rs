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
 * Module containing all functionality for interacting with the filesystem
 * database. This includes loading from a database and saving to a database.
 *
 * # Examples
 *
 * ```no_run
 * use planit::database;
 * let planets = database::load(None)?;
 * // Do some operations
 * database::save(planets, None)?;
 * # Ok::<(), database::DatabaseError>(())
 * ```
 */

use std::{
    env, error, fmt, fs, io,
    option::Option,
    path::{Path, PathBuf},
    result,
};

use serde::{Deserialize, Serialize};

use crate::core::Planet;

type Result<T> = std::result::Result<T, DatabaseError>;

/// Possible errors when loading / saving a database.
#[derive(Debug)]
pub enum DatabaseError {
    /// The specified database (or the default) could not be found
    DatabaseNotFound(String),
    /// An error occurred while performing an filesystem operation
    FileSystemError(io::Error),
    /// An error occurrd while parsing the database
    ParsingError(serde_json::Error),
}

impl error::Error for DatabaseError {}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatabaseError::DatabaseNotFound(name) => {
                write!(f, "Database not found: {name}")
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

/// The representation of the database
/// NOTE: If this struct is changed in any way, the `SCHEMA_VERSION` needs to be
/// incremented (see below)
#[derive(Deserialize, Serialize)]
struct Database {
    /// This represents the current schema version. It should NEVER be edited.
    #[serde(deserialize_with = "ensure_database_version")]
    version: u64,

    /// All planets that exist in this database
    planets: Vec<Planet>,
}

impl Database {
    /// NOTE: This value must be incremented if the struct above is changed
    const SCHEMA_VERSION: u64 = 1;

    /// The default file to search for if no database name is given
    const DEFAULT_FILENAME: &str = ".planit.json";
}

fn ensure_database_version<'de, D: serde::Deserializer<'de>>(
    d: D,
) -> result::Result<u64, D::Error> {
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

/// Loads a list of `Planets` from a database. The database can optionally be
/// specified. If the `name` is left as `None` then the database will be found
/// by searching in parent directories for a file named ".planit.json".
///
/// # Arguments
/// - `name`: The name of the database to load. `None` to load the default.
///
/// # Returns
/// A vector of all planets in the specified database.
///
/// # Errors
/// Errors will occur in the following situations:
/// - The specified database cannot be found
/// - There is an error while doing a filesystem operation
/// - There is an error while parsing the database
pub fn load(name: Option<String>) -> Result<Vec<Planet>> {
    let path = database_location(name)?;
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    let value: Database = serde_json::from_reader(reader)?;
    Ok(value.planets)
}

/// Saves a list of `Planets` to a database. The database can optionally be
/// specified. If the `name` is left as `None` then the database will be found
/// by searching in parent directories for a file named ".planit.json". This
/// action is destructive. The old database will be overwritten.
///
/// # Arguments
/// - `planets`: A vector of planets to save to the database. This will overwrite
///   the existing database with the planets from this vector.
/// - `name`: The name of the database to save to. `None` to save to the default.
///
/// # Returns
/// A vector of all planets in the specified database.
///
/// # Errors
/// Errors will occur in the following situations:
/// - The specified database cannot be found
/// - There is an error while doing a filesystem operation
/// - There is an error while parsing the database
pub fn save(planets: Vec<Planet>, name: Option<String>) -> Result<()> {
    let path = database_location(name)?;
    let file = fs::File::create(path)?;
    let writer = io::BufWriter::new(file);
    let value = Database {
        version: Database::SCHEMA_VERSION,
        planets,
    };
    match serde_json::to_writer_pretty(writer, &value) {
        Ok(_) => Ok(()),
        Err(e) => Err(DatabaseError::ParsingError(e)),
    }
}

/// Finds the location for the database file. If `name` is `None`, the default
/// database will be found. The default is a file named ".planit.json" in one
/// of the parent directories.
///
/// # Arguments
/// - `name`: The name of the database to return. `None` to return the default.
///
/// # Errors
/// Errors will occur in the following situations:
/// - The specified database cannot be found. This includes if the default
///   database cannot be found.
fn database_location(name: Option<String>) -> Result<PathBuf> {
    match name {
        Some(name) => todo!("Implement finding specific database"),
        None => {
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
    }
}
