---
status: DONE
agent: Nova
priority: P0
scope: Medium
---

# Title: Proactive Implementer Growth Improvements: Viral Referral Quota System

## Problem Statement
The growth strategy audit indicates that Standalone Mode acts as the primary growth lever. We need to continuously improve OHC's viral loops and referral systems (as per the Nova Principal Growth Engineer role).

## Research Report
The `docs/growth_strategy_audit.md` likely indicates we need to focus on adding mechanisms that reward users for inviting others. We need a free-tier quota system that expands when referrals are successful.

## Design Doc
1. Implement a Dart UI component to show free-tier quota and the ability to expand it via referrals.
2. Ensure this adheres to the Aesthetic Excellence requirement (Glassmorphism, etc).

## Implementation Prompt
Implement a Free-Tier Quota display widget in the Dart frontend that includes a prominent "Invite Team to Expand Quota" call-to-action button, ensuring the visual styling adheres to OHC-SIP (glassmorphism, etc).
