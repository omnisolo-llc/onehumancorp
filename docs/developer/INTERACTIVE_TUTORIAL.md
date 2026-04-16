<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# Interactive Developer Tutorial: Building a New Agent Persona

Welcome to the One Human Corp (OHC) interactive developer tutorial. This guide will walk you through the process of registering a new agent persona and wiring it into the Swarm Intelligence Protocol (OHC-SIP).

## Step 1: Define the Persona Configuration

First, define your new persona in the agent registry. This involves updating the provider configurations.

Create a new file in `srcs/server/agents/` or modify an existing registry file to include your agent's profile:

```go
package agents

import "github.com/onehumancorp/ohc/srcs/server/agents/types"

var QualityAssurancePersona = types.AgentProfile{
    Name:     "Sentinel",
    Role:     "qa_engineer",
    Priority: 1, // P1 Priority
    Capabilities: []string{
        "browser_verification",
        "playwright_execution",
    },
}
```

## Step 2: Implement the Action Loop

Every agent requires a main loop to execute its "Think → Act → Observe → Decide" methodology.

```go
func (a *SentinelAgent) Run(ctx context.Context) error {
    for {
        select {
        case <-ctx.Done():
            return ctx.Err()
        case task := <-a.mailbox:
            a.processTask(ctx, task)
        }
    }
}
```

## Step 3: Enforce Hybrid Persistence

Your agent must be able to persist its state regardless of the runtime mode (Cloud vs. Standalone).

Use the injected `db.Provider` interface to execute queries:

```go
// Example SQLite / PostgreSQL fallback execution
query := `INSERT INTO agent_missions (id, agent_id, status) VALUES ($1, $2, $3)`
// Note: Ensure the db package translates $1 to ? for SQLite automatically!
err := a.db.Exec(ctx, query, task.ID, a.id, "IN_PROGRESS")
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
# Update Go dependencies
bazelisk run //:gazelle

# Run the test suite
bazelisk test //srcs/server/agents/...
```

<div style="margin-top: 20px; padding: 15px; border-left: 4px solid #007BFF; background: rgba(0, 123, 255, 0.1);">
  <strong>Pro Tip:</strong> Ensure that your agent gracefully handles the absence of heavy dependencies (like Redis) by leveraging local memory structures when in Standalone Desktop Mode.
</div>

</div>
