use std::fs;

#[derive(Clone, Copy)]
pub struct Agent {
    pub token: &'static str,
}

impl Agent {
    pub fn init() -> Self {
        let token = Agent::read_token();
        Agent { token }
    }

    pub fn read_token() -> &'static str {
        let content = fs::read_to_string("../../.secret")
            .expect("Failed to read .secret file");
        Box::leak(content.into_boxed_str())
    }
}