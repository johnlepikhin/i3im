use serde::{Deserialize, Serialize};
use structdoc::StructDoc;

fn get_window_property(
    window: &i3ipc_jl::reply::Node,
    property: i3ipc_jl::reply::WindowProperty,
) -> Option<&str> {
    window
        .window_properties
        .as_ref()
        .and_then(|window_properties| window_properties.get(&property).map(|v| v.as_str()))
}

fn get_opt_window_property(
    window: &Option<i3ipc_jl::reply::Node>,
    property: i3ipc_jl::reply::WindowProperty,
) -> Option<&str> {
    window
        .as_ref()
        .and_then(|window| get_window_property(window, property))
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ConfigRegex {
    /// The regular expression
    #[serde(with = "serde_regex")]
    pub regex: regex::Regex,
}

impl StructDoc for ConfigRegex {
    fn document() -> structdoc::Documentation {
        structdoc::Documentation::leaf("A regular expression")
    }
}

#[derive(Clone, Serialize, Deserialize, StructDoc)]
/// A string match condition
pub enum StringMatch {
    /// Exact match
    Eq(String),
    /// Case-insensitive exact match
    EqIgnoreCase(String),
    /// Regular expression match
    Regex(ConfigRegex),
}

impl StringMatch {
    pub fn matches(&self, s: &str) -> bool {
        match self {
            StringMatch::Eq(v) => s == v,
            StringMatch::EqIgnoreCase(v) => s.eq_ignore_ascii_case(v),
            StringMatch::Regex(r) => r.regex.is_match(s),
        }
    }

    pub fn matches_option(&self, s: Option<&str>) -> bool {
        match s {
            Some(s) => self.matches(s),
            None => false,
        }
    }
}

/// Type of container
#[derive(Clone, Serialize, Deserialize, StructDoc, PartialEq, Eq, Debug)]
pub enum NodeType {
    Root,
    Output,
    Con,
    FloatingCon,
    Workspace,
    DockArea,
    Unknown,
}

impl std::fmt::Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<&i3ipc_jl::reply::NodeType> for NodeType {
    fn from(node: &i3ipc_jl::reply::NodeType) -> Self {
        use i3ipc_jl::reply::NodeType;
        match node {
            NodeType::Root => Self::Root,
            NodeType::Output => Self::Output,
            NodeType::Con => Self::Con,
            NodeType::FloatingCon => Self::FloatingCon,
            NodeType::Workspace => Self::Workspace,
            NodeType::DockArea => Self::DockArea,
            NodeType::Unknown => Self::Unknown,
        }
    }
}

impl NodeType {
    pub fn matches(&self, node: &i3ipc_jl::reply::Node) -> bool {
        let other = NodeType::from(&node.nodetype);
        self == &other
    }
}

/// Container layout
#[derive(Clone, Serialize, Deserialize, StructDoc, PartialEq, Eq, Debug)]
pub enum NodeLayout {
    SplitH,
    SplitV,
    Stacked,
    Tabbed,
    DockArea,
    Output,
    Unknown,
}

impl NodeLayout {
    pub fn matches(&self, node: &i3ipc_jl::reply::Node) -> bool {
        let other = NodeLayout::from(&node.layout);
        self == &other
    }
}

impl std::fmt::Display for NodeLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<&i3ipc_jl::reply::NodeLayout> for NodeLayout {
    fn from(layout: &i3ipc_jl::reply::NodeLayout) -> Self {
        use i3ipc_jl::reply::NodeLayout;
        match layout {
            NodeLayout::SplitH => Self::SplitH,
            NodeLayout::SplitV => Self::SplitV,
            NodeLayout::Stacked => Self::Stacked,
            NodeLayout::Tabbed => Self::Tabbed,
            NodeLayout::DockArea => Self::DockArea,
            NodeLayout::Output => Self::Output,
            NodeLayout::Unknown => Self::Unknown,
        }
    }
}

/// Floating state of container
#[derive(Clone, Serialize, Deserialize, StructDoc, PartialEq, Eq, Debug)]
pub enum NodeFloating {
    AutoOff,
    AutoOn,
    UserOff,
    UserOn,
    Unknown,
}

impl NodeFloating {
    pub fn matches(&self, node: &i3ipc_jl::reply::Node) -> bool {
        let other = NodeFloating::from(&node.floating);
        self == &other
    }
}

impl std::fmt::Display for NodeFloating {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<&i3ipc_jl::reply::NodeFloating> for NodeFloating {
    fn from(v: &i3ipc_jl::reply::NodeFloating) -> Self {
        use i3ipc_jl::reply::NodeFloating;
        match v {
            NodeFloating::AutoOff => Self::AutoOff,
            NodeFloating::AutoOn => Self::AutoOn,
            NodeFloating::UserOff => Self::UserOff,
            NodeFloating::UserOn => Self::UserOn,
            NodeFloating::Unknown => Self::Unknown,
        }
    }
}

/// Whether this container is in fullscreen state or not
#[derive(Clone, Serialize, Deserialize, StructDoc, PartialEq, Eq, Debug)]
pub enum NodeFullscreenMode {
    None,
    Fullscreen,
    Global,
    Unknown,
}

impl NodeFullscreenMode {
    pub fn matches(&self, node: &i3ipc_jl::reply::Node) -> bool {
        let other = NodeFullscreenMode::from(&node.fullscreen_mode);
        self == &other
    }
}

impl std::fmt::Display for NodeFullscreenMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<&i3ipc_jl::reply::NodeFullScreenMode> for NodeFullscreenMode {
    fn from(v: &i3ipc_jl::reply::NodeFullScreenMode) -> Self {
        use i3ipc_jl::reply::NodeFullScreenMode;
        match v {
            NodeFullScreenMode::None => Self::None,
            NodeFullScreenMode::Fullscreen => Self::Fullscreen,
            NodeFullScreenMode::Global => Self::Global,
            NodeFullScreenMode::Unknown => Self::Unknown,
        }
    }
}

pub mod event_action {
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};
    use structdoc::StructDoc;

    #[derive(Clone, Serialize, Deserialize, StructDoc)]
    pub struct ShellCommand {
        pub command: String,
        #[serde(default)]
        pub extra_env: HashMap<String, String>,
    }

    #[derive(Clone, Serialize, Deserialize, StructDoc)]
    pub enum EventAction {
        ShellCommand(ShellCommand),
    }
}

pub mod window {
    use super::{NodeFloating, NodeFullscreenMode, NodeLayout, NodeType, StringMatch};
    use serde::{Deserialize, Serialize};
    use structdoc::StructDoc;

    /// Window event type
    #[derive(Clone, Copy, Serialize, Deserialize, StructDoc, PartialEq, Eq, Debug)]
    pub enum WindowEventType {
        New,
        Close,
        Focus,
        Title,
        FullscreenMode,
        Move,
        Floating,
        Urgent,
        Mark,
        Unknown,
    }

    impl WindowEventType {
        pub fn matches(&self, event: &i3ipc_jl::event::inner::WindowChange) -> bool {
            let other = WindowEventType::from(event);
            self == &other
        }
    }

    impl std::fmt::Display for WindowEventType {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    impl From<&i3ipc_jl::event::inner::WindowChange> for WindowEventType {
        fn from(event: &i3ipc_jl::event::inner::WindowChange) -> Self {
            use i3ipc_jl::event::inner::WindowChange;
            match event {
                WindowChange::New => Self::New,
                WindowChange::Close => Self::Close,
                WindowChange::Focus => Self::Focus,
                WindowChange::Title => Self::Title,
                WindowChange::FullscreenMode => Self::FullscreenMode,
                WindowChange::Move => Self::Move,
                WindowChange::Floating => Self::Floating,
                WindowChange::Urgent => Self::Urgent,
                WindowChange::Mark => Self::Mark,
                WindowChange::Unknown => Self::Unknown,
            }
        }
    }

    /// Window event condition
    #[derive(Clone, Serialize, Deserialize, StructDoc)]
    pub enum WindowEventCondition {
        /// Window event type
        EventType(#[serde(with = "serde_yaml::with::singleton_map")] WindowEventType),
        /// Container name
        Name(#[serde(with = "serde_yaml::with::singleton_map")] StringMatch),
        /// Container type
        NodeType(#[serde(with = "serde_yaml::with::singleton_map")] NodeType),
        /// Container layout
        NodeLayout(#[serde(with = "serde_yaml::with::singleton_map")] NodeLayout),
        /// Container fullscreen mode
        NodeFullscreenMode(#[serde(with = "serde_yaml::with::singleton_map")] NodeFullscreenMode),
        /// Container floating status
        NodeFloating(#[serde(with = "serde_yaml::with::singleton_map")] NodeFloating),
        /// Whether this container (window, split container, floating container or workspace) has the urgency hint set,
        /// directly or indirectly. All parent containers up until the workspace container will be marked urgent if they
        /// have at least one urgent child.
        Urgent(bool),
        /// Whether this container is currently focused.
        Focused(bool),
        /// Whether this window is "sticky". If it is also floating, this window will be present on all workspaces on
        /// the same output.
        Sticky(bool),
        /// Window title
        Title(#[serde(with = "serde_yaml::with::singleton_map")] StringMatch),
        /// Window instance
        Instance(#[serde(with = "serde_yaml::with::singleton_map")] StringMatch),
        /// Window class name
        Class(#[serde(with = "serde_yaml::with::singleton_map")] StringMatch),
        /// Window role
        WindowRole(#[serde(with = "serde_yaml::with::singleton_map")] StringMatch),
        /// Window transient for
        TransientFor(#[serde(with = "serde_yaml::with::singleton_map")] StringMatch),
        /// Window machine
        Machine(#[serde(with = "serde_yaml::with::singleton_map")] StringMatch),
        /// Window mark
        Mark(#[serde(with = "serde_yaml::with::singleton_map")] StringMatch),
    }

    impl WindowEventCondition {
        pub fn matches(&self, event: &i3ipc_jl::event::WindowEventInfo) -> bool {
            match self {
                Self::EventType(v) => v.matches(&event.change),
                Self::Name(v) => v.matches_option(event.container.name.as_deref()),
                Self::NodeType(v) => v.matches(&event.container),
                Self::NodeLayout(v) => v.matches(&event.container),
                Self::NodeFullscreenMode(v) => v.matches(&event.container),
                Self::NodeFloating(v) => v.matches(&event.container),
                Self::Urgent(v) => *v == event.container.urgent,
                Self::Focused(v) => *v == event.container.focused,
                Self::Sticky(v) => *v == event.container.sticky,
                Self::Title(v) => v.matches_option(super::get_window_property(
                    &event.container,
                    i3ipc_jl::reply::WindowProperty::Title,
                )),
                Self::Instance(v) => v.matches_option(super::get_window_property(
                    &event.container,
                    i3ipc_jl::reply::WindowProperty::Instance,
                )),
                Self::Class(v) => v.matches_option(super::get_window_property(
                    &event.container,
                    i3ipc_jl::reply::WindowProperty::Class,
                )),
                Self::WindowRole(v) => v.matches_option(super::get_window_property(
                    &event.container,
                    i3ipc_jl::reply::WindowProperty::WindowRole,
                )),
                Self::TransientFor(v) => v.matches_option(super::get_window_property(
                    &event.container,
                    i3ipc_jl::reply::WindowProperty::TransientFor,
                )),
                Self::Machine(v) => v.matches_option(super::get_window_property(
                    &event.container,
                    i3ipc_jl::reply::WindowProperty::Machine,
                )),
                Self::Mark(v) => v.matches_option(super::get_window_property(
                    &event.container,
                    i3ipc_jl::reply::WindowProperty::Mark,
                )),
            }
        }
    }

    #[derive(Deserialize, Serialize, Clone, StructDoc)]
    #[serde(transparent)]
    pub struct WindowEventConditionWrapper(
        #[serde(with = "serde_yaml::with::singleton_map")] pub WindowEventCondition,
    );

    #[derive(Clone, Serialize, Deserialize, StructDoc)]
    pub struct WindowEventHandler {
        pub condition_list: Vec<WindowEventConditionWrapper>,
        #[serde(with = "serde_yaml::with::singleton_map")]
        pub action: super::event_action::EventAction,
    }
}

pub mod workspace {
    use super::{NodeFloating, NodeFullscreenMode, NodeLayout, NodeType, StringMatch};
    use serde::{Deserialize, Serialize};
    use structdoc::StructDoc;

    #[derive(Clone, Copy, Serialize, Deserialize, StructDoc, Debug, PartialEq, Eq)]
    pub enum WorkspaceEventType {
        Focus,
        Init,
        Empty,
        Urgent,
        Rename,
        Reload,
        Restored,
        Move,
        Unknown,
    }

    impl WorkspaceEventType {
        pub fn matches(&self, change: &i3ipc_jl::event::inner::WorkspaceChange) -> bool {
            let other = Self::from(change);
            other == *self
        }
    }

    impl std::fmt::Display for WorkspaceEventType {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
    impl From<&i3ipc_jl::event::inner::WorkspaceChange> for WorkspaceEventType {
        fn from(event: &i3ipc_jl::event::inner::WorkspaceChange) -> Self {
            use i3ipc_jl::event::inner::WorkspaceChange;
            match event {
                WorkspaceChange::Focus => Self::Focus,
                WorkspaceChange::Init => Self::Init,
                WorkspaceChange::Empty => Self::Empty,
                WorkspaceChange::Urgent => Self::Urgent,
                WorkspaceChange::Rename => Self::Rename,
                WorkspaceChange::Reload => Self::Reload,
                WorkspaceChange::Restored => Self::Restored,
                WorkspaceChange::Move => Self::Move,
                WorkspaceChange::Unknown => Self::Unknown,
            }
        }
    }

    #[derive(Clone, Serialize, Deserialize, StructDoc)]
    pub enum WorkspaceEventCondition {
        EventType(#[serde(with = "serde_yaml::with::singleton_map")] WorkspaceEventType),
        OldName(#[serde(with = "serde_yaml::with::singleton_map")] StringMatch),
        OldNodeType(#[serde(with = "serde_yaml::with::singleton_map")] NodeType),
        OldNodeLayout(#[serde(with = "serde_yaml::with::singleton_map")] NodeLayout),
        OldNodeFullscreenMode(
            #[serde(with = "serde_yaml::with::singleton_map")] NodeFullscreenMode,
        ),
        OldNodeFloating(#[serde(with = "serde_yaml::with::singleton_map")] NodeFloating),
        OldUrgent(bool),
        OldFocused(bool),
        OldSticky(bool),
        OldTitle(#[serde(with = "serde_yaml::with::singleton_map")] StringMatch),
        OldInstance(#[serde(with = "serde_yaml::with::singleton_map")] StringMatch),
        OldClass(#[serde(with = "serde_yaml::with::singleton_map")] StringMatch),
        OldWindowRole(#[serde(with = "serde_yaml::with::singleton_map")] StringMatch),
        OldTransientFor(#[serde(with = "serde_yaml::with::singleton_map")] StringMatch),
        OldMachine(#[serde(with = "serde_yaml::with::singleton_map")] StringMatch),
        OldMark(#[serde(with = "serde_yaml::with::singleton_map")] StringMatch),
        CurrentName(#[serde(with = "serde_yaml::with::singleton_map")] StringMatch),
        CurrentNodeType(#[serde(with = "serde_yaml::with::singleton_map")] NodeType),
        CurrentNodeLayout(#[serde(with = "serde_yaml::with::singleton_map")] NodeLayout),
        CurrentNodeFullscreenMode(
            #[serde(with = "serde_yaml::with::singleton_map")] NodeFullscreenMode,
        ),
        CurrentNodeFloating(#[serde(with = "serde_yaml::with::singleton_map")] NodeFloating),
        CurrentUrgent(bool),
        CurrentFocused(bool),
        CurrentSticky(bool),
        CurrentTitle(#[serde(with = "serde_yaml::with::singleton_map")] StringMatch),
        CurrentInstance(#[serde(with = "serde_yaml::with::singleton_map")] StringMatch),
        CurrentClass(#[serde(with = "serde_yaml::with::singleton_map")] StringMatch),
        CurrentWindowRole(#[serde(with = "serde_yaml::with::singleton_map")] StringMatch),
        CurrentTransientFor(#[serde(with = "serde_yaml::with::singleton_map")] StringMatch),
        CurrentMachine(#[serde(with = "serde_yaml::with::singleton_map")] StringMatch),
        CurrentMark(#[serde(with = "serde_yaml::with::singleton_map")] StringMatch),
    }

    impl WorkspaceEventCondition {
        pub fn matches(&self, event: &i3ipc_jl::event::WorkspaceEventInfo) -> bool {
            match self {
                Self::EventType(v) => v.matches(&event.change),
                Self::OldName(v) => event
                    .old
                    .as_ref()
                    .map(|container| v.matches_option(container.name.as_deref()))
                    .unwrap_or_default(),
                Self::OldNodeType(v) => event
                    .old
                    .as_ref()
                    .map(|container| v.matches(container))
                    .unwrap_or_default(),
                Self::OldNodeLayout(v) => event
                    .old
                    .as_ref()
                    .map(|container| v.matches(container))
                    .unwrap_or_default(),
                Self::OldNodeFullscreenMode(v) => event
                    .old
                    .as_ref()
                    .map(|container| v.matches(container))
                    .unwrap_or_default(),
                Self::OldNodeFloating(v) => event
                    .old
                    .as_ref()
                    .map(|container| v.matches(container))
                    .unwrap_or_default(),
                Self::OldUrgent(v) => event
                    .old
                    .as_ref()
                    .map(|container| *v == container.urgent)
                    .unwrap_or_default(),
                Self::OldFocused(v) => event
                    .old
                    .as_ref()
                    .map(|container| *v == container.focused)
                    .unwrap_or_default(),
                Self::OldSticky(v) => event
                    .old
                    .as_ref()
                    .map(|container| *v == container.sticky)
                    .unwrap_or_default(),
                Self::OldTitle(v) => v.matches_option(super::get_opt_window_property(
                    &event.old,
                    i3ipc_jl::reply::WindowProperty::Title,
                )),
                Self::OldInstance(v) => v.matches_option(super::get_opt_window_property(
                    &event.old,
                    i3ipc_jl::reply::WindowProperty::Instance,
                )),
                Self::OldClass(v) => v.matches_option(super::get_opt_window_property(
                    &event.old,
                    i3ipc_jl::reply::WindowProperty::Class,
                )),
                Self::OldWindowRole(v) => v.matches_option(super::get_opt_window_property(
                    &event.old,
                    i3ipc_jl::reply::WindowProperty::WindowRole,
                )),
                Self::OldTransientFor(v) => v.matches_option(super::get_opt_window_property(
                    &event.old,
                    i3ipc_jl::reply::WindowProperty::TransientFor,
                )),
                Self::OldMachine(v) => v.matches_option(super::get_opt_window_property(
                    &event.old,
                    i3ipc_jl::reply::WindowProperty::Machine,
                )),
                Self::OldMark(v) => v.matches_option(super::get_opt_window_property(
                    &event.old,
                    i3ipc_jl::reply::WindowProperty::Mark,
                )),
                Self::CurrentName(v) => event
                    .current
                    .as_ref()
                    .map(|container| v.matches_option(container.name.as_deref()))
                    .unwrap_or_default(),
                Self::CurrentNodeType(v) => event
                    .current
                    .as_ref()
                    .map(|container| v.matches(container))
                    .unwrap_or_default(),
                Self::CurrentNodeLayout(v) => event
                    .current
                    .as_ref()
                    .map(|container| v.matches(container))
                    .unwrap_or_default(),
                Self::CurrentNodeFullscreenMode(v) => event
                    .current
                    .as_ref()
                    .map(|container| v.matches(container))
                    .unwrap_or_default(),
                Self::CurrentNodeFloating(v) => event
                    .current
                    .as_ref()
                    .map(|container| v.matches(container))
                    .unwrap_or_default(),
                Self::CurrentUrgent(v) => event
                    .current
                    .as_ref()
                    .map(|container| *v == container.urgent)
                    .unwrap_or_default(),
                Self::CurrentFocused(v) => event
                    .current
                    .as_ref()
                    .map(|container| *v == container.focused)
                    .unwrap_or_default(),
                Self::CurrentSticky(v) => event
                    .current
                    .as_ref()
                    .map(|container| *v == container.sticky)
                    .unwrap_or_default(),
                Self::CurrentTitle(v) => v.matches_option(super::get_opt_window_property(
                    &event.current,
                    i3ipc_jl::reply::WindowProperty::Title,
                )),
                Self::CurrentInstance(v) => v.matches_option(super::get_opt_window_property(
                    &event.current,
                    i3ipc_jl::reply::WindowProperty::Instance,
                )),
                Self::CurrentClass(v) => v.matches_option(super::get_opt_window_property(
                    &event.current,
                    i3ipc_jl::reply::WindowProperty::Class,
                )),
                Self::CurrentWindowRole(v) => v.matches_option(super::get_opt_window_property(
                    &event.current,
                    i3ipc_jl::reply::WindowProperty::WindowRole,
                )),
                Self::CurrentTransientFor(v) => v.matches_option(super::get_opt_window_property(
                    &event.current,
                    i3ipc_jl::reply::WindowProperty::TransientFor,
                )),
                Self::CurrentMachine(v) => v.matches_option(super::get_opt_window_property(
                    &event.current,
                    i3ipc_jl::reply::WindowProperty::Machine,
                )),
                Self::CurrentMark(v) => v.matches_option(super::get_opt_window_property(
                    &event.current,
                    i3ipc_jl::reply::WindowProperty::Mark,
                )),
            }
        }
    }

    #[derive(Clone, Serialize, Deserialize, StructDoc)]
    pub struct WorkspaceEventHandler {
        pub condition_list: Vec<WorkspaceEventCondition>,
        pub action: super::event_action::EventAction,
    }
}
