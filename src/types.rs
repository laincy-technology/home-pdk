
#[derive(Debug, Clone, Default)]
pub enum Scope{
    #[default]
    Widget,
    Connector,
    Process
}

pub type PluginScope = Vec<Scope>;

