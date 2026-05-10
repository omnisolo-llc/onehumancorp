# Unified Social Media Inbox for DMs and Comments

## Problem Statement
Small business owners struggle to keep up with customer messages scattered across Instagram, Facebook, WhatsApp, and TikTok. They often miss sales inquiries or take too long to reply because they have to constantly switch between apps.

## Research Report
Evaluated connecting Instagram DMs, Facebook comments, WhatsApp messages, and TikTok comments.

- **Ease of Use**: A unified inbox saves hours a week and prevents missed leads.
- **Pricing**: Most platforms charge $15-$50/mo for a unified inbox. OHC can build a compelling integrated solution.
- **Risks**: High OAuth complexity and webhook reliability. Meta's API reviews can be strict.
- **Modes**: Works well in both Cloud and Standalone (with appropriate public SaaS integration apps).

## Design Doc
When a customer sends a message on Instagram/Facebook/WhatsApp/TikTok, a webhook triggers OHC. OHC creates a unified conversation thread in the user's dashboard. The business owner replies from OHC, and the platform sends the reply back to the respective social platform via API.

## Implementation Prompt
Create a unified inbox UI where messages from different social channels appear in a single chronological feed. Allow the user to reply directly from this feed, and ensure messages are routed back to the correct original platform.

## Priority
P0

## Estimated Scope
Large
