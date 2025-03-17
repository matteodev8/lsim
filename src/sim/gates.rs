use bevy::prelude::*;

/// TODO: Make it more universal, currently it's meant for gates only but should also work for
/// input toggles and output lights
pub trait Gate: Send + Sync {
    fn simulate(&mut self) -> bool;
}
type GateComponent = Box<dyn Gate>;

pub struct Input {
    pub state: bool,
}

impl Gate for Input {
    fn simulate(&mut self) -> bool {
        self.state
    }
}

#[derive(Component)]
pub struct AndGate {
    a_gate: GateComponent,
    b_gate: GateComponent,
    state: bool,
}

impl AndGate {
    pub fn new(a_gate: GateComponent, b_gate: GateComponent) -> AndGate {
        AndGate {
            a_gate,
            b_gate,
            state: false,
        }
    }
}

impl Gate for AndGate {
    fn simulate(&mut self) -> bool {
        self.state = self.a_gate.simulate() && self.b_gate.simulate();
        self.state
    }
}

#[derive(Component)]
pub struct OrGate {
    a_gate: GateComponent,
    b_gate: GateComponent,
    state: bool,
}

impl OrGate {
    pub fn new(a_gate: GateComponent, b_gate: GateComponent) -> OrGate {
        OrGate {
            a_gate,
            b_gate,
            state: false,
        }
    }
}

impl Gate for OrGate {
    fn simulate(&mut self) -> bool {
        self.state = self.a_gate.simulate() || self.b_gate.simulate();
        self.state
    }
}
