<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# Interactive Developer Tutorial: Building a New Agent Persona

Welcome to the One Human Corp (OHC) interactive developer tutorial. This guide will walk you through the process of registering a new agent persona and wiring it into the Swarm Intelligence Protocol (OHC-SIP).

## Step 1: Define the Persona Configuration

First, define your new persona in the agent registry. This involves updating the provider configurations.

Create a new module under `src/agents/builtin/` or extend the relevant orchestration department under `src/server/orchestration/departments/`:

```rust
#[derive(Debug, Clone)]
pub struct AgentProfile {
    pub name: String,
    pub role: String,
    pub priority: i32,
    pub capabilities: Vec<String>,
}

pub fn quality_assurance_persona() -> AgentProfile {
    AgentProfile {
        name: "Sentinel".to_string(),
        role: "qa_engineer".to_string(),
        priority: 1,
        capabilities: vec![
            "browser_verification".to_string(),
            "playwright_execution".to_string(),
        ],
    }
}
```

## Step 2: Implement the Action Loop

Every agent requires a main loop to execute its "Think → Act → Observe → Decide" methodology.

```rust
pub async fn run(mut mailbox: tokio::sync::mpsc::Receiver<Task>) -> Result<(), String> {
    while let Some(task) = mailbox.recv().await {
        process_task(task).await?;
    }
    Ok(())
}
```

## Step 3: Enforce Hybrid Persistence

Your agent must be able to persist its state regardless of the runtime mode (Cloud vs. Standalone).

Use `crate::db::DB` and branch on `DbStore` when SQL dialects differ:

```rust
match &db.store {
    crate::db::DbStore::Postgres => {
        sqlx::query("INSERT INTO agent_missions (id, agent_id, status) VALUES ($1, $2, $3)")
            .bind(&task.id)
            .bind(&agent_id)
            .bind("IN_PROGRESS")
            .execute(&db.pool)
            .await?;
    }
    crate::db::DbStore::Sqlite(pool) => {
        sqlx::query("INSERT INTO agent_missions (id, agent_id, status) VALUES (?, ?, ?)")
            .bind(&task.id)
            .bind(&agent_id)
            .bind("IN_PROGRESS")
            .execute(pool)
            .await?;
    }
}
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
bazelisk test //src/agents/... //src/server/...
```

<div style="margin-top: 20px; padding: 15px; border-left: 4px solid #007BFF; background: rgba(0, 123, 255, 0.1);">
  <strong>Tip:</strong> Ensure that your agent gracefully handles the absence of heavy dependencies like Redis by using local transport or SQLite-backed state in Standalone Desktop Mode.
</div>

</div>
