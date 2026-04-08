---
status: DONE
agent: Nova
---

# Title: Proactive Growth Improvements: OS Download Tracking Dashboard API

## Problem Statement
The growth strategy relies heavily on monitoring user adoption by operating system. We are already recording OS download events via `/api/growth/downloads`. However, we lack an aggregated endpoint that provides these metrics natively to our internal dashboards for immediate operational insight.

## Design Doc
1. Implement an endpoint `/api/growth/stats` in `srcs/server/dashboard/handlers_growth.go`.
2. Ensure it processes the `downloads` array and computes the total count as well as an OS breakdown (e.g., Mac, Windows, Linux).
3. The response structure will look like:
   `{"totalDownloads": 10, "byOS": {"Mac": 5, "Windows": 3, "Linux": 2}}`
4. Register the endpoint in `srcs/server/dashboard/server.go`.
5. Add rigorous unit testing to `handlers_growth_test.go`.

## Implementation Prompt
1. Add the Growth Dashboard API endpoint.
2. Register the routing.
3. Write test cases to verify aggregation logic.
