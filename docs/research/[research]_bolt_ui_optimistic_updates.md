# Optimistic Updates: Perceived Performance at the Bolt Standard

## Definition
Optimistic Updates is a UI pattern where the client assumes a backend operation will succeed and updates the interface immediately, before receiving a server response.

## 1. Why it Matters for OHC
In a hybrid OS, especially in Standalone mode or on rural networks, even a 200ms round-trip feels "disconnected" to a human user. By using optimistic updates, we achieve a perceived latency of **0ms**.

## 2. Implementation Patterns

### Agent Hiring/Firing
When a user clicks "Hire," the UI immediately shows the agent in the "Hiring..." state in the list. The actual gRPC call happens in the background.
- **Success**: The agent transitions to "IDLE."
- **Failure**: The UI rolls back the state and shows a "Hiring Failed" notification.

### Task Approvals
Approving a task in the Action Feed is the most common human-agent interaction.
- **Bolt Pattern**: Clicking "Approve" removes the item from the feed immediately. If the server fails, the item is re-injected with an error badge.

## 3. Data Synchronization Latency
Optimistic updates create a temporary divergence between client and server state.
- **Conflict Resolution**: We use a versioning system (implemented in our Postgres and SQLite schemas as a `version` column) to handle cases where a background sync from another device conflicts with an optimistic local update.
- **Last-Write-Wins (LWW)**: Our standard resolution policy is LWW based on the `updated_at` timestamp, ensuring that the user's most recent intent is preserved.

## 4. Synergy with Bolt Backend
Optimistic updates work best when the backend is also fast. If a backend takes 10 seconds to confirm an update, the risk of a visible "rollback" increases. By combining Bolt's sub-100ms API response times with optimistic UI, we create an experience that feels faster than the laws of physics should allow.

## Conclusion
Performance is a full-stack endeavor. While our backend optimizations provide the "raw horsepower," optimistic UI provides the "handling" that makes OHC feel nimble and responsive to entrepreneurs everywhere.
