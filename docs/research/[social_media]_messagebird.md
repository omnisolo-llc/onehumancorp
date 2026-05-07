# Title: Implement Unified Inbox via MessageBird API

**Problem Statement:** Users like Maya (Baker) manage orders across Instagram, WhatsApp, and email, leading to missed messages and lost revenue. A unified inbox is needed.

**Research Report:** MessageBird (Bird AI) provides robust APIs for Email, SMS, and WhatsApp marketing and unified messaging. It allows businesses to handle Omni-channel communication seamlessly. It's a strong fit for Maya's Instagram DMs and WhatsApp inquiries.

**Design Doc:** Integrate MessageBird API with the OHC backend. Webhooks from MessageBird will alert the OHC backend of new messages. The frontend will present a unified "Inbox" UI for the user aggregating all channels. "The Ambassador" AI agent will draft replies within this interface.

**Implementation Prompt:** Add a unified inbox view in the Flutter app that shows messages from Instagram and WhatsApp via MessageBird. Users should be able to reply directly from the app. Include AI-drafted responses.

**Priority:** P0

**Estimated Scope:** Large
