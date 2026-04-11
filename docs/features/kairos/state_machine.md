<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; font-family: 'Outfit', 'Inter', sans-serif;">

# Distributed State Machine

The Distributed State Machine is a core component of the KAIROS Orchestration layer, designed to ensure robust and consistent state management across the One Human Corp (OHC) Swarm. It prevents race conditions and manages task assignments effectively.

## Core Mechanisms

- **Cloud-Native Mode**: In multi-tenant environments, the state machine utilizes PostgreSQL's `FOR UPDATE SKIP LOCKED` functionality. This allows multiple agents to concurrently query the shared task list without colliding, securely claiming tasks.
- **Standalone Mode**: When operating locally on a single machine, the system gracefully degrades to SQLite, utilizing application-level Mutex locks to maintain state integrity.

## State Transitions

The state machine manages the lifecycle of tasks within the `shared_tasks` table. The typical task state flow is:
`PENDING` -> `ASSIGNED` -> `IN_PROGRESS` -> `DONE` (or `BLOCKED`)

## API Reference
The API endpoints for interacting with the State Machine are documented in the [API Playbook](../../api_playbook.md).

</div>
