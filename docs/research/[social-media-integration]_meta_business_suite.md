# Social Media Integration: Meta Business Suite (Instagram & Facebook DMs)

## Problem Statement
Small business owners miss potential sales because they don't check their Instagram DMs, Facebook comments, or WhatsApp messages fast enough. They are too busy running their business to check 3 different apps.

## Research Report
Meta provides an official Graph API that covers Facebook Pages, Instagram Professional accounts, and WhatsApp Business. It's the standard for unifying messages.
- *Ease of Use*: High for the user (standard OAuth login), but complex to set up Meta app review on the backend.
- *Pricing*: Free for standard messaging API limits. WhatsApp Business has per-conversation pricing after a free tier.
- *Reputation*: Industry standard. Reliable webhooks but strict policy constraints (e.g. 24-hour reply window).

## Design Doc
- *Trigger*: User connects their Facebook/Instagram account via an OAuth modal in OHC settings.
- *Action*: OHC subscribes to webhooks for new messages and comments. Incoming messages are routed into the OHC Unified Inbox.
- *User Interface*: A new "Social Channels" tab in Settings. The Unified Inbox will show a small icon (Instagram, Facebook) next to incoming messages. The user can reply directly from OHC.

## Implementation Prompt
Implement an OAuth connection flow for Meta Business Suite and a webhook handler that routes incoming DMs/comments into the existing OHC Unified Inbox. The user should be able to click "Connect Facebook/Instagram" in Settings, authenticate, and then immediately start seeing and replying to their social media messages within the OHC Inbox.

## Priority
P0

## Estimated Scope
Large

## Environment Support
Cloud (Webhooks), Standalone (Requires ngrok/tunneling or long-polling if possible, else Cloud-only feature).
