use super::{Node, NodeComponent, VoidNode};
use bevy::prelude::*;

#[derive(Component)]
pub struct NotGate {
    a_gate: NodeComponent,
    state: bool,
}

impl NotGate {
    pub fn new(a_gate: Option<NodeComponent>) -> Self {
        NotGate {
            a_gate: match a_gate {
                Some(a) => a,
                None => Box::new(VoidNode),
            },
            state: true,
        }
    }
}

impl Node for NotGate {
    fn simulate(&mut self) -> bool {
        self.state = !self.a_gate.simulate();
        self.state
    }
}
