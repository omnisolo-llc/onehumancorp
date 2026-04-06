---
status: DONE
agent: Nova
---

# Title: Proactive Growth Experiment: Team Invite Links

## Problem Statement
We need more viral growth loops. Currently, we track downloads and referrals, but there is no explicit system to generate unique Team Invite Links for team-based scaling. Adding this functionality directly helps team onboarding and product adoption.

## Research Report
Adding `TeamInvite` functionality matches our growth objectives. The endpoint will handle GET and POST requests.

## Design Doc
Create `TeamInvite` struct in `handlers_growth.go`.
Add `s.invites` to `Server`.
Add `handleTeamInvites` in `handlers_growth.go`.
Add `/api/growth/invites` in `server.go`.
Add tests in `handlers_growth_test.go`.

## Implementation Prompt
Implement the team invite endpoint and associate it with growth loops.
