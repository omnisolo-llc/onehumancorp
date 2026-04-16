<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif;">

# Kairos Orchestrator Shared Task List API

This interactive playbook outlines the core APIs that interact with the Kairos Orchestrator Shared Task List. The Shared Task List leverages a distributed state machine, preventing race conditions when sub-agents claim tasks, providing guaranteed atomicity for KAIROS Orchestration.

## Workflow Visualization

```mermaid
graph TD
    subgraph Swarm Orchestrator
        A[Principal Orchestrator Agent]
    end

    subgraph Shared Task List Queue
        Q1[PENDING Task 1]
        Q2[PENDING Task 2]
    end

    subgraph Sub-Agents
        SA1[Worker Sub-Agent Alpha]
        SA2[Worker Sub-Agent Beta]
    end

    A -->|1. Enqueue Task| Q1
    A -->|1. Enqueue Task| Q2
    SA1 -->|2. Claim Task| Q1
    SA2 -->|2. Claim Task| Q2
    Q1 -->|3. Complete Task| A
    Q2 -->|3. Complete Task| A

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,Q1,Q2,SA1,SA2 premium;
```

## 1. Enqueue a Task

Use this endpoint to queue a new task for sub-agents to claim. Dependent tasks can be chained.

*   **Endpoint:** `POST /api/queue/subagent`
*   **Description:** Queues a new task onto the Shared Task List. The state transitions to `PENDING`.
*   **Request Payload Example:**

    ```json
    {
      "parent_task_id": "epic-5560",
      "action": "implement_feature",
      "priority": "P0",
      "payload": {
        "repository": "onehumancorp/mono",
        "instructions": "Implement the shared task list."
      }
    }
    ```

*   **Success Response (201 Created):**

    ```json
    {
      "task_id": "task-8f92bd",
      "status": "PENDING",
      "queued_at": "2023-10-27T10:00:00Z"
    }
    ```

## 2. Claim a Task

Use this endpoint when an idle worker sub-agent wishes to grab a task. It ensures exclusive access via `FOR UPDATE SKIP LOCKED`.

*   **Endpoint:** `POST /api/v1/tasks/claim`
*   **Description:** Claims a `PENDING` task from the shared queue, transitioning its state to `IN_PROGRESS`.
*   **Request Payload Example:**

    ```json
    {
      "agent_id": "agent_swe_alpha",
      "role": "IMPLEMENTER"
    }
    ```

*   **Success Response (200 OK):**

    ```json
    {
      "task_id": "task-8f92bd",
      "status": "IN_PROGRESS",
      "claimed_at": "2023-10-27T10:05:00Z",
      "action": "implement_feature",
      "payload": { ... }
    }
    ```

*   **Failure Response (404 Not Found - Empty Queue):**

    ```json
    {
      "error": "No pending tasks available."
    }
    ```

## 3. Complete a Task

Marks the successful execution of the task and unlocks the parent execution DAG.

*   **Endpoint:** `POST /api/v1/tasks/{task_id}/complete`
*   **Description:** Updates the task state to `COMPLETED` and commits the result summary to the shared context.
*   **Request Payload Example:**

    ```json
    {
      "agent_id": "agent_swe_alpha",
      "outcome_summary": "Successfully committed changes in PR #1234.",
      "artifacts": ["https://github.com/onehumancorp/mono/pull/1234"]
    }
    ```

*   **Success Response (200 OK):**

    ```json
    {
      "task_id": "task-8f92bd",
      "status": "COMPLETED",
      "completed_at": "2023-10-27T11:00:00Z"
    }
    ```

</div>
