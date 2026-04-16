---
status: DONE
agent: Link
---

# Title: Fix OHC-SIP Teammate Mesh API Middleware Compliance

## Problem Statement
The current `ValidationMiddleware` in `srcs/server/api/mesh/middleware.go` enforces an incorrect payload structure (`agent_id`, `action`, `status`) for the Teammate Mesh unified gateway (`POST /api/mesh/broadcast`). The Swarm Intelligence Protocol (OHC-SIP) mandates a specific payload structure for Teammate Mesh APIs, requiring the keys `agent_id`, `channel`, `event_type`, and `data` in JSON broadcast requests.

## Research Report
According to `docs/architecture/KAIROS_AI_OS_HYBRID_CORE_DESIGN.md` and the newly authored `docs/architecture/KAIROS_OHC_SIP_COMPLIANCE_DESIGN.md`, the correct OHC-SIP payload structure for Teammate Mesh APIs MUST include `agent_id`, `channel`, `event_type`, and `data`. The current implementation in `srcs/server/api/mesh/middleware.go` is verifying deprecated keys, causing valid OHC-SIP events to be rejected or improperly structured events to be accepted.

## Design Doc
Refer to `docs/architecture/KAIROS_OHC_SIP_COMPLIANCE_DESIGN.md` for the exact payload contract.
- Update `meshPayload` struct in `srcs/server/api/mesh/middleware.go` to require `agent_id`, `channel`, `event_type`, and `data` (where `data` is `json.RawMessage`).
- Update the validation logic in `ValidationMiddleware` to reject requests missing any of these four keys.
- Update all associated unit tests in `srcs/server/api/mesh/middleware_test.go` and `srcs/server/api/mesh/mesh_test.go` to use the correct OHC-SIP payload structure.

## Implementation Prompt
Hello Implementer! Your objective is to align the Teammate Mesh unified gateway middleware with the strict OHC-SIP compliance rules.
1. Read the exact specifications in `docs/architecture/KAIROS_OHC_SIP_COMPLIANCE_DESIGN.md`.
2. Update `srcs/server/api/mesh/middleware.go` so the `meshPayload` struct maps `agent_id` (string), `channel` (string), `event_type` (string), and `data` (json.RawMessage).
3. Update the `ValidationMiddleware` function to verify the presence of all four keys.
4. Update `srcs/server/api/mesh/middleware_test.go` and `srcs/server/api/mesh/mesh_test.go` to match the new strict requirements.
5. Provide a summary of the implementation via a new GitHub PR.

## Priority
P0

## Estimated Scope
Small
