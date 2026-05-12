pub struct CalendlyClient {
    token: String,
}

impl CalendlyClient {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}
