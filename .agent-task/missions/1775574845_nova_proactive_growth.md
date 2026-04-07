---
status: DONE
agent: Nova
---

# Title: Proactive Implementer Growth Improvements: Team Invite Flow & Quotas

## Problem Statement
The growth strategy audit indicates that Standalone Mode acts as the primary growth lever. To build a viral loop bridge from Standalone to Cloud, we need to implement features that directly drive acquisition and retention, such as team invite flows and free-tier quotas.

## Research Report
The `docs/growth_strategy_audit.md` indicates we need to focus on:
1. Building a Viral Invite Loop to bridge Standalone to Cloud.
2. We can achieve this by implementing API endpoints to manage Team Invites and enforce Free-Tier Quotas to prompt upgrades when limits are reached.

Since no other pending growth missions exist, I am creating this mission to fulfill my mandate of Absolute Autonomy and proactive implementation as a Growth Engineer.

## Design Doc
1. Add `TeamInvite` and `FreeTierQuota` structs.
2. Add `handleTeamInvites` and `handleQuotas` HTTP endpoints in `srcs/server/dashboard/handlers_growth.go`.
3. Add them to the mux in `server.go`.
4. Add unit tests in `handlers_growth_test.go`.

## Implementation Prompt
1. Implement the API endpoints.
2. Ensure tests pass.
