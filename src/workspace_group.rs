use anyhow::Result;

fn get_i3_workspaces(state: &crate::state::State) -> Result<Vec<i3ipc_jl::reply::Workspace>> {
    let r = state
        .with_i3connection(|conn| conn.get_workspaces())?
        .workspaces;
    Ok(r)
}

// 1           1 none none               JustI3ID
// 1:web       1 "web" none              I3IDWithGroup
// web         none "web" none           JustGroup
// web:1       none "web" 1              GroupWithWorkspace
// 1:web:1     1 "web" 1                 Full
// other:web:1 none "other:web" 1        GroupWithWorkspace
// other:web:a none "other:web:a" none   GroupWithWorkspace
// 1:1         1 "1" none                I3IDWithGroup
// aa:bb:cc    none "aa:bb" none         JustGroup
#[derive(Clone)]
pub enum WorkspaceID {
    JustI3ID(i64),
    I3IDWithGroup(i64, String),
    JustGroup(String),
    GroupWithWorkspace(String, i64),
    Full(i64, String, i64),
}

impl WorkspaceID {
    pub fn list(state: &crate::state::State) -> Result<Vec<Self>> {
        let workspaces = get_i3_workspaces(state)?;
        Ok(workspaces
            .into_iter()
            .map(|w| Self::of_i3_workspace(&w.name))
            .collect())
    }

    pub fn i3_id(&self) -> Option<i64> {
        match self {
            Self::JustI3ID(i3_id) => Some(*i3_id),
            Self::I3IDWithGroup(i3_id, _) => Some(*i3_id),
            Self::JustGroup(_) => None,
            Self::GroupWithWorkspace(_, _) => None,
            Self::Full(i3_id, _, _) => Some(*i3_id),
        }
    }

    pub fn is_just_i3_id(&self) -> bool {
        matches!(self, Self::JustI3ID(_))
    }

    pub fn i3_workspace_name(&self) -> String {
        match self {
            Self::JustI3ID(i3_id) => format!("{i3_id}"),
            Self::I3IDWithGroup(i3_id, group_name) => format!("{i3_id}:{group_name}"),
            Self::JustGroup(group_name) => group_name.to_owned(),
            Self::GroupWithWorkspace(group_name, group_workspace) => {
                format!("{group_name}:{group_workspace}")
            }
            Self::Full(i3_id, group_name, group_workspace) => {
                format!("{i3_id}:{group_name}:{group_workspace}")
            }
        }
    }

    pub fn with_group(&self, group: Option<&str>) -> Self {
        match (self, group) {
            (Self::JustI3ID(i3_id), Some(group_name)) => {
                Self::Full(*i3_id, group_name.to_owned(), *i3_id)
            }
            (Self::JustI3ID(i3_id), None) => Self::JustI3ID(*i3_id),
            (Self::I3IDWithGroup(i3_id, _), Some(group_name)) => {
                Self::I3IDWithGroup(*i3_id, group_name.to_owned())
            }
            (Self::I3IDWithGroup(i3_id, _), None) => Self::JustI3ID(*i3_id),
            (Self::JustGroup(_), Some(group_name)) => Self::JustGroup(group_name.to_owned()),
            (Self::JustGroup(_), None) => Self::JustGroup(String::new()),
            (Self::GroupWithWorkspace(_, group_workspace), Some(group_name)) => {
                Self::GroupWithWorkspace(group_name.to_owned(), *group_workspace)
            }
            (Self::GroupWithWorkspace(_, group_workspace), None) => {
                Self::JustI3ID(*group_workspace)
            }
            (Self::Full(i3_id, _, group_workspace), Some(group_name)) => {
                Self::Full(*i3_id, group_name.to_owned(), *group_workspace)
            }
            (Self::Full(_, _, group_workspace), None) => Self::JustI3ID(*group_workspace),
        }
    }

    pub fn with_group_workspace(self, group_workspace: i64) -> Self {
        match self {
            Self::JustI3ID(_) => Self::JustI3ID(group_workspace),
            Self::I3IDWithGroup(i3_id, group_name) => {
                Self::Full(i3_id, group_name, group_workspace)
            }
            Self::JustGroup(group_name) => Self::GroupWithWorkspace(group_name, group_workspace),
            Self::GroupWithWorkspace(group_name, _) => {
                Self::GroupWithWorkspace(group_name, group_workspace)
            }
            Self::Full(i3_id, group_name, _) => Self::Full(i3_id, group_name, group_workspace),
        }
    }

