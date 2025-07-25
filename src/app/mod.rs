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
 * Contains the implementation of the application. This includes both the
 * command line version and TUI version.
 */

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                  MODULES                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

pub mod cli;

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                  IMPORTS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

use std::{env, io};

pub use cli::Cli;
use cli::Commands;

use crate::core::galaxy::DatabaseError;

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                   TYPES                                    //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

type Result<T> = std::result::Result<T, AppError>;

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                   ENUMS                                    //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

/// All errors that might happen when running the application
#[derive(Debug)]
pub enum AppError {
    IoError(io::Error),
    DatabaseError(DatabaseError),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "Error during IO operation: {e}"),
            Self::DatabaseError(e) => write!(f, "Error during database operation: {e}"),
        }
    }
}

impl std::error::Error for AppError {}

impl From<io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<DatabaseError> for AppError {
    fn from(value: DatabaseError) -> Self {
        Self::DatabaseError(value)
    }
}

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                 FUNCTIONS                                  //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

/// Runs the application. Does not return until all operations are completed.
///
/// # Arguments
/// `args`: The parsed command line arguments
///
/// # Returns
/// Any errors that are encountered. `Ok(())` otherwise
pub fn run(args: Cli) -> Result<()> {
    if let Some(dir) = args.dir {
        env::set_current_dir(dir)?;
    }

    match args.verbose {
        0 => {}
        _ => todo!(),
    }

    match args.command {
        Some(Commands::Init(args)) => cli::init(args),
        Some(Commands::List(args)) => cli::list(args),
        Some(Commands::New(args)) => cli::new(args),
        None => todo!(),
    }
}
