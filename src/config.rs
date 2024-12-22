use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use structdoc::StructDoc;

#[derive(Clone, Copy, Serialize, Deserialize, StructDoc, Default)]
pub enum LogLevel {
    Critical,
    Error,
    Warning,
    #[default]
    Info,
    Debug,
    Trace,
}

impl From<LogLevel> for slog::Level {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Critical => slog::Level::Critical,
            LogLevel::Error => slog::Level::Error,
            LogLevel::Warning => slog::Level::Warning,
            LogLevel::Info => slog::Level::Info,
            LogLevel::Debug => slog::Level::Debug,
            LogLevel::Trace => slog::Level::Trace,
        }
    }
}

#[derive(Serialize, Deserialize, StructDoc, Default)]
pub struct Config {
    /// Max log level for syslog mode
    pub log_level: LogLevel,
    /// Window events handlers
    #[serde(default)]
    pub window_event_handlers: Vec<crate::event_processor::config::window::WindowEventHandler>,
    /// Workspace events handlers
    #[serde(default)]
    pub workspace_event_handlers:
        Vec<crate::event_processor::config::workspace::WorkspaceEventHandler>,
}

impl Config {
    fn validate(&self) -> Result<()> {
        Ok(())
    }

    pub fn read(file: &str) -> Result<Self> {
        let config = std::fs::read_to_string(file)
            .with_context(|| format!("Failed to load config file {:?}", file))?;
        let config: Self = serde_yaml::from_str(&config)
            .with_context(|| format!("Failed to parse config file {:?}", file))?;

        config.validate()?;
        Ok(config)
    }
}
