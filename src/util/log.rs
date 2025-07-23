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
 * Helper utilities related to logging
 */

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                  IMPORTS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

use std::fs;

use log::LevelFilter;
use tui_logger::{
    init_logger, set_default_level, set_env_filter_from_string, set_log_file, TuiLoggerFile,
};

pub use log::{debug, error, info, trace, warn};

use super::dir;

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                 FUNCTIONS                                  //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

pub fn init() {
    init_logger(LevelFilter::Trace).expect("Could not initialize logging");
    set_default_level(LevelFilter::Trace);

    if let Ok(s) = std::env::var("PLANIT_LOG_LEVEL") {
        set_env_filter_from_string(&s);
    } else if let Ok(s) = std::env::var("RUST_LOG") {
        set_env_filter_from_string(&s);
    }

    let mut path = dir::cache().expect("Could not find directory to store logs");
    fs::create_dir_all(&path).expect("Could not create directory to store logs");
    path.push("planit.log");
    if !path.exists() {
        let _ = fs::File::create(&path);
    }
    let file = TuiLoggerFile::new(path.to_str().expect("Invalid path"));
    set_log_file(file);
}
