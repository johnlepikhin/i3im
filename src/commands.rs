use anyhow::Result;
use clap::{Args, Subcommand};

#[derive(Args)]
pub struct FocusGroup {
    name: String,
}

impl FocusGroup {
    pub fn run(&self, state: crate::state::State) -> Result<()> {
        crate::workspace_group::Group::new(&self.name).focus(&state)
    }
}

#[derive(Args)]
pub struct FocusWorkspace {
    name: String,
}

impl FocusWorkspace {
    pub fn run(&self, state: crate::state::State) -> Result<()> {
        let focused = crate::workspace_group::CustomWorkspace::get_focused(&state)?;
        focused.with_name(&self.name).focus(&state)
    }
}

#[derive(Args)]
pub struct FocusWmWorkspace {
    name: String,
}

impl FocusWmWorkspace {
    pub fn run(&self, state: crate::state::State) -> Result<()> {
        crate::workspace_group::CustomWorkspace::of_i3_workspace_name(&state, &self.name)?
            .focus(&state)
    }
}

#[derive(Subcommand)]
pub enum Focus {
    Group(FocusGroup),
    Workspace(FocusWorkspace),
    WmWorkspace(FocusWmWorkspace),
}

impl Focus {
    pub fn run(&self, state: crate::state::State) -> Result<()> {
        match self {
            Focus::Group(group) => group.run(state),
            Focus::Workspace(workspace) => workspace.run(state),
            Focus::WmWorkspace(workspace) => workspace.run(state),
        }
    }
}

#[derive(Subcommand)]
pub enum List {
    Groups,
    WmWorkspaces,
}

impl List {
    pub fn run(&self, state: crate::state::State) -> Result<()> {
        match self {
            List::Groups => {
                let list = crate::workspace_group::Group::list(&state)?;
                for group in list {
                    println!("{}", group.name());
                }
                Ok(())
            }
            List::WmWorkspaces => {
                let list = crate::workspace_group::CustomWorkspace::list(&state)?;
                for workspace in list {
                    println!("{}", workspace.i3_workspace_name());
                }
                Ok(())
            }
        }
    }
}

#[derive(Args)]
pub struct RenameGroup {
    pub name: String,
}

impl RenameGroup {
    pub fn run(&self, state: crate::state::State) -> Result<()> {
        let new_group = if self.name.is_empty() {
            None
        } else {
            Some(crate::workspace_group::Group::new(&self.name))
        };
        let old_group = crate::workspace_group::CustomWorkspace::get_focused(&state)?
            .group()
            .clone();
        crate::workspace_group::rename_group(&state, &old_group, new_group)
    }
}

#[derive(Subcommand)]
pub enum Rename {
    Group(RenameGroup),
}

impl Rename {
    pub fn run(&self, state: crate::state::State) -> Result<()> {
        match self {
            Rename::Group(group) => group.run(state),
        }
    }
}
