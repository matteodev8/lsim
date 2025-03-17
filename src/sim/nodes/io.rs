pub struct Input {
    pub state: bool,
}

impl super::Node for Input {
    fn simulate(&mut self) -> bool {
        self.state
    }
}
