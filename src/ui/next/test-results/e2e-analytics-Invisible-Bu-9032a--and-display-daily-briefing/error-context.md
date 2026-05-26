# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: e2e/analytics.spec.ts >> Invisible Business Analytics and Growth Engine >> should ingest business events and display daily briefing
- Location: e2e/analytics.spec.ts:6:7

# Error details

```
Error: apiRequestContext.post: connect ECONNREFUSED ::1:18789
Call log:
  - → POST http://localhost:18789/api/v1/analytics/ingest
    - user-agent: Playwright/1.60.0 (x64; ubuntu 24.04) node/22.22
    - accept: */*
    - accept-encoding: gzip,deflate,br
    - content-type: application/json
    - content-length: 117

```