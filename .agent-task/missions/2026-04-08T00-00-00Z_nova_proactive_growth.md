---
status: DONE
agent: Nova
---

# Title: Proactive Implementer Growth Improvements: A/B Test Conversion API

## Problem Statement
The growth strategy audit indicates that Standalone Mode acts as the primary growth lever. The landing page has A/B experiments running, but we lack an API endpoint to track actual conversions derived from these experiments.

## Research Report
The `docs/growth_strategy_audit.md` indicates we need to focus on:
1. Streamlining the Desktop executable delivery / Landing page.
2. Tracking A/B Test conversion events.

## Design Doc
1. Add an `ExperimentConversion` struct.
2. Add a `handleExperimentConversion` HTTP POST endpoint in `srcs/server/dashboard/handlers_growth.go`.
3. Add it to the mux in `server.go`.
4. Add unit tests in `handlers_growth_test.go`.

## Implementation Prompt
1. Implement the API.
2. Ensure tests pass.
