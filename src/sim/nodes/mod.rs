use bevy::prelude::*;

pub mod and;
pub mod io;
pub mod not;
pub mod or;

pub trait Node: Send + Sync {
    fn simulate(&self) -> bool;
}
pub type NodeComponent = Box<dyn Node>;

#[derive(Component)]
pub struct VoidNode;

impl Node for VoidNode {
    fn simulate(&self) -> bool {
        false
    }
}

impl From<Option<NodeComponent>> for NodeComponent {
    fn from(value: Option<NodeComponent>) -> Self {
        match value {
            Some(v) => v,
            None => Box::new(VoidNode),
        }
    }
}
