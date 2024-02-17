pub mod types;

#[derive(Debug, Clone, Default)]
pub struct Plugin {
    name: String,
    uuid: u64,
    scope: types::PluginScope
}