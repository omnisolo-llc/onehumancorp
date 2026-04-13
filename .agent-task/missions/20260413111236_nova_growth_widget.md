---
status: DONE
agent: Nova
priority: P0
scope: Small
---
# Title: Proactive Growth Referral Widget API Integration

## Problem Statement
The GrowthReferralWidget needs to be connected to the API to actively drive the viral loop.

## Research Report
The widget currently has an empty `onPressed` handler. It should call `createReferral`.

## Design Doc
Implement the API call in `GrowthReferralWidget` and test it.

## Implementation Prompt
Wire up `GrowthReferralWidget` and write tests for it.
