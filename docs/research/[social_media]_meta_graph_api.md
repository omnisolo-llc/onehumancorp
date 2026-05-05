## [Social Media] Meta Graph API Integration
**Title**: Integrate Meta Graph API for Unified Native Social Media Inbox
**Problem Statement**: Small business owners like Maya (The Home Baker) receive orders and inquiries across Instagram DMs, Facebook Messenger, and WhatsApp. Managing these manually is overwhelming and leads to missed sales. They need a single, unified inbox where an AI agent can read and reply to messages from all platforms automatically, maintaining the Radical Simplicity ethos by avoiding complex third-party tools like Manychat.
**Research Report**:
- **Strategy**: Direct integration with Meta Graph API
- **Target Persona**: Maya (Home Baker), Priya (Boutique Owner)
- **Advantages**: No third-party SaaS fees, maintains Radical Simplicity. Direct, deep integration tailored specifically for OHC's unified inbox UI without extraneous features.
- **Risks**: Requires building and maintaining the OAuth flow and webhook handlers directly. Meta's API reviews can be stringent.
- **Pricing**: Free API usage.
- **Compatibility**: Cloud (via webhooks/OAuth). Standalone (requires routing via a lightweight cloud proxy).
**Design Doc**:
- User goes to the Operations dashboard and clicks "Connect Instagram".
- User authenticates with Facebook/Instagram via OAuth.
- OHC registers webhooks to receive new DMs.
- When a DM arrives, the Customer Success agent reads it, generates a reply (e.g., "Yes, we do vegan cakes!"), and sends it back via Meta Graph API.
- The user sees a unified "Customer Inbox" on their phone showing the conversation history.
- **AI Integration**: The Customer Success Agent ("The Ambassador") listens to the incoming webhook queue, generates draft responses for unread messages based on the business's knowledge base, and auto-replies if the user enables "Auto-Pilot".
**Implementation Prompt**: Implement a direct Meta Graph API OAuth flow. Create a native webhook endpoint that receives incoming messages, stores them in the OHC unified inbox, and triggers the Customer Success agent to draft a reply.
**Priority**: P0
**Estimated Scope**: Large
