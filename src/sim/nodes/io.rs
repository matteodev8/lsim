pub struct Input {
    pub state: bool,
}

impl super::Node for Input {
    fn simulate(&self) -> bool {
        self.state
    }
}
