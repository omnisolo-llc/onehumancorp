# Title: Unify All Customer Messages into One Inbox with Respond.io

## Problem Statement
Small business owners often miss customer messages because they are scattered across Instagram, Facebook, WhatsApp, and TikTok. Checking multiple apps takes time, and forgotten messages mean lost sales and unhappy customers. Business owners need a single, simple inbox where they can see and reply to every customer, no matter which app the customer used to reach out.

## Research Report
Respond.io is a leading customer conversation management platform designed to unify messaging.
- **Ease of Use**: Excellent for non-technical users. The interface looks like a standard chat app (like WhatsApp or Messenger), making it instantly familiar. Connecting channels is done via standard OAuth flows.
- **Pricing**: Plans start at around $79/month, which may be steep for very small businesses, but the value of unifying multiple high-traffic channels often justifies the cost. They offer a comprehensive free trial.
- **Reputation**: Highly rated for reliability, especially with WhatsApp Business API and Instagram DMs. Their webhook reliability is top-tier, ensuring messages aren't lost.
- **Comparison**: Compared to alternatives like ManyChat or Chatfuel (which focus heavily on bots), Respond.io focuses on the shared inbox experience, which is exactly what our users need first.
- **Cloud vs Standalone**: This tool relies on webhooks and external APIs, meaning it is perfectly suited for Cloud mode but will require ngrok or a similar tunnel to function in Standalone mode.

## Design Doc
- **Triggers & Actions**: When a customer sends a message on any connected social platform, Respond.io pushes this message to OHC. OHC displays it in a unified "Inbox" tab. When the business owner replies in OHC, the message is routed back through Respond.io to the correct platform and customer.
- **User Experience**: The user will see an "Inbox" tab in their OHC dashboard. In the "App Settings", they will find a "Connect Social Media" section where they can click simple buttons like "Connect Instagram" or "Connect WhatsApp". Once connected, all new messages appear in the OHC Inbox like a normal chat thread.

## Implementation Prompt
Create a unified "Inbox" feature in the OHC dashboard.
- **User-Facing Outcome**: The business owner should be able to go to a single "Inbox" page to see conversations from Instagram, Facebook, and WhatsApp. They must be able to read messages and type replies directly within OHC.
- **Acceptance Criteria**:
  - A settings page exists to connect various social platforms using plain language (e.g., "Connect Instagram").
  - Messages received from a connected platform appear in the OHC Inbox in real-time.
  - Replies sent from the OHC Inbox are successfully delivered to the customer on their original platform.
  - The UI must be simple, resembling a standard chat application.

## Priority
P1

## Estimated Scope
Large
