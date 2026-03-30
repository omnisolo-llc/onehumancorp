# Checkpointer Module

## Identity
The `checkpointer` module provides persistence for LangGraph threads in the One Human Corp platform. It implements a generic `LangGraphCheckpointer` interface to save and load state snapshots, ensuring agent workflows can be paused, resumed, and recovered seamlessly.

## Architecture
This module currently implements `PGCheckpointer`, a database-backed checkpointer (compatible with both PostgreSQL and SQLite). It utilizes robust exponential backoff and retry mechanisms to handle transient database locks, crucial for high-concurrency environments.

```mermaid
graph TD;
    Agent[Agent Workflow] -->|Save State| Checkpointer[LangGraphCheckpointer];
    Checkpointer -->|SQL Upsert| DB[(PostgreSQL / SQLite)];
    DB -->|Load State| Checkpointer;
    Checkpointer -->|Restore Context| Agent;

    %% OHC Premium Branding Tokens
    style Checkpointer fill:rgba(255, 255, 255, 0.05),stroke:rgba(255, 255, 255, 0.1),backdrop-filter:blur(15px) saturate(180%)
    style DB fill:rgba(255, 255, 255, 0.03),stroke:rgba(255, 255, 255, 0.08)
```

> **Developer Insight:**
> *When implementing persistence in a distributed agentic framework, treating memory as a functional checkpoint (rather than an append-only log) prevents context window bloat and allows immediate recovery of "known-good" states.*

## Quick Start
Initialize the checkpointer with a valid `*sql.DB` connection and ensure the required table exists:

```go
package main

import (
	"context"
	"database/sql"
	"fmt"
	_ "github.com/mattn/go-sqlite3"
	"github.com/onehumancorp/mono/srcs/checkpointer"
)

func main() {
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		panic(err)
	}

	chk := checkpointer.NewPGCheckpointer(db)
	ctx := context.Background()

	// 1. Initialize Schema
	_ = chk.EnsureTableExists(ctx)

	// 2. Save State
	threadID := "agent-task-123"
	state := map[string]interface{}{
		"status": "In_Progress",
		"step":   2,
	}
	_ = chk.SaveCheckpoint(ctx, threadID, state)

	// 3. Load State
	snapshot, _ := chk.LoadCheckpoint(ctx, threadID)
	fmt.Printf("Recovered Thread: %s, State: %v\n", snapshot.ThreadID, snapshot.State)
}
```

## Developer Workflow
This module is built and tested using Bazel.

- **Build**: `bazelisk build //srcs/checkpointer`
- **Test**: `bazelisk test //srcs/checkpointer/...`
