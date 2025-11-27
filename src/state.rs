use std::sync::{Arc, Mutex};

use anyhow::Result;

pub struct State {
    i3connection: Arc<Mutex<i3ipc_jl::I3Connection>>,
    config: Arc<Mutex<crate::config::Config>>,
}

impl State {
    pub fn new(config: crate::config::Config) -> Result<Self> {
        let i3connection = i3ipc_jl::I3Connection::connect()?;
        let r = Self {
            i3connection: Arc::new(Mutex::new(i3connection)),
            config: Arc::new(Mutex::new(config)),
        };
        Ok(r)
    }

    pub fn with_i3connection<CB, R>(&self, cb: CB) -> R
    where
        CB: Fn(&mut i3ipc_jl::I3Connection) -> R,
    {
        let mut connection = self.i3connection.lock().unwrap();
        cb(&mut connection)
    }

    pub fn run_i3_command(&self, command: &str) -> Result<()> {
        self.with_i3connection(|connection| {
            slog_scope::info!("Running i3 command: {}", command);
            connection.run_command(command)?;
            Ok(())
        })
    }

    pub fn with_config<CB, R>(&self, cb: CB) -> R
    where
        CB: Fn(&crate::config::Config) -> R,
    {
        let config = self.config.lock().unwrap();
        cb(&config)
    }

    // Workspace history methods - delegate to last_workspaces module

    /// Updates the last workspace for current_group and returns the last workspace for target_group.
    /// This is more efficient than separate set + get calls.
    pub fn update_and_get_last_workspace(
        &self,
        current_group: Option<&str>,
        current_workspace: i64,
        target_group: Option<&str>,
    ) -> Option<i64> {
        crate::last_workspaces::update_and_get(current_group, current_workspace, target_group)
    }

    pub fn get_last_workspace(&self, group: Option<&str>) -> Option<i64> {
        crate::last_workspaces::get_last_workspace(group)
    }
}
