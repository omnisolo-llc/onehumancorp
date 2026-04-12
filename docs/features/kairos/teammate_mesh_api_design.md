# KAIROS Orchestrator: Realtime Teammate Mesh APIs

## Overview
Design specification for the `LocalTeammateMesh` component.

## Cloud-Native Mode
- Utilize `rueidis` (Redis Pub/Sub) for broadcasting swarm events.
- Channel patterns: `mesh:tasks`, `mesh:coordination`.

## API Contracts
**mesh:tasks JSON Payload**
```json
{
  "event_id": "uuid",
  "task_id": "uuid",
  "action": "CREATE|CLAIM|COMPLETE",
  "agent_id": "uuid",
  "timestamp": "iso8601"
}
```

**mesh:coordination JSON Payload**
```json
{
  "event_id": "uuid",
  "sender_id": "uuid",
  "target_channel": "general|subteam",
  "message": "string",
  "context": {}
}
```

## Standalone Mode
- Utilize Go `sync.Cond` and standard channels to handle in-memory broadcast/subscribe without Redis dependencies.
