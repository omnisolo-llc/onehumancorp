# Unified Inbox Integration via Meta Graph API

**Title**: Unified Inbox Integration via Meta Graph API
**Problem Statement**: Small business owners like Fatima receive customer inquiries across Instagram, Facebook, and WhatsApp, often missing messages and losing sales because they can't monitor all apps at once. They need a single place to view and respond to all customer messages.

**Research Report**:
- The Meta Graph API provides comprehensive access to Instagram DMs, Facebook comments, and WhatsApp messages.
- It has widespread adoption, making it the standard for social media integrations.
- **Ease of Use**: Once connected via a simple OAuth flow, the business owner doesn't need to interact with Meta's developer tools.
- **Pricing**: Free for standard usage; WhatsApp Business API has per-conversation pricing after the first 1,000 free tier.
- **Reputation**: Highly reliable, though subject to strict review processes.
- **Cloud vs Standalone**: Works in Cloud mode well. In Standalone mode, users might need to provide their own API credentials or use an OHC proxy.
- **Key Advantages**: Unifies the most popular communication channels.
- **Key Risks**: Meta's strict review processes and API changes.

**Design Doc**:
- The user navigates to the "Communications" tab and clicks "Connect Social Accounts."
- They are redirected to Meta's secure login to authorize OHC.
- Once connected, a "Unified Inbox" widget aggregates all incoming messages, showing the source icon next to each message. The user can reply directly from the widget, and OHC routes it back to the correct platform.

**Implementation Prompt**: Create a unified inbox interface where users can authenticate their Meta accounts (Instagram, Facebook, WhatsApp) and seamlessly read and reply to messages from one centralized dashboard.

**Priority**: P0
**Estimated Scope**: Large
