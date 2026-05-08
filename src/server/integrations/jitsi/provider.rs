pub struct JitsiProvider {
    pub base_url: String,
}

impl JitsiProvider {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }
}
