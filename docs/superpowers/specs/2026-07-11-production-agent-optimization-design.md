# Production Backend and Agent Optimization Design

## Objective

Review and optimize the production-critical backend and agent request paths for performance, security, and LLM token efficiency. Response quality takes priority over token savings. The work may include deeper architectural refactors when measurements justify them.

Because production profiling data is not available in the workspace, the project will first establish reproducible local baselines. Every optimization must be supported by before-and-after evidence.

## Scope

The audit follows representative production requests vertically through this path:

```text
API/authentication
  -> tenant isolation
  -> agent service and orchestration
  -> memory and context assembly
  -> LLM provider
  -> tool and MCP execution
  -> persistence and queueing
  -> streamed response
```

Primary source areas are:

- `src/server/api/agents`, `src/server/services/agent`, `src/server/orchestration`, and `src/server/workers`.
- `src/server/auth` and the storage boundaries reached by agent requests.
- `src/agents/builtin/core`, `llm`, `tools`, `memory`, `guardrails`, `mcp`, and `sandbox`.
- Shared pricing, prompt caching, telemetry, and harness code used by those request paths.
- Deployment configuration where it directly affects runtime security or performance.

UI code, unrelated business domains, and integrations are out of scope unless a traced agent request reaches them. Findings outside the primary path may be recorded, but they will not trigger unrelated refactoring in this cycle.

## Audit Method

### Representative scenarios

A reproducible harness will cover a small set of representative agent operations:

1. An authenticated conversational request that uses memory and streams a response.
2. A request that selects and executes a tool or MCP operation.
3. A request that persists state or queues follow-up work.
4. Denied and malformed requests that exercise authorization, tenant isolation, and tool-policy boundaries.
5. Provider, database, queue, and tool failures that exercise retry, timeout, cancellation, and error-redaction behavior.

The harness will use deterministic fixtures and a deterministic fake provider when an external model would make comparisons unstable. Focused provider contract tests may use recorded or synthetic responses without storing secrets or sensitive production data.

### Performance baseline

Each scenario will record, where the existing architecture makes the metric observable:

- End-to-end latency and stage-level timings.
- Time to first streamed response item.
- CPU time and allocations or process memory.
- Database query counts and durations.
- Queue operations and payload sizes.
- Serialization, cloning, buffering, and network payload sizes.
- Tool-call counts, retries, timeout events, and concurrency.

Repeated runs will include warm-up and report the sample count, central tendency, and dispersion. Comparisons must use the same build profile, fixture, environment, and measurement procedure.

### Token and quality baseline

The harness will record input, cached-input, and output token counts using provider usage data when available and the provider-compatible tokenizer or current repository estimator otherwise. It will attribute context size to system instructions, conversation history, memory, tool definitions, tool results, and other retrieved material.

Quality is protected by deterministic scenario-specific assertions. These include required facts, correct tool selection and arguments, expected safety or authorization decisions, response structure, and preservation of essential instructions. Token reductions that fail an existing quality assertion are rejected. When a change intentionally alters expected behavior to fix a vulnerability, the new secure behavior becomes an explicit regression assertion.

### Security baseline

The security review combines repository-supported dependency and configuration scans with manual data-flow analysis of:

- Authentication, authorization, and tenant isolation.
- Secret storage, credential propagation, and telemetry redaction.
- Prompt injection and untrusted context boundaries.
- Tool argument validation and privileged-operation approval.
- SSRF, path traversal, unsafe URL handling, and outbound destination policy.
- Sandbox boundaries and command or file access.
- Deserialization, SQL construction, and injection surfaces.
- Cryptographic use and token verification.
- Denial-of-service risks such as unbounded input, recursion, retries, buffering, or concurrency.

Findings will include severity, affected path, evidence, exploit or failure preconditions, and a concrete remediation. Automated scanner output without validation is not treated as a confirmed vulnerability.

## Optimization Architecture

### Request-path performance

The implementation will target measured sources of latency or resource use, including redundant serialization, excessive cloning, repeated database queries, blocking operations in asynchronous paths, unnecessary buffering, and unbounded concurrency. Bounded parallelism may be introduced when ordering and cancellation semantics remain correct.

