use std::sync::{Arc, Mutex};

use anyhow::Result;

pub struct State {
    i3connection: Arc<Mutex<i3ipc::I3Connection>>,
    config: Arc<Mutex<crate::config::Config>>,
}

impl State {
    pub fn new(config: crate::config::Config) -> Result<Self> {
        let i3connection = i3ipc::I3Connection::connect()?;
        let r = Self {
            i3connection: Arc::new(Mutex::new(i3connection)),
            config: Arc::new(Mutex::new(config)),
        };
        Ok(r)
    }

    pub fn with_i3connection<CB, R>(&self, cb: CB) -> R
    where
        CB: Fn(&mut i3ipc::I3Connection) -> R,
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
}
