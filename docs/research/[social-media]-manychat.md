# Title: Integrate ManyChat for Unified Social Media Inbox

## Problem Statement
Small business owners receive customer inquiries across Instagram DMs, Facebook comments, and WhatsApp. Juggling multiple apps leads to missed messages, slow response times, and lost sales. They need a single, unified inbox to view and reply to all social messages easily without technical setup.

## Research Report
ManyChat offers a robust API for connecting Instagram, Facebook Messenger, and WhatsApp. It's designed for marketing automation but serves excellently as a unified messaging hub.
- **Ease of use:** High. Uses standard OAuth flows for business owners to connect their pages.
- **Pricing:** Freemium model; Pro tier starts at $15/mo which is affordable for most small businesses.
- **Reputation:** Market leader in social messaging automation.
- **Cloud/Standalone:** Works in both Cloud (webhook routing) and Standalone (direct webhooks if exposed, or via a cloud relay) modes.

## Design Doc
- **Trigger:** Business owner connects their Facebook/Instagram account via ManyChat OAuth from the OHC Settings page.
- **Action:** Incoming messages to connected social channels trigger a webhook to OHC. OHC stores the message and alerts the user.
- **User Interface:** A new "Unified Inbox" tab in the dashboard showing threaded conversations from all channels. Users can type a reply which is sent back via the ManyChat API.

## Implementation Prompt
Create a "Unified Inbox" feature where users can connect their social media accounts using a one-click OAuth button. Once connected, all incoming messages from Instagram, Facebook, and WhatsApp should appear in a single threaded view. When the user replies, the message should be delivered back to the customer on their original platform.

## Priority
P1

## Estimated Scope
Medium