    pub fn with_i3_id(self, i3_id: i64) -> Self {
        match self {
            Self::JustI3ID(_) => Self::JustI3ID(i3_id),
            Self::I3IDWithGroup(_, group_name) => Self::I3IDWithGroup(i3_id, group_name),
            Self::JustGroup(group_name) => Self::I3IDWithGroup(i3_id, group_name),
            Self::GroupWithWorkspace(group_name, group_workspace) => {
                Self::Full(i3_id, group_name, group_workspace)
            }
            Self::Full(_, group_name, group_workspace) => {
                Self::Full(i3_id, group_name, group_workspace)
            }
        }
    }

    pub fn group(&self) -> Option<&String> {
        match self {
            Self::JustI3ID(_) => None,
            Self::I3IDWithGroup(_, group_name) => Some(group_name),
            Self::JustGroup(group_name) => Some(group_name),
            Self::GroupWithWorkspace(group_name, _) => Some(group_name),
            Self::Full(_, group_name, _) => Some(group_name),
        }
    }

    pub fn group_workspace(&self) -> Option<i64> {
        match self {
            Self::JustI3ID(_) => None,
            Self::I3IDWithGroup(_, _) => None,
            Self::JustGroup(_) => None,
            Self::GroupWithWorkspace(_, group_workspace) => Some(*group_workspace),
            Self::Full(_, _, group_workspace) => Some(*group_workspace),
        }
    }

    fn of_triplet(i3_id: &str, group_name: &str, group_workspace: &str) -> Self {
        match (
            i3_id.parse::<i64>().ok(),
            group_workspace.parse::<i64>().ok(),
        ) {
            (Some(i3_id), Some(group_workspace)) => {
                Self::Full(i3_id, group_name.to_string(), group_workspace)
            }
            (Some(i3_id), None) => {
                Self::I3IDWithGroup(i3_id, format!("{group_name}:{group_workspace}"))
            }
            (None, Some(group_workspace)) => {
                Self::GroupWithWorkspace(format!("{i3_id}:{group_name}"), group_workspace)
            }
            (None, None) => Self::JustGroup(format!("{i3_id}:{group_name}:{group_workspace}")),
        }
    }

    pub fn of_i3_workspace(i3_workspace_name: &str) -> Self {
        let fields = i3_workspace_name.split(':').collect::<Vec<_>>();
        match *fields.as_slice() {
            [i3_id, group_name, group_workspace] => {
                Self::of_triplet(i3_id, group_name, group_workspace)
            }
            [field1, field2] => match (field1.parse::<i64>().ok(), field2.parse::<i64>().ok()) {
                (Some(field1_num), _) => Self::I3IDWithGroup(field1_num, field2.to_owned()),
                (None, Some(field2_num)) => Self::GroupWithWorkspace(field1.to_owned(), field2_num),
                (None, None) => Self::JustGroup(i3_workspace_name.to_owned()),
            },
            [field] => match field.parse::<i64>().ok() {
                Some(field_num) => Self::JustI3ID(field_num),
                None => Self::JustGroup(i3_workspace_name.to_owned()),
            },
            [] => Self::JustGroup(String::new()),
            _ => {
                let first = fields.first().unwrap();
                let last = fields.last().unwrap();
                let middle = &fields[1..fields.len() - 1];
                Self::of_triplet(first, &middle.join(":").to_owned(), last)
            }
        }
    }

    pub fn focus(&self, state: &crate::state::State) -> Result<()> {
        state.run_i3_command(&format!("workspace {}", self.i3_workspace_name()))?;

        Ok(())
    }

    pub fn rename(&self, state: &crate::state::State, new_id: &Self) -> Result<()> {
        state
            .run_i3_command(&format!(
                "rename workspace \"{}\" to \"{}\"",
                self.i3_workspace_name(),
                new_id.i3_workspace_name()
            ))
            .map_err(anyhow::Error::from)
    }

    pub fn cmp_group_and_workspace(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        let r = self.group().cmp(&other.group());
        if r != Ordering::Equal {
            return r;
        }
        self.group_workspace().cmp(&other.group_workspace())
    }

    pub fn eq_group_and_workspace(&self, other: &Self) -> bool {
        self.cmp_group_and_workspace(other) == std::cmp::Ordering::Equal
    }
}

pub struct Workspace {
    workspace_with_group: WorkspaceID,
    workspace: i3ipc_jl::reply::Workspace,
}

