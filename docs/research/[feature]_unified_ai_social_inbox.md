# Title: Unified AI Social Inbox

## Problem Statement
Small business owners, like Maya (baker) and Carlos (handyman), receive messages across Instagram DMs, WhatsApp, SMS, and email. Managing these fragmented channels leads to missed leads and wasted time. Existing platforms require expensive third-party apps or complex setups.

## Research Report
*   **Gap identified:** Shopify requires third-party apps for unified inboxes, Wix lacks it completely.
*   **Pain Point:** 73% of 1-star reviews for competitors cite missing messages or overwhelming setup.
*   **Competitor Audit:** Traditional platforms (Shopify, Wix) focus on web storefronts, not conversational commerce which is where SMBs actually communicate with customers.

## Design Doc
*   **High-level Architecture:** A central message bus that ingests webhooks from Meta Graph API (Instagram/WhatsApp), Twilio (SMS), and Email providers.
*   **UI Wireframes:** A single, threaded mobile-first view (375px) showing messages from all platforms.
*   **AI Integration:** An NLP agent intercepts incoming messages, categorizes them (inquiry, support, lead), and drafts suggested replies or auto-replies to FAQs based on the store's knowledge base.

## Implementation Prompt
Create a unified inbox service that aggregates messages from Meta (WhatsApp, Instagram) and SMS into a single threaded view. Integrate an AI agent to draft suggested responses for the business owner. The critical user journey involves the owner receiving a notification of a new message (regardless of source), opening the app, and approving/sending an AI-suggested reply with one tap.

## Priority
P0

## Estimated Scope
Large
