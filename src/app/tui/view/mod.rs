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
 * Contains the `View` trait. This trait defines the interface with which the
 * application will interact with all views. It also contains the `PaneView`
 * implementation. This is the structure the application uses to organize views
 * and divide them into panes.
 */

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                  MODULES                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

mod default;
mod opening;

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                  IMPORTS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

use std::{cell::RefCell, fmt::Debug, rc::Rc};

use default::DefaultView;
use opening::OpeningView;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
};

use crate::{core::Galaxy, util::queue::PushQueue};

use super::{Command, MovementDirection, SplitDirection};

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                   TRAITS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

/// The interface that all views must implement.
pub trait View: Debug {
    /// Renders self into the given buffer. A reference to the app itself is
    /// given if the view relies on the state of the application.
    ///
    /// # Arguments
    /// - `area`: The area within the buffer that is owned by this view.
    /// - `buf`: The buffer to render into.
    fn render(&self, area: Rect, buf: &mut Buffer);

    /// Updates self based on the given command. Any commands that are not
    /// recognized should be ignored. If the handling of the command results in
    /// other commands that should be processed, the extra commands should be
    /// added to `queue`.
    ///
    /// # Arguments
    /// - `command`: The command to be processed.
    /// - `queue`: All commands that are still waiting to be processed. Any new
    ///   commands that arise from processing `command` should be added to this.
    fn update(&mut self, command: Command, queue: &mut PushQueue<Command>);
}

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                   ENUMS                                    //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

/// A node within the `PaneView`. This can either hold a branch or a leaf.
#[derive(Debug)]
enum PaneNode {
    /// A further collection of views split in a certain direction.
    Branch(PaneBranch),
    /// A single view.
    Leaf(Box<dyn View>),
}

impl PaneNode {
    /// Splits the current node in the direction specified. The provided view
    /// will be placed in the newly created area.
    ///
    /// # Arguments
    /// - `direction`: Direction in which to split the focused view.
    /// - `new_view`: The view to use for the newly created area.
    fn split(&mut self, direction: SplitDirection, new_view: Box<dyn View>) {
        match self {
            PaneNode::Leaf(view) => {
                let nodes: Vec<PaneNode> = vec![PaneNode::Leaf(*view), PaneNode::Leaf(new_view)];
                *self = PaneNode::Branch(PaneBranch::new(direction, nodes));
            }
            PaneNode::Branch(branch) => branch.split(direction, new_view),
        }
    }
}

impl View for PaneNode {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        match self {
            PaneNode::Branch(branch) => branch.render(area, buf),
            PaneNode::Leaf(view) => view.render(area, buf),
        }
    }

    fn update(&mut self, command: Command, queue: &mut PushQueue<Command>) {
        match self {
            PaneNode::Branch(branch) => branch.update(command, queue),
            PaneNode::Leaf(view) => view.update(command, queue),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//                                  STRUCTS                                   //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

/// Organizes multiple views into a grid of panes. This grid can be made up of
/// both vertical and horizontal splits.
#[derive(Debug)]
pub struct PaneView {
    /// Reference to the global galaxy.
    galaxy: Rc<RefCell<Galaxy>>,
    /// The root of the tree of panes.
    root: PaneNode,
}

impl PaneView {
    /// A conversion from a string identifier to a constructor for the
    /// associated view. NOTE: This should eventually be converted into a
    /// hashmap that views can register themselves to, but for now I am fine
    /// with this method. The hashmap can happen when I create trackit.
    const VIEW_CONSTRUCTORS: &[(
        &'static str,
        fn(galaxy: Rc<RefCell<Galaxy>>) -> Box<dyn View>,
    )] = &[
        ("opening", |_| Box::new(OpeningView)),
        ("default", |_| Box::new(DefaultView)),
    ];

    /// Creates a new pane view with the opening view.
    ///
    /// # Arguments
    /// - `galaxy`: A reference to the global galaxy.
    pub fn new(galaxy: Rc<RefCell<Galaxy>>) -> Self {
        let opening_view = Self::open_view("opening".to_string(), galaxy.clone()).unwrap();
        Self {
            galaxy,
            root: PaneNode::Leaf(opening_view),
        }
    }

    /// Calls the constructor for the view and returns the constructed view.
    ///
    /// # Arguments
    /// - `view`: The string identifier for the view.
    /// - `galaxy`: A reference to the global galaxy.
    ///
    /// # Returns
    /// `Some(...)` if the identifier was found. `None` otherwise.
    fn open_view(view: String, galaxy: Rc<RefCell<Galaxy>>) -> Option<Box<dyn View>> {
        for (identifier, constructor) in Self::VIEW_CONSTRUCTORS {
            if view == *identifier {
                return Some(constructor(galaxy.clone()));
            }
        }
        None
    }
}

impl View for PaneView {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        self.root.render(area, buf);
    }

    fn update(&mut self, command: Command, queue: &mut PushQueue<Command>) {
        match command {
            Command::MoveFocus(direction) => self.root.move_focus(direction),
            Command::SplitView(direction) => {
                let new_view = Self::open_view("default".to_string(), self.galaxy.clone()).unwrap();
                self.root.split(direction, new_view);
            }
            command => self.root.update(command, queue),
        }
    }
}

/// A branch within the `PaneView`. This divides its pane into multiple
/// sections (or branches), each of which is used by a child.
#[derive(Debug)]
struct PaneBranch {
    /// The direction that the pane is split.
    axis: SplitDirection,
    /// The child nodes of this branch.
    nodes: Vec<PaneNode>,
    /// The percentage of the pane used by each node. `nodes` and `percentage`
    /// should always be the same length. Additionally, the sum of `percentage`
    /// should be 100.
    percentage: Vec<u16>,
    /// The child that currently has focus.
    focused: usize,
}

impl PaneBranch {
    fn new(axis: SplitDirection, nodes: Vec<PaneNode>) -> Self {
        todo!()
    }

    fn split(&mut self, direction: SplitDirection, new_view: Box<dyn View>) {
        todo!()
    }
}

impl View for PaneBranch {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        assert_eq!(self.percentage.iter().sum::<u16>(), 100);
        assert_eq!(self.percentage.len(), self.nodes.len());

        let layouts = Layout::default()
            .direction(self.axis.into())
            .constraints(self.percentage.iter().map(|x| Constraint::Percentage(*x)))
            .split(area);

        for itr in self.nodes.iter().zip(layouts.iter()) {
            let (node, layout) = itr;
            node.render(*layout, buf);
        }
    }

    fn update(&mut self, command: Command, queue: &mut PushQueue<Command>) {
        self.nodes[self.focused].update(command, queue);
    }
}
