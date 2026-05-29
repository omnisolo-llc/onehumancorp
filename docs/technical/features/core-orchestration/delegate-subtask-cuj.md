<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# CUJ: Hierarchical Task Delegation (Delegate SubTask)

**Author(s):** TPM Agent
**Status:** Approved
**Last Updated:** 2026-03-27

## 1. User Journey Overview
An agent receives a complex task and decides to provision a temporary, specialized sub-agent to handle a specific part of that task. The system enforces strict VRAM quota limits to prevent unbounded resource consumption, provisions the new sub-agent, and assigns the instructions in an isolated thread.

## 2. Detailed Step-by-Step Breakdown

| Step | User Action / Agent Trigger | System Trigger | Resulting State | Verification |
|------|-----------------------------|----------------|-----------------|--------------|
| 1 | Agent identifies a sub-task requiring specialized skills | Agent calls `DelegateSubTask` with `task_id` and `target_role` | System checks VRAM quota (max 10 active agents) | Quota is evaluated |
| 2 | System validates quota and creates sub-agent | System provisions a new `Agent` with `dynamic-delegation` organization | Sub-agent is registered in the Hub | Sub-agent ID is generated |
| 3 | System assigns instructions | System publishes a `TaskDelegation` message to the sub-agent from the originating agent | Sub-agent receives instructions and isolated thread context | Original caller receives success response |

## 3. Implementation Details

```mermaid
graph TD;
    Agent[Agent Caller] -->|DelegateSubTask| Hub;
    Hub -->|Quota Check < 10| Provision[Spawn Sub-Agent];
    Hub -.->|Quota Check >= 10| Reject[ResourceExhausted Error];
    Provision --> Thread[Isolate Thread Context];
    Thread --> Execute[Publish Instruction];
    Execute --> Success[Caller Receives Success];

    %% Premium OHC Glassmorphism Tokens
    style Agent fill:rgba(255, 255, 255, 0.05),stroke:rgba(255, 255, 255, 0.1),backdrop-filter:blur(15px) saturate(180%)
    style Hub fill:rgba(255, 255, 255, 0.05),stroke:rgba(255, 255, 255, 0.1),backdrop-filter:blur(15px) saturate(180%)
    style Reject fill:rgba(255, 0, 0, 0.1),stroke:rgba(255, 0, 0, 0.3)

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class Provision,Agent,Thread,saturate,Success,Reject,blur,Execute,rgba premium;
```

- **Architecture**: The `DelegateSubTask` RPC method handles quota checking, dynamic agent provisioning, and message publication on the `Hub`.
- **State Management**: The sub-agent is registered dynamically with `StatusIdle` and a generated ID containing its role and a timestamp.

## 4. Edge Cases
- **VRAM Quota Exceeded**: If the total active agents across the Hub reaches or exceeds 10, the system blocks the creation of the sub-agent and returns a `ResourceExhausted` error.
- **Missing Arguments**: If the `task_id` or `target_role` are missing, an `InvalidArgument` error is returned immediately.

## 5. UI/UX Details
- The UI can display a "Spawning Specialized [Role] Agent" indicator in the transcript, reflecting dynamic team assembly.
- Quota errors should be surfaced as a "VRAM Quota Reached: Cannot spawn additional agents" notification to the human CEO.

</div>
