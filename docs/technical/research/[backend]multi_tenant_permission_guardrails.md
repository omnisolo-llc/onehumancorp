# [backend] Multi-Tenant Permission Guardrails

## Problem Statement
OHC agents currently execute commands without interactive user approval. For high-risk operations (e.g., deleting a database, making large financial transactions), we need a standardized "Request-Approval" flow that works across Cloud-Native and Standalone modes.

## Research Report
- **Competitor Analysis**: Claude Code implements a sophisticated `PermissionPrompt` that pauses execution until the user clicks "Approve" in the TUI/CLI.
- **KAIROS Gap**: We lack a centralized policy engine that determines which tools require approval based on the `OrganizationID` and user roles.

## Design Doc
- **Permission Store**: A Redis-backed (Cloud) or SQLite-backed (Standalone) store for permission rules.
- **Guardrail Middleware**: A Go middleware that intercepts tool calls and checks against the policy.
- **Approval Queue**: If a tool is "Sensitive", enqueue a request in the `shared_tasks_decomposition` table with state `PENDING_APPROVAL`.
- **Real-time Notify**: Use Teammate Mesh (Redis Pub/Sub) to notify the UI of pending approvals.

## Implementation Prompt
1. Create a `PermissionManager` in `src/server/auth/permissions.go`.
2. Define `PermissionRule` struct: `ToolName`, `Pattern`, `Action` (Allow, Deny, Ask).
3. Update `src/server/orchestration/statemachine/machine.go` to handle the `PENDING_APPROVAL` state.
4. Implement a gRPC endpoint `RequestPermission(ctx, req)` that triggers the UI notification.
5. Write tests verifying that an agent task pauses when a "Sensitive" tool is called and resumes only after a "user" approves it via a mocked UI call.

## Priority
P1

## Estimated Scope
Medium
