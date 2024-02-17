use std::fmt;

pub enum Scope{
    Widget,
    Connector,
    Process
}

pub type PluginScope = Vec<Scope>;

pub struct Plugin {
    name: String,
    uuid: u64,
    scope: PluginScope
}
