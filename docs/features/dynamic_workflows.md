# Dynamic Workflows

Dynamic workflows let OHC turn a large prompt into a coordinated, resumable set of sub-agent jobs. The implementation follows the Claude Code dynamic workflow pattern: plan first, fan out across specialized workers, verify shard results independently, and synthesize only checked work into the final result.

## When a Workflow Starts

A workflow starts in either of these cases:

1. The user explicitly asks for one with phrases such as `create a workflow`, `dynamic workflow`, `run a workflow`, or `start a workflow`.
2. The request uses `effort: "ultracode"` or `effort: "xhigh"` with `auto_mode: true`, and the prompt looks broad enough to benefit from parallel orchestration.

The automatic path scores the prompt for signals such as `codebase-wide`, `entire`, `migration`, `rewrite`, `audit`, `security`, `profiler`, `optimization`, `verify`, `large`, and `legacy`.

## Confirmation Gate

Dynamic workflows can create many sub-agent jobs and consume materially more tokens than a normal session. For that reason, the first request only prepares the plan unless `confirm: true` is provided.

If confirmation is required, the workflow status is `awaiting_confirmation` and no queue jobs are created. Confirming the workflow enqueues the planned jobs.

## Execution Shape

Each workflow plan contains four phases:

| Phase | Purpose |
| --- | --- |
| `planning` | Map the prompt into independently executable shards. |
| `execution` | Run specialized sub-agents in parallel against those shards. |
| `verification` | Assign adversarial or independent reviewers to check each shard. |
| `synthesis` | Fold verified results into one coordinated final result. |

Plans are saved to `.ohc/dynamic-workflows` by default. Set `OHC_DYNAMIC_WORKFLOW_STATE_DIR` to store them elsewhere. Queue jobs are written through the existing `TaskQueue` abstraction, using PostgreSQL in cloud mode and SQLite in standalone mode.

## API

Create or preview a workflow:

```bash
curl -X POST http://localhost:18789/api/v1/dynamic-workflows \
  -H 'content-type: application/json' \
  -d '{
    "tenant_id": "org-1",
    "prompt": "Create a workflow to run a codebase-wide security audit and verify every finding",
    "effort": "medium",
    "confirm": false,
    "max_parallel_agents": 8,
    "verifier_agents_per_task": 1
  }'
```

Confirm a prepared workflow:

```bash
curl -X POST http://localhost:18789/api/v1/dynamic-workflows/{workflow_id}/confirm
```

Fetch a saved workflow:

```bash
curl http://localhost:18789/api/v1/dynamic-workflows/{workflow_id}
```

Use `ultracode` automatic mode:

```json
{
  "tenant_id": "org-1",
  "prompt": "Audit the entire legacy service for auth, input validation, and secret exposure, then verify every finding",
  "effort": "ultracode",
  "auto_mode": true,
  "confirm": true
}
```

## Queue Payload Contract

Each queued sub-agent job includes:

| Field | Meaning |
| --- | --- |
| `dynamic_workflow` | Always `true` for jobs created by this feature. |
| `workflow_id` | The parent dynamic workflow id. |
| `phase` | One of `planning`, `execution`, `verification`, or `synthesis`. |
| `dependencies` | Task ids that must be considered before this job reports final output. |
| `verification_of` | The execution task being checked, for verification jobs. |
| `agent_role` | Specialized worker role, such as `security-auditor` or `adversarial-reviewer`. |

The current implementation records dependencies in the payload so workers and downstream schedulers can enforce or inspect them. The existing sub-agent queue remains the transport layer.
