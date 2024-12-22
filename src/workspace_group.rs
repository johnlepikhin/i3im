use anyhow::Result;
use std::collections::HashSet;

fn get_i3_workspaces(state: &crate::state::State) -> Result<Vec<i3ipc_jl::reply::Workspace>> {
    let r = state
        .with_i3connection(|conn| conn.get_workspaces())?
        .workspaces;
    Ok(r)
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Group {
    name: String,
}

impl Group {
    pub fn new(name: &str) -> Self {
        Group {
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn list(state: &crate::state::State) -> Result<Vec<Self>> {
        let mut r = HashSet::new();
        for workspace in get_i3_workspaces(state)? {
            if let Some(group_name) = workspace.name.split_once(':') {
                let _ = r.insert(Group::new(group_name.0));
            } else {
                let _ = r.insert(Group::new(""));
            }
        }

        Ok(r.into_iter().collect())
    }

    pub fn focus(&self, state: &crate::state::State) -> Result<()> {
        let group = if self.name.is_empty() {
            None
        } else {
            Some(self.clone())
        };
        let new_workspace = CustomWorkspace::list(state)?
            .into_iter()
            .find(|w| *w.group() == group)
            .map(|w| w.workspace_with_group().clone())
            .unwrap_or_else(|| WorkspaceWithGroup::new(group, "1"));

        state.run_i3_command(&format!("workspace {}", new_workspace.i3_workspace_name()))
    }
}

#[derive(Clone)]
pub struct WorkspaceWithGroup {
    group: Option<Group>,
    name: String,
}

impl WorkspaceWithGroup {
    pub fn new(group: Option<Group>, name: &str) -> Self {
        WorkspaceWithGroup {
            group,
            name: name.to_string(),
        }
    }

    pub fn i3_workspace_name(&self) -> String {
        if let Some(group) = &self.group {
            format!("{}:{}", group.name(), self.name)
        } else {
            self.name.clone()
        }
    }

    pub fn with_group(self, group: Option<Group>) -> Self {
        WorkspaceWithGroup {
            group,
            name: self.name,
        }
    }

    pub fn with_name(self, name: &str) -> Self {
        WorkspaceWithGroup {
            group: self.group,
            name: name.to_owned(),
        }
    }

    pub fn group(&self) -> &Option<Group> {
        &self.group
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn of_i3_workspace(i3_workspace_name: &str) -> Self {
        match i3_workspace_name.split_once(':') {
            Some((group_name, workspace_name)) => WorkspaceWithGroup {
                group: Some(Group::new(group_name)),
                name: workspace_name.to_string(),
            },
            None => WorkspaceWithGroup {
                group: None,
                name: i3_workspace_name.to_string(),
            },
        }
    }

    pub fn focus(&self, state: &crate::state::State) -> Result<()> {
        state.run_i3_command(&format!("workspace {}", self.i3_workspace_name()))?;

        Ok(())
    }
}

pub struct CustomWorkspace {
    workspace_with_group: WorkspaceWithGroup,
    workspace: i3ipc_jl::reply::Workspace,
}

impl CustomWorkspace {
    pub fn of_i3_workspace(workspace: i3ipc_jl::reply::Workspace) -> Self {
        CustomWorkspace {
            workspace_with_group: WorkspaceWithGroup::of_i3_workspace(&workspace.name),
            workspace,
        }
    }

    pub fn of_i3_workspace_name(state: &crate::state::State, name: &str) -> Result<Self> {
        let workspaces = get_i3_workspaces(state)?;
        let workspace = workspaces
            .into_iter()
            .find(|w| w.name == name)
            .ok_or(anyhow::anyhow!("Workspace not found"))?;
        Ok(CustomWorkspace::of_i3_workspace(workspace))
    }

    pub fn i3_workspace_name(&self) -> String {
        self.workspace_with_group.i3_workspace_name()
    }

    pub fn group(&self) -> &Option<Group> {
        self.workspace_with_group.group()
    }

    pub fn name(&self) -> &str {
        self.workspace_with_group.name()
    }

    pub fn workspace_with_group(&self) -> &WorkspaceWithGroup {
        &self.workspace_with_group
    }

    pub fn with_group(self, group: Option<Group>) -> Self {
        CustomWorkspace {
            workspace_with_group: self.workspace_with_group.with_group(group),
            workspace: self.workspace,
        }
    }

    pub fn with_name(self, name: &str) -> Self {
        CustomWorkspace {
            workspace_with_group: self.workspace_with_group.with_name(name),
            workspace: self.workspace,
        }
    }

    pub fn focus(&self, state: &crate::state::State) -> Result<()> {
        self.workspace_with_group.focus(state)
    }

    pub fn list(state: &crate::state::State) -> Result<Vec<Self>> {
        let workspaces = get_i3_workspaces(state)?;
        Ok(workspaces.into_iter().map(Self::of_i3_workspace).collect())
    }

    pub fn get_focused(state: &crate::state::State) -> Result<Self> {
        let r = Self::list(state)?
            .into_iter()
            .find(|w| w.workspace.focused)
            .ok_or(anyhow::anyhow!("No focused workspace"))?;
        Ok(r)
    }
}

fn _rename_workspace(
    state: &crate::state::State,
    workspace: Option<CustomWorkspace>,
    new_name: &str,
) -> Result<()> {
    let workspace = if let Some(workspace) = workspace {
        workspace
    } else {
        CustomWorkspace::get_focused(state)?
    };

    let current_i3_workspace_name = workspace.i3_workspace_name();
    let new_workspace = workspace.with_name(new_name);
    state
        .run_i3_command(&format!(
            "rename workspace \"{}\" to \"{}\"",
            current_i3_workspace_name,
            new_workspace.i3_workspace_name()
        ))
        .map_err(anyhow::Error::from)?;

    Ok(())
}

pub fn rename_group(
    state: &crate::state::State,
    group: &Option<Group>,
    new_group: Option<Group>,
) -> Result<()> {
    for custom_workspace in CustomWorkspace::list(state)? {
        if custom_workspace.group() != group {
            continue;
        }
        let old_i3_workspace_name = custom_workspace.i3_workspace_name();
        let new_workspace = custom_workspace.with_group(new_group.clone());
        state
            .run_i3_command(&format!(
                "rename workspace \"{}\" to \"{}\"",
                old_i3_workspace_name,
                new_workspace.i3_workspace_name()
            ))
            .map_err(anyhow::Error::from)?;
    }

    Ok(())
}
