use super::{Node, NodeComponent};
use bevy::prelude::*;

#[derive(Component)]
pub struct AndGate {
    a_gate: NodeComponent,
    b_gate: NodeComponent,
}

impl AndGate {
    pub fn new(a_gate: impl Into<NodeComponent>, b_gate: impl Into<NodeComponent>) -> Self {
        AndGate {
            a_gate: a_gate.into(),
            b_gate: b_gate.into(),
        }
    }
}

impl Node for AndGate {
    fn simulate(&self) -> bool {
        self.a_gate.simulate() && self.b_gate.simulate()
    }
}
