use anyhow::Result;
use clap::{Parser, Subcommand};
use slog::{o, Drain};
use structdoc::StructDoc;

mod commands;
mod config;
mod event_processor;
mod last_workspaces;
mod listener;
mod state;
mod workspace_group;

const CONFIG_DEFAULT_PATH: &str = "~/.config/i3im.yaml";

/// Config operations
#[derive(Subcommand)]
enum ConfigCommand {
    /// Dump parsed config file. Helps to find typos
    Dump,
    /// Print config file documentation
    Documentation,
    /// Generate default config
    Generate,
}

impl ConfigCommand {
    fn config_documentation() {
        println!(
            "Configuration file format. Default path is {}\n\n{}",
            CONFIG_DEFAULT_PATH,
            crate::config::Config::document()
        )
    }

    fn config_generate() {
        println!(
            "{}",
            serde_yaml::to_string(&crate::config::Config::default()).unwrap()
        );
    }

    fn config_dump(config_path: &Option<String>) {
        let config_path = config_path
            .clone()
            .unwrap_or(shellexpand::tilde(CONFIG_DEFAULT_PATH).to_string());
        let config = config::Config::read(&config_path).expect("Failed to read config");
        println!("{}", serde_yaml::to_string(&config).unwrap());
    }

    pub fn run(&self, config_path: &Option<String>) {
        match self {
            ConfigCommand::Dump => Self::config_dump(config_path),
            ConfigCommand::Documentation => Self::config_documentation(),
            ConfigCommand::Generate => Self::config_generate(),
        }
    }
}

/// Command line interface for i3im
#[derive(Subcommand)]
enum CommandLine {
    /// Config operations
    #[command(subcommand)]
    Config(ConfigCommand),
    /// Focus operations
    #[command(subcommand)]
    Focus(crate::commands::Focus),
    /// List operations
    #[command(subcommand)]
    List(crate::commands::List),
    /// Rename operations
    #[command(subcommand)]
    Rename(crate::commands::Rename),
    /// Move operations
    #[command(subcommand)]
    Move(crate::commands::Move),
    /// Run listener
    Listen(crate::listener::ListenerCmd),
}

/// Example of simple cli program
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Application {
    /// Path to configuration file, default is ~/.config/i3im.yaml
    #[clap(short, long)]
    config_path: Option<String>,
    /// Subcommand
    #[clap(subcommand)]
    command: CommandLine,
}

impl Application {
    fn init_syslog_logger(log_level: slog::Level) -> Result<()> {
        let logger = slog_syslog_jl::SyslogBuilder::new()
            .facility(slog_syslog_jl::Facility::LOG_USER)
            .level(log_level)
            .unix("/dev/log")
            .start()?;

        let logger = slog::Logger::root(logger.fuse(), o!());
        slog_scope::set_global_logger(logger).cancel_reset();
        Ok(())
    }

    fn init_env_logger() -> Result<()> {
        let drain =
            slog_term::CompactFormat::new(slog_term::TermDecorator::new().stderr().build()).build();
        // let drain = new(drain);
        let drain = std::sync::Mutex::new(drain.fuse());
        let logger = slog::Logger::root(drain.fuse(), o!());
        slog_scope::set_global_logger(logger).cancel_reset();
        Ok(())
    }

    fn init_logger(&self, config: &config::Config) -> Result<()> {
        if std::env::var("RUST_LOG").is_ok() {
            Self::init_env_logger()?
        } else {
            Self::init_syslog_logger(config.log_level.into())?
        }
        slog_stdlog::init()?;

        Ok(())
    }

    fn init_config(&self) -> Result<crate::config::Config> {
        let config_path = self
            .config_path
            .clone()
            .unwrap_or(shellexpand::tilde(CONFIG_DEFAULT_PATH).to_string());
        let config = config::Config::read(&config_path)?;
        self.init_logger(&config)?;
        Ok(config)
    }

    fn init_state(&self) -> Result<crate::state::State> {
        let config = self.init_config()?;
        let state = crate::state::State::new(config)?;
        Ok(state)
    }

    fn run_command(&self) -> Result<()> {
        match &self.command {
            CommandLine::Config(cmd) => {
                cmd.run(&self.config_path);
                Ok(())
            }
            CommandLine::Focus(cmd) => {
                let state = self.init_state()?;
                cmd.run(state)
            }
            CommandLine::List(cmd) => {
                let state = self.init_state()?;
                cmd.run(state)
            }
            CommandLine::Rename(cmd) => {
                let state = self.init_state()?;
                cmd.run(state)
            }
            CommandLine::Move(cmd) => {
                let state = self.init_state()?;
                cmd.run(state)
            }
            CommandLine::Listen(listener) => {
                let state = self.init_state()?;
                listener.run(state)
            }
        }
    }

    pub fn run(&self) {
        self.run_command().expect("Run");
    }
}

fn main() {
    Application::parse().run();
}
