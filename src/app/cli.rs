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
 * Contains the implementation for the command line application.
 */

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                  IMPORTS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

use std::{env, path::PathBuf};

use clap::{ArgAction, Args, Subcommand};
pub use clap::{Parser, ValueEnum};

use super::Result;
use crate::core::{CelestialBodyKind, Galaxy};

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                  STRUCTS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Sets the current working directory for the given command
    #[arg(short, long)]
    pub dir: Option<PathBuf>,

    /// Adds more logging messages
    #[arg(short, long, action = ArgAction::Count)]
    pub verbose: u8,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new Galaxy in the current directory
    Init(InitArgs),
    /// List the celestial bodies in the Galaxy
    List(ListArgs),
    /// Create a new celestial body
    New(NewArgs),
}

#[derive(Args)]
pub struct InitArgs {
    /// Title for the new project
    pub title: String,
    /// Description for the new project
    pub description: Option<String>,
}

#[derive(Args)]
pub struct ListArgs {
    /// List recursively, or just list top-level
    #[arg(short, long)]
    pub recursive: bool,
}

#[derive(Args)]
pub struct NewArgs {
    /// Type of celestial body to create
    #[arg(value_enum)]
    pub kind: CelestialBodyKind,
    /// Title for the new celestial body
    pub title: String,
    /// Description for the new celestial body
    pub description: Option<String>,
}

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                 FUNCTIONS                                  //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

/// Initializes a new Galaxy in the current directory
pub fn init(args: InitArgs) -> Result<()> {
    let mut galaxy = Galaxy::default().title(args.title);
    if let Some(description) = args.description {
        galaxy = galaxy.description(description);
    }

    let dir = env::current_dir()?;
    galaxy.init(dir)?;

    Ok(())
}

/// Lists all celestial bodies in the Galaxy
pub fn list(args: ListArgs) -> Result<()> {
    todo!()
}

/// Creates a new celestial body
pub fn new(args: NewArgs) -> Result<()> {
    todo!()
}
