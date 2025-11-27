use anyhow::Result;

mod event_action {
    use std::collections::HashMap;

    use crate::event_processor::config::event_action;
    use anyhow::Result;

    fn make_container_env_map(
        prefix: &str,
        container: &i3ipc_jl::reply::Node,
    ) -> HashMap<String, String> {
        let mut r = HashMap::new();

        r.insert(
            format!("I3IM_{prefix}CONTAINER_ID"),
            format!("{}", container.id),
        );
        r.insert(
            format!("I3IM_{prefix}CONTAINER_NAME"),
            container.name.clone().unwrap_or_default(),
        );
        r.insert(
            format!("I3IM_{prefix}CONTAINER_NODE_TYPE"),
            crate::event_processor::config::NodeType::from(&container.nodetype).to_string(),
        );
        r.insert(
            format!("I3IM_{prefix}CONTAINER_BORDER_WIDTH"),
            format!("{}", container.current_border_width),
        );
        r.insert(
            format!("I3IM_{prefix}CONTAINER_LAYOUT"),
            crate::event_processor::config::NodeLayout::from(&container.layout).to_string(),
        );
        r.insert(
            format!("I3IM_{prefix}CONTAINER_PERCENT"),
            container
                .percent
                .map(|v| format!("{v}"))
                .unwrap_or_default(),
        );
        r.insert(
            format!("I3IM_{prefix}CONTAINER_WINDOW_ID"),
            container.window.map(|v| format!("{v}")).unwrap_or_default(),
        );

        r.insert(
            format!("I3IM_{prefix}CONTAINER_WINDOW_TITLE"),
            container
                .window_properties
                .as_ref()
                .and_then(|window_properties| {
                    window_properties
                        .get(&i3ipc_jl::reply::WindowProperty::Title)
                        .cloned()
                })
                .unwrap_or_default(),
        );
        r.insert(
            format!("I3IM_{prefix}CONTAINER_WINDOW_CLASS"),
            container
                .window_properties
                .as_ref()
                .and_then(|window_properties| {
                    window_properties
                        .get(&i3ipc_jl::reply::WindowProperty::Class)
                        .cloned()
                })
                .unwrap_or_default(),
        );
        r.insert(
            format!("I3IM_{prefix}CONTAINER_WINDOW_INSTANCE"),
            container
                .window_properties
                .as_ref()
                .and_then(|window_properties| {
                    window_properties
                        .get(&i3ipc_jl::reply::WindowProperty::Instance)
                        .cloned()
                })
                .unwrap_or_default(),
        );
        r.insert(
            format!("I3IM_{prefix}CONTAINER_WINDOW_ROLE"),
            container
                .window_properties
                .as_ref()
                .and_then(|window_properties| {
                    window_properties
                        .get(&i3ipc_jl::reply::WindowProperty::WindowRole)
                        .cloned()
                })
                .unwrap_or_default(),
        );
        r.insert(
            format!("I3IM_{prefix}CONTAINER_WINDOW_TRANSIENT_FOR"),
            container
                .window_properties
                .as_ref()
                .and_then(|window_properties| {
                    window_properties
                        .get(&i3ipc_jl::reply::WindowProperty::TransientFor)
                        .cloned()
                })
                .unwrap_or_default(),
        );
        r.insert(
            format!("I3IM_{prefix}CONTAINER_WINDOW_MACHINE"),
            container
                .window_properties
                .as_ref()
                .and_then(|window_properties| {
                    window_properties
                        .get(&i3ipc_jl::reply::WindowProperty::Machine)
                        .cloned()
                })
                .unwrap_or_default(),
        );
        r.insert(
            format!("I3IM_{prefix}CONTAINER_WINDOW_MARK"),
            container
                .window_properties
                .as_ref()
                .and_then(|window_properties| {
                    window_properties
                        .get(&i3ipc_jl::reply::WindowProperty::Mark)
                        .cloned()
                })
                .unwrap_or_default(),
        );

        if container.urgent {
            r.insert(format!("I3IM_{prefix}CONTAINER_URGENT"), "1".to_owned());
        }
        if container.focused {
            r.insert(format!("I3IM_{prefix}CONTAINER_FOCUSED"), "1".to_owned());
        }

        r
    }

