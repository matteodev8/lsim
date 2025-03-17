use bevy::prelude::*;

pub mod and;
pub mod io;
pub mod not;
pub mod or;

pub trait Node: Send + Sync {
    fn simulate(&mut self) -> bool;
}
type NodeComponent = Box<dyn Node>;

#[derive(Component)]
pub struct VoidNode;

impl Node for VoidNode {
    fn simulate(&mut self) -> bool {
        false
    }
}
