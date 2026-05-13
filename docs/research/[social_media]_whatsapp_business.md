# Issue Brief: WhatsApp Business Integration

## Title
Implement WhatsApp Business Integration for OHC unified dashboard.

## Problem Statement
Small business owners struggle with context switching. Managing social media operations across multiple disparate apps leads to missed opportunities, delayed responses, and customer dissatisfaction. For non-technical users, logging into 5 different portals daily is an operational nightmare. They need a single pane of glass to oversee their business without understanding API keys or webhooks. Specifically, the burden of manually synchronizing state between their social media tool and their primary business ledger is a massive time sink.

## Research Report
### Overview
WhatsApp Business Integration represents a critical pillar in modern SMB operations. Integrating this tool allows the OHC platform to act as the central nervous system for the business.
External Data Synthesis: WhatsApp Messenger, commonly known simply as WhatsApp, is an American social media, instant messaging (IM), and Voice over IP (VoIP) service accessible via desktop and mobile app. Owned by Meta Platforms, the service allows users to send text messages, voice messages, and video messages, make voice and video calls, and share images, documents, user locations, and other content. The service requires a cellular mobile telephone number to register. WhatsApp was launched in May 2009. In January 2018, WhatsApp released a standalone business app called WhatsApp Business which can communicate with the standard WhatsApp client. As of May 2025, the service had 3 billion monthly active users, making it the most used messenger app. The name of the app is meant to sound like "what's up".
The service was created by WhatsApp Inc. of Mountain View, California, which was acquired by Facebook in February 2014 for approximately US$19.3 billion. It became the world's most popular messaging application in 2015, with 900 million users, and had more than 2 billion active users worldwide in February 2020. WhatsApp Business had approximately 200 million monthly users in 2023. By 2016, it had become the primary means of Internet communication in regions including the Americas, the Indian subcontinent, and large parts of Europe and Africa.

### Ease of Use (Non-Technical Persona)
The target audience (e.g., Fatima the baker, Jose the plumber) requires zero-configuration setups. We analyzed the onboarding flows of WhatsApp Business Integration. The standard OAuth 2.0 flow is acceptable, provided we abstract away token refresh mechanisms. The UI must speak plain language: "Connect your account" instead of "Authorize Application". Furthermore, error states must be translated from technical API responses (e.g., '401 Unauthorized') into actionable advice ('Your connection expired. Please log in again.').

### Pricing & Business Model
Conversation-based pricing; varying rates depending on user-initiated vs. business-initiated chats.
Based on market standards for social_media, pricing models generally involve either per-transaction fees or monthly tiered subscriptions. For our Standalone users, we must ensure the free-tier API limits of WhatsApp Business Integration are sufficient for a single small business. For Cloud users, we must negotiate enterprise rates or utilize sub-merchant routing to avoid aggregate rate limits.

### Competitive Analysis
When compared to alternatives in the social_media space, WhatsApp Business Integration holds significant market share, making it a high-confidence integration target.
- **Pros**: High reliability, widespread consumer trust, extensive documentation. Deeply embedded in LATAM, India, and European markets; acts as the primary internet interface for many users.
- **Cons**: Potential data lock-in, rate limiting on free tiers, complex error states. Template message approval process can be cumbersome for dynamic promotional content.
- **Context**: Competes directly with standard SMS but offers richer media capabilities and global reach.

### Mode Compatibility
- **Cloud Mode**: Fully compatible. Webhooks can be routed through our standard ingestion pipeline. Multi-tenant isolation is standard. We will utilize our Redis cluster to handle bursty webhook traffic to ensure no payloads are dropped during high-volume events.
- **Standalone Mode**: Compatible, but requires local polling or secure tunneling if webhooks are strictly required by WhatsApp Business Integration. Local encrypted SQLite storage is sufficient for caching. We must ensure that the polling interval is tuned to balance responsiveness with API rate limits, defaulting to a 5-minute jittered interval.

### Security and Privacy Implications
Integrating WhatsApp Business Integration requires careful handling of PII (Personally Identifiable Information). We must ensure that OAuth scopes requested are the absolute minimum necessary to fulfill the integration's purpose (Principle of Least Privilege). In Standalone mode, OAuth tokens must be encrypted at rest within the SQLite database using go-sqlcipher. In Cloud mode, tokens must be securely stored in our vault infrastructure, completely isolated per tenant.

### Onboarding Heuristics
To ensure successful adoption, the integration must include a 'test connection' heuristic. Immediately after the user connects WhatsApp Business Integration, the platform should perform a silent API call to fetch a trivial piece of data (e.g., account profile name) to verify the connection is healthy. If this fails, the user should be immediately notified and guided through troubleshooting steps, rather than discovering the failure days later when expected data is missing.

## Design Doc
### User Experience
1. User navigates to Settings > Integrations in the OHC dashboard.
2. User sees a card for WhatsApp Business Integration highlighting the core value proposition.
3. User clicks "Connect WhatsApp Business Integration".
4. A secure popup handles authentication.
5. Upon return, the status immediately reflects "Connected" with a green indicator.
6. Relevant data (e.g., new messages, calendar events, payment statuses) begins flowing into the unified activity feed.

### Integration Flow
- **Trigger**: User OAuth completion.
- **Action**: OHC platform stores the connection securely and begins bidirectional sync.
- **Visibility**: The user sees plain-language updates in their main feed, like "New appointment booked via WhatsApp Business Integration".

## Implementation Prompt
Implement the WhatsApp Business Integration integration. Ensure the setup flow is accessible to non-technical users. The final outcome must be a fully connected state where data flows seamlessly into the user's unified dashboard. Use plain language for all labels and error messages. Ensure compatibility for both Cloud (PostgreSQL/Redis) and Standalone (SQLite) modes. Follow OHC Premium Design Standards for all UI elements (Outfit font, Inter body, glassmorphism). Do not implement complex retry logic in the initial PR; focus on establishing the core connection and data ingest pipeline.

## Priority
P1

## Estimated Scope
Medium
