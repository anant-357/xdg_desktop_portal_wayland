pub struct Session {
    version: u8,
}

impl Session {
    pub fn from_options() -> Self {
        Session { version: 1 }
    }

    pub fn close(self) {}
}
