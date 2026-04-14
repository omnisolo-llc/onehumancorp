---
status: IN_PROGRESS
agent: Nova
---
<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif;">

# Title: Integrate Growth Invite Tracker into Dashboard Handlers

## Problem Statement
The OHC dashboard currently relies on in-memory arrays (`s.teamInvites`) within the `Server` struct to handle growth team invites (`POST /api/growth/team-invites` and `GET /api/growth/team-invites`). This mocks persistence and causes growth loop data to be lost upon server restarts, breaking the viral referral tracking core to OHC's product led growth motion.

## Research Report
- A `team_invites` table already exists in the database migrations (`051_team_invites.sql`).
- There is a `growth` package (`srcs/server/services/growth`) that provides an `InviteTracker` interacting with the database.
- The `dashboard` handler `handleTeamInvites` still uses `s.teamInvites = append(s.teamInvites, invite)`.

## Design Doc
We need to connect the `dashboard.Server` to the `growth.InviteTracker`.
1. Modify `dashboard.Server` to hold an instance of `*growth.InviteTracker` (we can add it to the `Server` struct or use a new integration).
2. Actually, looking at `srcs/server/dashboard/server.go`, the `Server` initialization doesn't inject the database to everything, but it has `hub` which has DB access, or we can just initialize `growth.NewInviteTracker(store.DB())` (or something similar). Wait, let's just make the persistent `handleTeamInvites` use an active DB connection.
3. Wait, maybe it's better to implement real Free-Tier Quota tracking to hit the `handleQuota` stub instead? Or maybe both. Let's do Quota since it says "Implement features that directly drive acquisition and retention (e.g., team invite flows, free-tier quotas)."

Let's do Quota instead.

**Free-Tier Quota Tracking**:
- Add `quota_usage` table.
- Implement `/api/growth/quota` handler.

Let me explore `srcs/server/services/growth/quota.go`. Does it exist?
