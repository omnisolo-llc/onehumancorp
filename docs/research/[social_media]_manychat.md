# [Social Media] ManyChat Unified Inbox

**Title**: Implement ManyChat for Unified Social Inbox
**Problem Statement**: Small business owners struggle to keep up with customer messages scattered across Instagram DMs, Facebook comments, WhatsApp, and TikTok. They miss sales opportunities because they cannot track all platforms from a single interface.
**Research Report**:
- **Target Persona**: Businesses relying heavily on social media marketing and DM sales (e.g., boutique clothing, bespoke services).
- **Evaluation**: ManyChat is a top-tier tool for unifying social channels. It is extremely easy for non-technical users to connect via standard OAuth. It supports Facebook, Instagram, and WhatsApp. Reputation is strong.
- **Ease of Use**: Very High.
- **Pricing**: Starts at around $15/mo for basic features, which is highly affordable.
- **Key Risks**: API rate limits from Meta, complex webhook handling for real-time messaging, reliance on Meta's ever-changing API policies.
- **Compatibility**: Works seamlessly in Cloud environments; Standalone might require manual webhook config which is too hard for SMBs.
**Design Doc**: The business owner will navigate to a "Social" tab in OHC and click "Connect ManyChat". They will authenticate via OAuth. Once connected, a unified inbox view within OHC will display all incoming messages. The user can reply directly from OHC, which sends the response back through ManyChat.
**Implementation Prompt**: Build a unified inbox interface that allows users to see and reply to messages from ManyChat. Acceptance criteria: user can connect their account, see unread messages, and reply to a message successfully.
**Priority**: P0
**Estimated Scope**: Large
