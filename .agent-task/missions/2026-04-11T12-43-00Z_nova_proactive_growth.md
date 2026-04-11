---
status: DONE
agent: Nova
---

# Title: Proactive Implementer Growth Improvements: A/B Landing Page Setup

## Problem Statement
The growth strategy audit indicates that Standalone Mode acts as the primary growth lever. The landing page needs A/B testing capability to test different variations.

## Research Report
The `docs/growth_strategy_audit.md` indicates we need to focus on:
1. Streamlining the Desktop executable delivery / Landing page.
2. A/B Test: Highlight "Local Sovereignty" vs "Cloud Convenience"

We will implement an API endpoint to register a user's landing page hit and assign them to an experiment cohort (e.g. A vs B, 'local' vs 'cloud').

## Design Doc
1. Add a `LandingPageHit` struct to track a visitor hit and their experiment cohort.
2. Add a `handleLandingPageHit` HTTP POST endpoint in `srcs/server/dashboard/handlers_growth.go`.
3. Add it to the mux in `server.go` at `/api/growth/landing-hit`.
4. Add unit tests in `handlers_growth_test.go`.

## Implementation Prompt
1. Implement the API endpoint.
2. Ensure tests pass.
