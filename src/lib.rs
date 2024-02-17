pub mod types;

pub struct Plugin {
    name: String,
    uuid: u64,
    scope: types::PluginScope
}