# Social Media Integration Brief

## Problem Statement
Small business owners juggle customer queries across Instagram, Facebook, and WhatsApp, often missing leads or responding late. This fragmented experience is frustrating for both the owner and the customer.

## Research Report
**Tool Evaluated:** Respond.io
**Findings:** Respond.io offers a robust unified inbox that combines major social channels. It reduces tab-switching, improves response times, and allows for automated replies. It is highly rated for its reliability and feature set.
**Pricing:** ~$29-$79/month.
**Ease of Use:** While the interface is clean, the initial OAuth and permissions setup can be complex for non-technical users.
**Risks:** Reliance on third-party APIs (Meta, TikTok) means that changes on their end could disrupt service.

## Design Doc
**Trigger:** Customer sends a message on a connected social platform.
**Action:** The message is routed to the OHC unified inbox. The business owner receives a notification.
**User Experience:** The business owner sees all messages in one centralized view, categorized by channel. They can reply directly from OHC, and the response is sent back to the customer on the original platform.

## Implementation Prompt
**Outcome:** A unified inbox interface within OHC where business owners can connect their Instagram, Facebook, and WhatsApp accounts. They should be able to view and reply to messages from all connected channels in one place.
**Acceptance Criteria:**
- User can successfully connect their accounts.
- Messages from connected platforms appear in the OHC inbox.
- Replies sent from OHC are delivered to the customer on the correct platform.
- The interface is simple and intuitive, focusing on conversation flow rather than technical details.

## Priority
P1

## Estimated Scope
Large