impl Workspace {
    pub fn of_i3_workspace(workspace: i3ipc_jl::reply::Workspace) -> Self {
        Workspace {
            workspace_with_group: WorkspaceID::of_i3_workspace(&workspace.name),
            workspace,
        }
    }

    pub fn of_i3_workspace_name(state: &crate::state::State, name: &str) -> Result<Self> {
        let workspaces = get_i3_workspaces(state)?;
        let workspace = workspaces
            .into_iter()
            .find(|w| w.name == name)
            .ok_or(anyhow::anyhow!("Workspace not found"))?;
        Ok(Workspace::of_i3_workspace(workspace))
    }

    pub fn id(&self) -> &WorkspaceID {
        &self.workspace_with_group
    }

    pub fn with_group(self, group: Option<&str>) -> Self {
        Workspace {
            workspace_with_group: self.workspace_with_group.with_group(group),
            workspace: self.workspace,
        }
    }

    pub fn with_group_workspace(self, group_workspace: i64) -> Self {
        Workspace {
            workspace_with_group: self
                .workspace_with_group
                .with_group_workspace(group_workspace),
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

pub fn reassign_i3_ids(state: &crate::state::State) -> Result<()> {
    let mut ids = Workspace::list(state)?
        .into_iter()
        .map(|w| w.id().to_owned())
        .collect::<Vec<_>>();
    ids.sort_by(|a, b| a.cmp_group_and_workspace(b));
    let mut i3_id = ids
        .iter()
        .filter(|id| id.is_just_i3_id())
        .filter_map(|id| id.i3_id())
        .max()
        .unwrap_or(0)
        + 1;
    for id in ids {
        if id.is_just_i3_id() {
            continue;
        }
        slog_scope::info!("Renaming {:?}", id.i3_workspace_name());
        let new_id = id.clone().with_i3_id(i3_id);
        id.rename(state, &new_id)?;
        i3_id += 1
    }
    Ok(())
}

pub fn focus_group(state: &crate::state::State, group: Option<&str>) -> Result<()> {
    let workspaces = Workspace::list(state)?;
    for workspace in workspaces {
        if workspace.id().group().map(|v| v.as_str()) != group {
            continue;
        }

        return workspace.id().focus(state);
    }

    let new_id = match group {
        Some(group_name) => WorkspaceID::GroupWithWorkspace(group_name.to_owned(), 1),
        None => WorkspaceID::JustI3ID(1),
    };
    new_id.focus(state)?;
    reassign_i3_ids(state)
}

pub fn rename_group(
    state: &crate::state::State,
    group: Option<&str>,
    new_group: Option<&str>,
) -> Result<()> {
    for workspace in Workspace::list(state)? {
        if workspace.id().group().map(|v| v.as_str()) != group {
            continue;
        }
        let new_id = workspace.id().with_group(new_group);
        workspace.id().rename(state, &new_id)?;
        state
            .run_i3_command(&format!(
                "rename workspace \"{}\" to \"{}\"",
                workspace.id().i3_workspace_name(),
                new_id.i3_workspace_name()
            ))
            .map_err(anyhow::Error::from)?;
    }
    reassign_i3_ids(state)
}

pub fn move_window_to_group_workspace(state: &crate::state::State, workspace: i64) -> Result<()> {
    let new_id = Workspace::get_focused(state)?
        .id()
        .clone()
        .with_group_workspace(workspace);
    state.run_i3_command(&format!(
        "move container to workspace {}",
        new_id.i3_workspace_name()
    ))
}

pub fn move_window_to_workspace(state: &crate::state::State, id: &str) -> Result<()> {
    let new_id = WorkspaceID::of_i3_workspace(id);
    state.run_i3_command(&format!(
        "move container to workspace {}",
        new_id.i3_workspace_name()
    ))
}

pub fn focus_group_workspace(state: &crate::state::State, group_workspace: i64) -> Result<()> {
    let workspaces = Workspace::list(state)?;
    let focused = workspaces
        .iter()
        .find(|w| w.workspace.focused)
        .ok_or(anyhow::anyhow!("No focused workspace"))?;
    let existing = workspaces.iter().find(|ws| {
        ws.id().group() == focused.id().group()
            && ws.id().group_workspace() == Some(group_workspace)
    });
    if let Some(existing) = existing {
        return existing.focus(state);
    }
    focused
        .id()
        .clone()
        .with_group_workspace(group_workspace)
        .focus(state)?;
    reassign_i3_ids(state)
}
