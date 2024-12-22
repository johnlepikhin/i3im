use anyhow::Result;
use clap::Args;
use std::sync::Mutex;

#[derive(Args, Clone)]
pub struct ListenerCmd {}

impl ListenerCmd {
    pub fn run(&self, state: crate::state::State) -> Result<()> {
        Listener::new(state)?.run()
    }
}

struct Listener {
    i3_listener: Mutex<i3ipc::I3EventListener>,
    state: crate::state::State,
}

impl Listener {
    pub fn new(state: crate::state::State) -> Result<Self> {
        let i3_listener = i3ipc::I3EventListener::connect()?;
        Ok(Self {
            i3_listener: Mutex::new(i3_listener),
            state,
        })
    }

    fn handle_event(&self, event: &i3ipc::event::Event) -> Result<()> {
        crate::event_processor::processor::handle_event(&self.state, event)
    }

    pub fn run(&self) -> Result<()> {
        self.i3_listener
            .lock()
            .unwrap()
            .subscribe(&[i3ipc::Subscription::Window, i3ipc::Subscription::Workspace])?;

        for event in self.i3_listener.lock().unwrap().listen() {
            match &event {
                Ok(event) => {
                    self.handle_event(event)?;
                }
                Err(err) => {
                    slog_scope::error!("{}", err);
                }
            }
        }

        Ok(())
    }
}
