use super::{Node, NodeComponent, VoidNode};
use bevy::prelude::*;

#[derive(Component)]
pub struct OrGate {
    a_gate: NodeComponent,
    b_gate: NodeComponent,
    state: bool,
}

impl OrGate {
    pub fn new(a_gate: Option<NodeComponent>, b_gate: Option<NodeComponent>) -> Self {
        OrGate {
            a_gate: match a_gate {
                Some(a) => a,
                None => Box::new(VoidNode),
            },
            b_gate: match b_gate {
                Some(b) => b,
                None => Box::new(VoidNode),
            },
            state: false,
        }
    }
}

impl Node for OrGate {
    fn simulate(&mut self) -> bool {
        self.state = self.a_gate.simulate() || self.b_gate.simulate();
        self.state
    }
}
