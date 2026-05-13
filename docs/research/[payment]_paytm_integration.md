# Issue Brief: Paytm Integration

## Title
Implement Paytm Integration for OHC unified dashboard.

## Problem Statement
Small business owners struggle with context switching. Managing payment operations across multiple disparate apps leads to missed opportunities, delayed responses, and customer dissatisfaction. For non-technical users, logging into 5 different portals daily is an operational nightmare. They need a single pane of glass to oversee their business without understanding API keys or webhooks. Specifically, the burden of manually synchronizing state between their payment tool and their primary business ledger is a massive time sink.

## Research Report
### Overview
Paytm Integration represents a critical pillar in modern SMB operations. Integrating this tool allows the OHC platform to act as the central nervous system for the business.
External Data Synthesis: Paytm (short for Pay Through Mobile) is an Indian multinational financial technology company, that specializes in digital payments and financial services, based in Noida. Paytm was founded in 2010 by Vijay Shekhar Sharma under One97 Communications. The company offers mobile payment services to consumers and enables merchants to receive payments through QR code payment, Soundbox, Android-based-payment terminal, and online payment gateway. In partnership with financial institutions, Paytm also offers financial services such as microcredit and buy now, pay later to its consumers and merchants.
Apart from bill payments and money transfer, the company also provides ticketing services, retail brokerage products and online games.
Paytm's parent company One97 Communications was listed on the Indian stock exchanges on 18 November 2021 after an initial public offering, which was the largest in India at the time. For the fiscal year 2022–23, Paytm's gross merchandise value (GMV) was reported to be ₹13.2 lakh crore (US$140 billion).

### Ease of Use (Non-Technical Persona)
The target audience (e.g., Fatima the baker, Jose the plumber) requires zero-configuration setups. We analyzed the onboarding flows of Paytm Integration. The standard OAuth 2.0 flow is acceptable, provided we abstract away token refresh mechanisms. The UI must speak plain language: "Connect your account" instead of "Authorize Application". Furthermore, error states must be translated from technical API responses (e.g., '401 Unauthorized') into actionable advice ('Your connection expired. Please log in again.').

### Pricing & Business Model
Low transaction fees for UPI; standard percentage for credit cards.
Based on market standards for payment, pricing models generally involve either per-transaction fees or monthly tiered subscriptions. For our Standalone users, we must ensure the free-tier API limits of Paytm Integration are sufficient for a single small business. For Cloud users, we must negotiate enterprise rates or utilize sub-merchant routing to avoid aggregate rate limits.

### Competitive Analysis
When compared to alternatives in the payment space, Paytm Integration holds significant market share, making it a high-confidence integration target.
- **Pros**: High reliability, widespread consumer trust, extensive documentation. Deep market penetration in India; supports UPI which is the backbone of Indian digital payments.
- **Cons**: Potential data lock-in, rate limiting on free tiers, complex error states. Regulatory environment in India is highly volatile; integration requires strict KYC compliance.
- **Context**: Competes fiercely with PhonePe and Google Pay in the region.

### Mode Compatibility
- **Cloud Mode**: Fully compatible. Webhooks can be routed through our standard ingestion pipeline. Multi-tenant isolation is standard. We will utilize our Redis cluster to handle bursty webhook traffic to ensure no payloads are dropped during high-volume events.
- **Standalone Mode**: Compatible, but requires local polling or secure tunneling if webhooks are strictly required by Paytm Integration. Local encrypted SQLite storage is sufficient for caching. We must ensure that the polling interval is tuned to balance responsiveness with API rate limits, defaulting to a 5-minute jittered interval.

### Security and Privacy Implications
Integrating Paytm Integration requires careful handling of PII (Personally Identifiable Information). We must ensure that OAuth scopes requested are the absolute minimum necessary to fulfill the integration's purpose (Principle of Least Privilege). In Standalone mode, OAuth tokens must be encrypted at rest within the SQLite database using go-sqlcipher. In Cloud mode, tokens must be securely stored in our vault infrastructure, completely isolated per tenant.

### Onboarding Heuristics
To ensure successful adoption, the integration must include a 'test connection' heuristic. Immediately after the user connects Paytm Integration, the platform should perform a silent API call to fetch a trivial piece of data (e.g., account profile name) to verify the connection is healthy. If this fails, the user should be immediately notified and guided through troubleshooting steps, rather than discovering the failure days later when expected data is missing.

## Design Doc
### User Experience
1. User navigates to Settings > Integrations in the OHC dashboard.
2. User sees a card for Paytm Integration highlighting the core value proposition.
3. User clicks "Connect Paytm Integration".
4. A secure popup handles authentication.
5. Upon return, the status immediately reflects "Connected" with a green indicator.
6. Relevant data (e.g., new messages, calendar events, payment statuses) begins flowing into the unified activity feed.

### Integration Flow
- **Trigger**: User OAuth completion.
- **Action**: OHC platform stores the connection securely and begins bidirectional sync.
- **Visibility**: The user sees plain-language updates in their main feed, like "New appointment booked via Paytm Integration".

## Implementation Prompt
Implement the Paytm Integration integration. Ensure the setup flow is accessible to non-technical users. The final outcome must be a fully connected state where data flows seamlessly into the user's unified dashboard. Use plain language for all labels and error messages. Ensure compatibility for both Cloud (PostgreSQL/Redis) and Standalone (SQLite) modes. Follow OHC Premium Design Standards for all UI elements (Outfit font, Inter body, glassmorphism). Do not implement complex retry logic in the initial PR; focus on establishing the core connection and data ingest pipeline.

## Priority
P1

## Estimated Scope
Medium
