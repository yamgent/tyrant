pub struct CommandBar {
    active: bool,
}

impl CommandBar {
    pub fn new() -> Self {
        Self { active: false }
    }

    pub fn active(&self) -> bool {
        self.active
    }
}
