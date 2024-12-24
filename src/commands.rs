use std::collections::HashSet;

use anyhow::Result;
use clap::{Args, Subcommand};

#[derive(Args)]
pub struct FocusGroup {
    name: String,
}

impl FocusGroup {
    pub fn run(&self, state: crate::state::State) -> Result<()> {
        let group = if self.name.is_empty() {
            None
        } else {
            Some(self.name.as_str())
        };
        crate::workspace_group::focus_group(&state, group)
    }
}

#[derive(Args)]
pub struct FocusWorkspace {
    group_workspace: i64,
}

impl FocusWorkspace {
    pub fn run(&self, state: crate::state::State) -> Result<()> {
        crate::workspace_group::focus_group_workspace(&state, self.group_workspace)
    }
}

#[derive(Args)]
pub struct FocusI3Workspace {
    name: String,
}

impl FocusI3Workspace {
    pub fn run(&self, state: crate::state::State) -> Result<()> {
        crate::workspace_group::Workspace::of_i3_workspace_name(&state, &self.name)?
            .focus(&state)
    }
}

#[derive(Subcommand)]
pub enum Focus {
    Group(FocusGroup),
    Workspace(FocusWorkspace),
    WmWorkspace(FocusI3Workspace),
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
                let groups = crate::workspace_group::WorkspaceID::list(&state)?
                    .into_iter()
                    .map(|v| v.group().cloned())
                    .collect::<HashSet<_>>();
                for group in groups {
                    println!("{}", group.unwrap_or_default())
                }
                Ok(())
            }
            List::WmWorkspaces => {
                let list = crate::workspace_group::WorkspaceID::list(&state)?;
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
            Some(self.name.as_str())
        };
        let focused_workspace = crate::workspace_group::Workspace::get_focused(&state)?;
        let old_group = focused_workspace.id().group().map(|v| v.as_str());
        crate::workspace_group::rename_group(&state, old_group, new_group)
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

#[derive(Args)]
pub struct MoveWindowToGroupWorkspace {
    pub workspace: i64,
}

impl MoveWindowToGroupWorkspace {
    pub fn run(&self, state: crate::state::State) -> Result<()> {
        crate::workspace_group::move_window_to_group_workspace(&state, self.workspace)
    }
}

#[derive(Args)]
pub struct MoveWindowToWorkspace {
    pub workspace: String,
}

impl MoveWindowToWorkspace {
    pub fn run(&self, state: crate::state::State) -> Result<()> {
        crate::workspace_group::move_window_to_workspace(&state, &self.workspace)
    }
}

#[derive(Subcommand)]
pub enum Move {
    WindowToGroupWorkspace(MoveWindowToGroupWorkspace),
    WindowToWorkspace(MoveWindowToWorkspace),
}

impl Move {
    pub fn run(&self, state: crate::state::State) -> Result<()> {
        match self {
            Move::WindowToGroupWorkspace(cmd) => cmd.run(state),
            Move::WindowToWorkspace(cmd) => cmd.run(state),
        }
    }
}
