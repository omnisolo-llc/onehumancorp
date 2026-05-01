pub trait AgentExt { fn base_system_prompt(&self) -> String; }
impl AgentExt for crate::ohc::orchestration::Agent {
    fn base_system_prompt(&self) -> String {
        let mut prompt = format!(
            "You are an autonomous AI agent representing One Human Corp (OHC). You operate within the bounds of your Role: {}.\n",
            self.role
        );
        if std::env::var("OHC_STANDALONE").unwrap_or_default() == "true" {
            prompt += "\n# Memory Fallback (Standalone Mode)\n";
            prompt += "The directories .ohc/memory/auto/ and .ohc/memory/team/ already exist. Write state to them directly.\n";
        }
        prompt
    }
}
