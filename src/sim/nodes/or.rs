use super::{Node, NodeComponent};
use bevy::prelude::*;

#[derive(Component)]
pub struct OrGate {
    a_gate: NodeComponent,
    b_gate: NodeComponent,
}

impl OrGate {
    pub fn new(a_gate: impl Into<NodeComponent>, b_gate: impl Into<NodeComponent>) -> Self {
        OrGate {
            a_gate: a_gate.into(),
            b_gate: b_gate.into(),
        }
    }
}

impl Node for OrGate {
    fn simulate(&self) -> bool {
        self.a_gate.simulate() || self.b_gate.simulate()
    }
}
