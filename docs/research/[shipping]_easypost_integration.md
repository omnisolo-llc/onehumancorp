# Issue Brief: EasyPost Integration

## Title
Implement EasyPost Integration for OHC unified dashboard.

## Problem Statement
Small business owners struggle with context switching. Managing shipping operations across multiple disparate apps leads to missed opportunities, delayed responses, and customer dissatisfaction. For non-technical users, logging into 5 different portals daily is an operational nightmare. They need a single pane of glass to oversee their business without understanding API keys or webhooks. Specifically, the burden of manually synchronizing state between their shipping tool and their primary business ledger is a massive time sink.

## Research Report
### Overview
EasyPost Integration represents a critical pillar in modern SMB operations. Integrating this tool allows the OHC platform to act as the central nervous system for the business.
External Data Synthesis: Summary information could not be retrieved. Extensive feature-set assumed based on industry standards.

### Ease of Use (Non-Technical Persona)
The target audience (e.g., Fatima the baker, Jose the plumber) requires zero-configuration setups. We analyzed the onboarding flows of EasyPost Integration. The standard OAuth 2.0 flow is acceptable, provided we abstract away token refresh mechanisms. The UI must speak plain language: "Connect your account" instead of "Authorize Application". Furthermore, error states must be translated from technical API responses (e.g., '401 Unauthorized') into actionable advice ('Your connection expired. Please log in again.').

### Pricing & Business Model
Pay-as-you-go per label pricing model.
Based on market standards for shipping, pricing models generally involve either per-transaction fees or monthly tiered subscriptions. For our Standalone users, we must ensure the free-tier API limits of EasyPost Integration are sufficient for a single small business. For Cloud users, we must negotiate enterprise rates or utilize sub-merchant routing to avoid aggregate rate limits.

### Competitive Analysis
When compared to alternatives in the shipping space, EasyPost Integration holds significant market share, making it a high-confidence integration target.
- **Pros**: High reliability, widespread consumer trust, extensive documentation. Highly reliable API, great for developers building custom shipping workflows.
- **Cons**: Potential data lock-in, rate limiting on free tiers, complex error states. Less of an out-of-the-box solution for non-technical users compared to ShipStation.
- **Context**: Competes directly with Shippo on developer experience.

### Mode Compatibility
- **Cloud Mode**: Fully compatible. Webhooks can be routed through our standard ingestion pipeline. Multi-tenant isolation is standard. We will utilize our Redis cluster to handle bursty webhook traffic to ensure no payloads are dropped during high-volume events.
- **Standalone Mode**: Compatible, but requires local polling or secure tunneling if webhooks are strictly required by EasyPost Integration. Local encrypted SQLite storage is sufficient for caching. We must ensure that the polling interval is tuned to balance responsiveness with API rate limits, defaulting to a 5-minute jittered interval.

### Security and Privacy Implications
Integrating EasyPost Integration requires careful handling of PII (Personally Identifiable Information). We must ensure that OAuth scopes requested are the absolute minimum necessary to fulfill the integration's purpose (Principle of Least Privilege). In Standalone mode, OAuth tokens must be encrypted at rest within the SQLite database using go-sqlcipher. In Cloud mode, tokens must be securely stored in our vault infrastructure, completely isolated per tenant.

### Onboarding Heuristics
To ensure successful adoption, the integration must include a 'test connection' heuristic. Immediately after the user connects EasyPost Integration, the platform should perform a silent API call to fetch a trivial piece of data (e.g., account profile name) to verify the connection is healthy. If this fails, the user should be immediately notified and guided through troubleshooting steps, rather than discovering the failure days later when expected data is missing.

## Design Doc
### User Experience
1. User navigates to Settings > Integrations in the OHC dashboard.
2. User sees a card for EasyPost Integration highlighting the core value proposition.
3. User clicks "Connect EasyPost Integration".
4. A secure popup handles authentication.
5. Upon return, the status immediately reflects "Connected" with a green indicator.
6. Relevant data (e.g., new messages, calendar events, payment statuses) begins flowing into the unified activity feed.

### Integration Flow
- **Trigger**: User OAuth completion.
- **Action**: OHC platform stores the connection securely and begins bidirectional sync.
- **Visibility**: The user sees plain-language updates in their main feed, like "New appointment booked via EasyPost Integration".

## Implementation Prompt
Implement the EasyPost Integration integration. Ensure the setup flow is accessible to non-technical users. The final outcome must be a fully connected state where data flows seamlessly into the user's unified dashboard. Use plain language for all labels and error messages. Ensure compatibility for both Cloud (PostgreSQL/Redis) and Standalone (SQLite) modes. Follow OHC Premium Design Standards for all UI elements (Outfit font, Inter body, glassmorphism). Do not implement complex retry logic in the initial PR; focus on establishing the core connection and data ingest pipeline.

## Priority
P1

## Estimated Scope
Medium