    fn make_env_map(event: &i3ipc_jl::event::Event) -> HashMap<String, String> {
        let mut r = HashMap::new();
        r.insert("I3IM_EVENT".to_owned(), "1".to_owned());

        use i3ipc_jl::event::Event;
        match event {
            Event::WindowEvent(e) => {
                use crate::event_processor::config::window;
                r.insert(
                    "I3IM_WINDOW_EVENT_TYPE".to_owned(),
                    window::WindowEventType::from(&e.change).to_string(),
                );
                r.extend(make_container_env_map("", &e.container))
            }
            Event::WorkspaceEvent(e) => {
                use crate::event_processor::config::workspace;
                r.insert(
                    "I3IM_WORKSPACE_EVENT_TYPE".to_owned(),
                    workspace::WorkspaceEventType::from(&e.change).to_string(),
                );
                if let Some(old_container) = &e.old {
                    r.extend(make_container_env_map("OLD_", old_container))
                }
                if let Some(current_container) = &e.current {
                    r.extend(make_container_env_map("CURRENT_", current_container))
                }
            }
            other => {
                slog_scope::warn!("Got unexpected event: {:?}", other);
            }
        }

        r
    }

    pub fn run_action(
        event: &i3ipc_jl::event::Event,
        action: &event_action::EventAction,
    ) -> Result<()> {
        match action {
            event_action::EventAction::ShellCommand(command) => {
                slog_scope::debug!("Running shell command: {:?}", command.command);
                std::process::Command::new("sh")
                    .envs(&command.extra_env)
                    .envs(make_env_map(event))
                    .arg("-c")
                    .arg(&command.command)
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .spawn()?;
            }
        }

        Ok(())
    }
}

mod window_handler {
    use crate::event_processor::config::window::WindowEventConditionWrapper;
    use anyhow::Result;

    fn check_condition_list(
        condition_list: &[WindowEventConditionWrapper],
        event: &i3ipc_jl::event::WindowEventInfo,
    ) -> bool {
        for condition in condition_list {
            if !condition.0.matches(event) {
                return false;
            }
        }

        slog_scope::debug!("All conditions matched");
        true
    }

    pub fn handle_event(
        state: &crate::state::State,
        event: &i3ipc_jl::event::Event,
        window_event: &i3ipc_jl::event::WindowEventInfo,
    ) -> Result<()> {
        state.with_config(|config| {
            for handler in &config.window_event_handlers {
                if check_condition_list(&handler.condition_list, window_event) {
                    super::event_action::run_action(event, &handler.action)?
                }
            }
            Ok(())
        })
    }
}

mod workspace_handler {
    use crate::event_processor::config::workspace::WorkspaceEventConditionWrapper;
    use anyhow::Result;

    fn check_condition_list(
        condition_list: &[WorkspaceEventConditionWrapper],
        event: &i3ipc_jl::event::WorkspaceEventInfo,
    ) -> bool {
        for condition in condition_list {
            if !condition.0.matches(event) {
                return false;
            }
        }

        true
    }

    pub fn handle_event(
        state: &crate::state::State,
        event: &i3ipc_jl::event::Event,
        workspace_event: &i3ipc_jl::event::WorkspaceEventInfo,
    ) -> Result<()> {
        state.with_config(|config| {
            for handler in &config.workspace_event_handlers {
                if check_condition_list(&handler.condition_list, workspace_event) {
                    super::event_action::run_action(event, &handler.action)?
                }
            }
            Ok(())
        })
    }
}

pub fn handle_event(state: &crate::state::State, event: &i3ipc_jl::event::Event) -> Result<()> {
    use i3ipc_jl::event::Event;
    match event {
        Event::WindowEvent(window_event) => {
            slog_scope::debug!("Window event: {:?}", window_event);
            window_handler::handle_event(state, event, window_event)
        }
        Event::WorkspaceEvent(workspace_event) => {
            slog_scope::debug!("Workspace event: {:?}", workspace_event);
            workspace_handler::handle_event(state, event, workspace_event)
        }
        other => {
            slog_scope::warn!("Got unexpected event: {:?}", other);
            Ok(())
        }
    }
}