Caching is permitted only when ownership, invalidation, memory limits, and tenant-safe cache keys are explicit. A cache must not weaken authorization checks or expose data across organizations.

### Agent and token efficiency

Context construction will become independently measurable and testable. Candidate improvements include:

- Removing duplicate or unused context while preserving instruction precedence.
- Ranking retrieved memory and enforcing per-source token budgets.
- Sending only tools available and relevant to the current request.
- Stabilizing cacheable prompt prefixes without mixing tenant-specific data into shared cache keys.
- Avoiding repeated inclusion of unchanged tool results or metadata.
- Summarizing history only when quality fixtures show that essential facts and instructions survive.
- Bounding planning, retry, and tool loops to prevent runaway token use.

Token budgets are policy inputs rather than scattered constants. The budgeting interface will expose why content was included, excluded, or summarized so regressions can be diagnosed.

### Security hardening

Security controls will be applied at trust boundaries rather than relying on prompt text. Authorization and tenant checks occur before data access and privileged tool execution. Tool inputs and outbound destinations are parsed and validated against explicit policy. Sensitive values are redacted before logging or telemetry export.

Sandbox and filesystem access use canonicalized, constrained paths and deny access outside configured roots. Network and subprocess operations use bounded inputs, timeouts, cancellation, and least privilege. Cryptographic behavior will use established library APIs and fail closed on invalid or ambiguous input.

### Component boundaries

Where coupling prevents reliable measurement or secure changes, the refactor will separate these responsibilities behind narrow interfaces:

- Context selection and provenance.
- Token budgeting and quality constraints.
- Provider invocation and usage accounting.
- Tool discovery, argument validation, authorization, and execution.
- Retry, timeout, and cancellation policy.
- Security-safe telemetry.

These boundaries should permit deterministic fakes in tests and allow internal implementation changes without changing consumers.

## Error Handling

Failures will be classified into user input errors, authentication or policy denials, retryable dependency failures, permanent dependency failures, cancellations, timeouts, and internal faults. User-visible errors remain sanitized. Internal diagnostics include correlation identifiers and actionable context without prompts, tokens, credentials, or tenant data unless an existing explicitly authorized secure diagnostic path requires them.

Retries are bounded, jittered, observable, and restricted to operations known to be idempotent or protected by an idempotency key. Cancellation and deadlines propagate through orchestration, providers, tools, database calls, and streaming. Partial failures must not silently commit inconsistent state.

## Verification

Implementation requires:

- Unit tests for extracted policies and each corrected defect.
- Integration tests for authentication, tenant isolation, context construction, provider and tool failures, persistence, queue behavior, and streaming.
- Security regression tests for every confirmed vulnerability.
- Deterministic quality fixtures comparing baseline and optimized behavior.
- Repeated before-and-after benchmarks with reproducible commands and environment notes.
- Relevant existing Rust, Bazel, and repository lint and test targets for every changed module.

No performance or token improvement is claimed without measurements from comparable runs. No completion claim is made while relevant tests or security regressions are failing.

## Prioritization and Delivery

Findings and changes are ordered by:

1. Confirmed critical or high-severity security issues.
2. Cross-tenant exposure, privilege escalation, or uncontrolled tool execution risks.
3. Large measured latency, memory, or token costs on common request paths.
4. Reliability defects that amplify retries, work, or token consumption.
5. Architectural improvements needed to safely implement or verify the items above.

Architectural changes will be divided into reviewable, reversible commits. Compatibility is preserved unless existing behavior is demonstrably insecure. Any required incompatible security change must include migration or rollout notes and explicit regression coverage. Configuration or rollout controls will guard changes whose operational risk cannot be eliminated through tests.

## Deliverables

The cycle will produce:

- Reproducible baseline and optimized benchmark results.
- A confirmed security findings report with severity and evidence.
- Implemented and tested performance, security, and token-efficiency changes.
- Quality comparison results for affected agent scenarios.
- A remaining-risk list covering deferred or unverified areas.
- Exact commands needed to reproduce the review, tests, and measurements.

Success means the production request paths have measurable improvements or documented evidence that no safe gain was available, confirmed security findings are fixed or explicitly deferred with rationale, and agent quality assertions are unchanged or strengthened.
