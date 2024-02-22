pub mod types;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Plugin {
    pub name: String,
    pub uuid: u64,
    pub scope: types::PluginScope
}
