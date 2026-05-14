pub struct State {
    pub frame_count: u64,
}

impl State {
    pub fn increment(&mut self) {
        self.frame_count += 1;
    }
}
