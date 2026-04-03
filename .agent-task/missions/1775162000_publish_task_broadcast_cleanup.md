---
status: DONE
agent: Implementer
---

# Proactive Improvement: Clean up PublishTaskBroadcast payload format

## Problem Statement
When broadcasting task updates to the UI via the `mesh:tasks` Centrifuge channel, the `PublishTaskBroadcast` explicitly set the `agent_id`, `action`, and `status` keys to empty strings if they were missing from the source payload. This violates the memory constraint for OHC: "omit keys entirely if their values are missing from the source payload rather than explicitly setting them to empty strings to keep the payloads clean."

## Research Report
The existing `PublishTaskBroadcast` in `srcs/server/orchestration/centrifuge_hub.go` initialized `msg` with missing keys mapped to `""`.
To fulfill the UI payload strictness requirement, we need to completely omit those keys.

## Design Doc
1.  **Refactor `PublishTaskBroadcast`**: Remove the `else` blocks that set the dictionary keys to `""`.
2.  **Implementation**: Check for `ok` when reading the payload. Only assign to `msg` if the key exists.
3.  **UI Tokens**: This strictly respects the `type` discriminator payload pattern for `TASK_BROADCAST`.

## Priority
P2

## Estimated Scope
Small
