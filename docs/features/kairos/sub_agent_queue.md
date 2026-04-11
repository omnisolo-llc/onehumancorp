<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; font-family: 'Outfit', 'Inter', sans-serif;">

# Sub-Agent Queue

The Sub-Agent Queue facilitates the asynchronous assignment and monitoring of sub-tasks to specialized agents within the OHC Swarm.

## Workflow

When a Director or Lead Agent decomposes a task:
1. Sub-tasks are created and added to the Queue.
2. Available agents poll or are pushed tasks based on the operating mode (Cloud vs. Standalone).
3. The Teammate Mesh broadcasts queue updates in real-time.

## Architecture Visualization

```mermaid
graph TD
    A[Director Agent] -->|Decomposes Task| Q[Sub-Agent Queue]
    Q -->|Assigns| W1[Worker Agent A]
    Q -->|Assigns| W2[Worker Agent B]
    W1 -->|Updates Status| Q
    W2 -->|Updates Status| Q

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,Q,W1,W2 premium;
```

## Real-time Coordination
The queue integrates closely with the Teammate Mesh APIs to emit `MeshEvent` objects whenever a queue item changes state. This enables immediate UI updates and agent synchronization.

For specific API endpoints, see the [API Playbook](../../api_playbook.md).

</div>
