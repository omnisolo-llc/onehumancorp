### Title
`[social_media]unified_inbox`: Implement Unified Social Inbox via Meta Graph API

### Problem Statement
Small business owners often miss critical customer inquiries because they are scattered across Instagram DMs, Facebook Messenger, and WhatsApp. Manually checking each app is a massive time sink. They need a single, unified inbox where all messages appear in one place, allowing them to reply to customers quickly without switching contexts.

### Research Report
- **Tool**: Meta Graph API
- **Pros**: Direct access to the largest social platforms (Instagram, Facebook, WhatsApp). High reliability and deep feature set.
- **Cons**: Complex OAuth flow, strict app review process, and frequent API changes. WhatsApp pricing involves per-conversation charges.
- **Reputation**: Industry standard, though developer experience can be frustrating due to Meta's aggressive review policies.
- **Pricing**: Free for standard APIs, WhatsApp Business API has per-conversation costs (approx. $0.01 - $0.08 depending on region).
- **Ease of Use for Non-Technical Users**: The user simply clicks "Connect Instagram" and authorizes the app. The complexity is hidden behind the scenes.
- **Modes Supported**: Cloud (webhooks) and Standalone (local polling or proxy).

### Design Doc
- **Trigger**: The business owner connects their Meta account via an OAuth flow in the OHC UI.
- **Action**: The OHC API server registers webhooks (in Cloud mode) or sets up a local polling mechanism/proxy (in Standalone mode) to receive incoming messages. These are stored in the shared PostgreSQL (Cloud) or local SQLite (Standalone).
- **User View**: A unified "Inbox" tab in the UI displaying all messages chronologically, with indicators for the source platform.

### Implementation Prompt
Implement a unified inbox feature that allows users to connect their Meta accounts. The system must ingest incoming messages from Instagram, Facebook, and WhatsApp, and present them in a single unified view. Users must be able to reply to messages directly from the OHC interface, and the responses should be routed back to the appropriate platform. Ensure the OAuth flow is seamless and clearly explains the required permissions.

### Priority
P0

### Estimated Scope
Large
