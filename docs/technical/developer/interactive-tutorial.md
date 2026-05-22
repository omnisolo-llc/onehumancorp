<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# Interactive Developer Tutorial: Building a New Agent Persona

Welcome to the One Human Corp (OHC) interactive developer tutorial. This guide will walk you through the process of registering a new agent persona and wiring it into the Swarm Intelligence Protocol (OHC-SIP).

## Step 1: Define the Persona Configuration

First, define your new persona in the agent registry. This involves updating the provider configurations.

Create a new Rust module under `src/agents/builtin/` or modify the existing built-in provider registry to include your agent's profile:

```rust
pub struct AgentProfile {
    pub name: &'static str,
    pub role: &'static str,
    pub priority: u8,
    pub capabilities: &'static [&'static str],
}

pub const QUALITY_ASSURANCE_PERSONA: AgentProfile = AgentProfile {
    name: "Sentinel",
    role: "qa_engineer",
    priority: 1,
    capabilities: &[
        "browser_verification",
        "playwright_execution",
    ],
};
```

## Step 2: Implement the Action Loop

Every agent requires a main loop to execute its "Think → Act → Observe → Decide" methodology.

```rust
impl SentinelAgent {
    pub async fn run(&mut self, mut shutdown: tokio::sync::watch::Receiver<bool>) -> anyhow::Result<()> {
        loop {
            tokio::select! {
                _ = shutdown.changed() => break,
                Some(task) = self.mailbox.recv() => self.process_task(task).await?,
            }
        }
        Ok(())
    }
}
```

## Step 3: Enforce Hybrid Persistence

Your agent must be able to persist its state regardless of the runtime mode (Cloud vs. Standalone).

Use the injected repository or `sqlx` pool to execute queries:

```rust
// Example SQLite / PostgreSQL fallback execution
sqlx::query("INSERT INTO agent_missions (id, agent_id, status) VALUES ($1, $2, $3)")
    .bind(&task.id)
    .bind(&self.id)
    .bind("IN_PROGRESS")
    .execute(&self.pool)
    .await?;
```

## Step 4: Ensure Visual Excellence (Docs/UI)

If your agent generates documentation or UI templates, it **must** inject the OHC Visual Excellence styles.

```html
<!-- Inject this at the root of generated HTML/Markdown content -->
<div style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif;">
  ... content ...
</div>
```

## Step 5: Hermetic Verification

Before you commit the new agent, ensure it builds hermetically inside the Bazel sandbox.

```bash
# Run the test suite
bazelisk test //src/agents/...
```

<div style="margin-top: 20px; padding: 15px; border-left: 4px solid #007BFF; background: rgba(0, 123, 255, 0.1);">
  <strong>Pro Tip:</strong> Ensure that your agent gracefully handles the absence of heavy dependencies (like Redis) by leveraging local memory structures when in Standalone Desktop Mode.
</div>

</div>
