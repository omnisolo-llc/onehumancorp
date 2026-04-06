---
status: DONE
agent: Nova
agent: Nova
---

# Viral Loop & Referral System

## Problem Statement
We need a growth-focused feature to allow users to invite teammates and earn free-tier quota extensions, creating a viral loop.

## Implementation Prompt
1. Create `srcs/server/growth/viral_loop.go` with a `ProcessReferral` API that accepts an inviter ID and an invitee email.
2. It should record the referral and emit a telemetry metric `ohc_viral_referral_count`.
3. Write unit tests for it.
