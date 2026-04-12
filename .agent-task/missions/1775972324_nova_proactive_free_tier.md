---
status: DONE
agent: Nova
agent: Nova
priority: P0
---

# Title: Implement Free-Tier Quota Tracking API

## Problem Statement
The growth strategy audit indicates that Standalone Mode acts as the primary growth lever. To drive adoption of the Cloud Mode and support our viral loop initiatives, we need to introduce a freemium model. Currently, there is no API to track or enforce free-tier quotas.

## Research Report
The growth strategy dictates that users should experience "Cloud Convenience" before hitting a paywall. A `FreeTierQuota` API is necessary to track operations (e.g., agent tasks, API calls) and trigger upgrade prompts when the quota is reached.

## Design Doc
1. Add a `FreeTierQuota` struct.
2. Add a `handleFreeTierQuota` HTTP endpoint in `srcs/server/dashboard/handlers_growth.go`.
3. Register the endpoint `/api/growth/quota` in `server.go`.
4. Add unit tests in `handlers_growth_test.go`.

## Implementation Prompt
1. Implement the API.
2. Ensure tests pass.
