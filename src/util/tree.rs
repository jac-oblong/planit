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
 * Utilities related to printing a tree-like structure in a pretty way. This is
 * primarily targetted towards printing celestial bodies to stdout.
 *
 * An example of the format is shown below.
 *
 * ```ignore
 *  ┏━ <Root Title>
 *  ┃  <Root Description>
 *  ┃
 *  ┣━ <Node Icon> <Node Label> <Node Status> <Node Title>
 *  ┃              <Node Description>
 *  ┣━ <Node Icon> <Node Label> <Node Status> <Node Title>
 *  ┃              <Node Description>
 *  ┣━ <Node Icon> <Node Label> <Node Status> <Node Title>
 *  ┃       ┃      <Node Description>
 *  ┃       ┣━ <Node Icon> <Node Label> <Node Status> <Node Title>
 *  ┃       ┃              <Node Description>
 *  ┃       ┗━ <Node Icon> <Node Label> <Node Status> <Node Title>
 *  ┃                      <Node Description>
 *  ┗━ <Node Icon> <Node Label> <Node Status> <Node Title>
 *          ┃      <Node Description>
 *          ┗━ <Node Icon> <Node Label> <Node Status> <Node Title>
 *                         <Node Description>
 *  ```
 *
 *  The format can also be customized to not show descriptions or not recurse
 *  into children
 */

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                  IMPORTS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

use std::io;

use colored::{ColoredString, Colorize};

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                   TRAITS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

/// Used for pretty-printing trees. This is primarily designed for nodes that
/// have a icon (should only be a single character), a label (denoting the
/// type / kind of object it is), a status, a title, and a description. Each
/// node may also have some number of children.
///
/// An example of the printed format is shown below.
///
/// ```ignore
/// <Node Icon> <Node Label> <Node Status> <Node Title>
///             <Node Description>
/// ```
///
/// The format can also be customized to not show descriptions or not recurse
/// into children.
pub trait PrintTreeNode<T> {
    /// The icon associated with the Node
    fn icon(&self) -> ColoredString;
    /// Labels the type / kind of object it is
    fn label(&self) -> ColoredString;
    /// Status of the node
    fn status(&self) -> ColoredString;
    /// Title of the node
    fn title(&self) -> ColoredString;
    /// Description for the node
    fn description(&self) -> ColoredString;
    /// Any potential children of the node
    fn children<'a>(&self, root: &'a T) -> Vec<Box<&'a dyn PrintTreeNode<T>>>;
}

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                 FUNCTIONS                                  //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

/// Pretty-prints the tree to the writer provided
///
/// # Arguments
/// - `root`: The root of the tree. This is passed to children when obtaining
///   their children. This is useful in data-structures where the root owns all
///   memory associated with the data-structure and the children keep track of
///   references between nodes
/// - `w`: Writer to write everything into
/// - `width`: Horizontal character limit. Lines will be truncated to that
///   length (with "..." to show lines were truncated)
/// - `include_description`: Whether or not to print the description field
///   when printing
/// - `recursive`: Recurse into children of the root's children as well
/// - `title`: The title of the root
/// - `description`: The description for the root
/// - `children`: The top-level children of root
pub fn print_to_writer<W: io::Write, T>(
    root: &T,
    w: &mut W,
    width: usize,
    include_description: bool,
    recursive: bool,
    title: ColoredString,
    description: ColoredString,
    children: Vec<Box<&dyn PrintTreeNode<T>>>,
) -> io::Result<()> {
    let top_corner = "┏━ ".purple();
    let vconnector = "┃  ".purple();

    // print out the root
    let root_title = truncate(title, width - top_corner.chars().count());
    writeln!(w, "{}{}", top_corner, root_title)?;
    if include_description {
        let root_description = truncate(description, width - vconnector.chars().count());
        writeln!(w, "{}{}", vconnector, root_description)?;
    }
    writeln!(w, "{}", vconnector)?;

    print_children_to_writer(root, w, width, include_description, recursive, children)?;

    Ok(())
}

// Helper function that prints all of `children`
//
/// # Arguments
/// - `w`: Writer to write everything into
/// - `width`: Horizontal character limit. Lines will be truncated to that
///   length (with "..." to show lines were truncated)
/// - `include_description`: Whether or not to print the description field
///   when printing
/// - `recursive`: Recurse into children of the children given
/// - `children`: Children to write to `w`
fn print_children_to_writer<W: io::Write, T>(
    root: &T,
    w: &mut W,
    width: usize,
    include_description: bool,
    recursive: bool,
    children: Vec<Box<&dyn PrintTreeNode<T>>>,
) -> io::Result<()> {
    let node_piece = "┣━ ".purple();
    let vconnector = "┃  ".purple();
    let bot_corner = "┗━ ".purple();
    let empty = ColoredString::from("   ");

    let mut itr = children.iter().peekable();
    while let Some(child) = itr.next() {
        let is_last = itr.peek().is_none();
        let connector = if is_last { &bot_corner } else { &node_piece };
        let icon = child.icon();
        let line = format!(
            "{}{} {} {} ",
            connector,
            icon,
            child.label(),
            child.status()
        );
        let title = truncate(child.title(), width - line.chars().count());
        writeln!(w, "{}{}", line, title)?;

        if include_description {
            let connector = if is_last { &empty } else { &vconnector };
            let line = format!(
                "{:<width$} ",
                connector,
                width = connector.chars().count() + icon.chars().count()
            );
            let description = truncate(child.description(), width - line.chars().count());
            writeln!(w, "{}{}", line, description)?;
        }

        if recursive {
            print_children_to_writer(
                root,
                w,
                width,
                include_description,
                recursive,
                child.children(root),
            )?;
        }
    }

    Ok(())
}

/// Helper function to truncate the length of `s` to be less than or equal to
/// `width` characters
fn truncate(mut s: ColoredString, width: usize) -> ColoredString {
    if s.input.chars().count() <= width {
        s
    } else {
        s.input.truncate(width - 3);
        s.input.push_str("...");
        s
    }
}
