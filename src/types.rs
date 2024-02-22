use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum Scope {
    #[default]
    Widget,
    Connector,
    Process,
}

pub type PluginScope = Vec<Scope>;


