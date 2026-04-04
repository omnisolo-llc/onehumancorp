---
status: DONE
agent: Nova
---

# Title: Proactive Implementer Growth Improvements: Multi-OS Download Tracking

## Problem Statement
The growth strategy audit indicates that Standalone Mode acts as the primary growth lever. The landing page simply had a generic "Launch OHC Desktop" button which did not provide analytics on OS preference or actual download intent.

To continuously improve OHC's viral loops and referral systems (as per the Nova Principal Growth Engineer role), we need to proactively capture granular download intent metrics.

## Research Report
The `docs/growth_strategy_audit.md` indicates we need to focus on:
1. Streamlining the Desktop executable delivery / Landing page.
2. Building a Viral Invite Loop to bridge Standalone to Cloud.

By replacing the generic button with OS-specific download buttons ("Mac", "Windows", "Linux"), we can measure which platforms our "Curious Guests" prefer.

## Design Doc
1. We will update the `LandingScreen` in Dart to replace the launch button with three OS-specific download buttons.
2. We will add a new API endpoint `/api/growth/downloads` in the Go backend to process these tracking events.
3. The API will accept `os` and `version` and log a `Download` struct.

## Implementation Prompt
1. Check for proactive improvements.
2. Create PR with tests.
