## [Social Media] Instagram Direct Message Integration
**Title**: Integrate Instagram Direct Messages via Meta API
**Problem Statement**: Managing Instagram DMs natively is inefficient for small business owners and siloed from other business operations, leading to missed messages and slow response times.
**Research Report**:
- **Tool**: Meta Messenger API for Instagram
- **Target Persona**: Small business owners (e.g., Boutique Owners, Bakers)
- **Advantages**: Implements a unified inbox channel, allowing owners to reply directly from OHC. Centralizing communications is a massive value-add.
- **Risks**: Cloud is straightforward via webhooks, but Standalone requires a polling or relay strategy.
- **Pricing**: Standard Meta API pricing / limits.
- **Compatibility**: Cloud (Webhooks), Standalone (Polling/Relay Strategy).
**Design Doc**:
- User authenticates via Meta OAuth.
- OHC registers webhooks to receive new DMs (in Cloud mode).
- DMs are populated in the unified OHC inbox.
- User can reply from OHC, which sends messages back via Meta API.
**Implementation Prompt**: Create an OAuth flow to connect an Instagram account via the Meta API. Implement a webhook handler to receive incoming messages and an API client to send replies from the unified inbox.
**Priority**: P1
**Estimated Scope**: Medium
