# Social Media Integration - ManyChat

## Problem Statement
Small business owners get overwhelmed by messages from Instagram, Facebook, and WhatsApp. Replying manually takes too much time, and they lose sales because they can't reply fast enough while running their business. They need an automated way to handle DMs and comments.

## Research Report
ManyChat is a leading chat marketing platform. It integrates well with Meta products (Instagram, Messenger, WhatsApp).
- **Ease of Use**: Very visual, drag-and-drop flow builder. Non-technical users can set up basic auto-replies quickly.
- **Pricing**: Free tier available (up to 1,000 contacts, basic features). Pro tier starts at $15/month, which is affordable for small businesses.
- **Reputation**: Highly regarded in the e-commerce space.
- **Cloud/Standalone**: Primarily cloud-based SaaS.

## Design Doc
- **Trigger**: User connects their Instagram/Facebook account to OHC.
- **Action**: OHC sets up webhooks with ManyChat to listen for incoming DMs and comments.
- **User View**: Business owner sees a unified "Inbox" in the OHC app. They can turn on "Auto-Reply" agents that use OHC's AI to draft responses, pushed back out through ManyChat's API.

## Implementation Prompt
Integrate ManyChat to allow users to connect their Instagram and Facebook accounts. Create a unified Inbox view in the OHC app where users can read and respond to messages. Add a toggle to enable an AI auto-reply agent for these channels.
- Acceptance Criteria: User can OAuth connect ManyChat. Incoming DMs appear in the OHC inbox. User can reply from OHC, and it sends to Instagram.

## Priority
P1

## Estimated Scope
Medium

---
