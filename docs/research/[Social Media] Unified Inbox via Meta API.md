# Title: Unified Inbox Integration for Instagram, Facebook, and WhatsApp
## Problem Statement
Small business owners struggle to keep up with customer inquiries scattered across Instagram DMs, Facebook comments, and WhatsApp messages. Constantly switching apps leads to missed sales, slow responses, and a chaotic workflow for non-technical users who just want to see all messages in one place.

## Research Report
* **Tool:** Meta Cloud API (Instagram, Messenger, WhatsApp)
* **What it does:** Allows fetching, replying to, and managing messages from all Meta-owned platforms.
* **Ease of Use for Owners:** High once connected. The owner clicks "Connect Facebook," logs in, and OHC handles the rest. No developer keys needed on their end if we use OHC Cloud's unified OAuth app.
* **Pricing:** WhatsApp charges per conversation (first 1,000 free/month). Instagram/FB are free.
* **Cloud vs. Standalone:**
  * Cloud: OHC acts as the registered Meta App, simplifying OAuth.
  * Standalone: Requires users to either use an OHC proxy or create their own Meta Developer account (high friction). The proxy approach is recommended.

## Design Doc
* **Trigger:** User navigates to Settings > Integrations and clicks "Connect Meta".
* **Action:** OHC initiates OAuth flow. Once approved, OHC subscribes to webhooks for incoming messages.
* **User Experience:** A new "Unified Inbox" tab appears. Messages from all platforms show up in a single chat interface. Replies sent from OHC are routed back to the correct platform natively.

## Implementation Prompt
Implement the user-facing "Unified Inbox" feature that lets business owners connect their Meta accounts and view/reply to messages from one screen. The outcome must allow a user to successfully authenticate their account, receive an incoming test DM on Instagram, and send a reply directly from OHC that appears on the customer's phone. Ensure clear error states if the connection fails.

## Priority
P0

## Estimated Scope
Large
