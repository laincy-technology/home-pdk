use std::fmt;

pub enum Scope{
    Widget,
    Connector,
    Process
}

pub type PluginScope = Vec<Scope>;

