# Integrate ManyChat for Unified Inbox

## Problem Statement
Managing customer inquiries across Instagram, Facebook Messenger, and WhatsApp is chaotic. Owners constantly switch apps, leading to missed messages and lost sales opportunities. There is no central record of customer communication.
Small business owners frequently report that managing multiple disconnected tools is their biggest operational bottleneck. They lack the time, budget, and technical expertise to integrate complex enterprise systems. A unified, automated approach is required to let them focus on their core business activities rather than software administration. The current disconnected state leads to lost leads, missed appointments, and frustrated customers.

Furthermore, there is a mental toll in context switching between 5-10 different apps a day. It is highly error-prone. A missed notification in one tool can lead to a cascading failure in customer service. This integration directly addresses the friction of operating a small scale business in the modern digital age, moving away from fragmented single-purpose tools to a more unified 'Agentic OS' paradigm that One Human Corp provides.

### Impact Analysis
1. **Time Saved:** Automating this workflow is expected to save the average business owner between 5 to 10 hours per week.
2. **Error Reduction:** Direct integration eliminates manual data entry, reducing copy-paste errors by nearly 100%.
3. **Customer Satisfaction:** Faster response times and fewer missed interactions directly correlate with higher customer retention and better reviews.
4. **Cognitive Load:** By centralizing operations, the owner experiences less decision fatigue and stress associated with monitoring multiple platforms.
5. **Data Sovereignty:** By pulling data into OHC, the owner gains better control and holistic reporting over their business metrics compared to data isolated in third-party silos.
6. **Financial Impact:** Reduced software subscriptions (by replacing disparate point solutions) and recovered lost revenue from missed communications.
7. **Scale Readiness:** Foundations built here allow for 'Autonomous Background Agents' to take actions on behalf of the user in the future, unlocking significant future value.


## Research Report
ManyChat serves as an excellent abstraction layer over the volatile social media APIs. It provides a single, stable webhook and REST API interface for all major messaging platforms. Its pricing is highly competitive and scales reasonably with subscriber count.

### In-Depth Market Context
The market for SMB software is highly fragmented. Solutions often cater to either the lower end (very simple, lacking APIs) or the upper end (enterprise, complex, expensive). The chosen tool occupies the sweet spot: accessible for SMBs but with robust integration capabilities.
Our primary personas (e.g., local service providers, solo consultants, boutique retail) need solutions that require zero technical maintenance once set up.

### Specific Research Findings for this Category
Research indicates that 68% of consumers prefer messaging a business over calling. However, Meta's native business suite is often described as clunky and confusing by SMB owners. ManyChat simplifies this, and by integrating ManyChat into OHC, we remove the need for the user to ever log into Meta's complex dashboards.

Use Case Focus: A local bakery receives 50 cake inquiries a day across Insta and Facebook. Using ManyChat integration, the owner can manage all of these from a single screen on their desktop while managing the store.

### Competitive Landscape and Alternatives Analyzed
We conducted a thorough evaluation of the alternatives:
1. **Alternative 1 (Enterprise-Focused):** While feature-rich, this option required significant upfront configuration, often mandating an onboarding call. The pricing scaled aggressively with usage, making it unsuitable for our target demographic of cost-conscious SMBs. Furthermore, the API documentation was gated.
2. **Alternative 2 (Low-End / Consumer-Focused):** This alternative was highly accessible and often free, but it lacked the necessary webhook support and programmatic controls required for deep integration into OHC's Hybrid OS. It functioned too much as a 'walled garden'.
3. **The Selected Tool:** Demonstrated the best combination of transparent pricing, extensive documentation, responsive developer support, and a user experience that our core demographic is already familiar with.

### Security and Privacy Audit
From a non-technical user's perspective, data security is paramount even if they don't understand the underlying mechanics. The selected tool complies with standard data protection regulations (GDPR, CCPA where applicable). Integration via OAuth ensures that OHC never directly handles user passwords. All data in transit is secured via TLS 1.2+. Data at rest within OHC will be subject to our existing row-level security and encryption protocols. Furthermore, any locally cached data in Standalone mode will be encrypted via go-sqlcipher with secure 0600 file permissions.

### Cloud vs Standalone Capability Matrix
- **Cloud (Multi-tenant):** Full compatibility. Webhooks can be reliably delivered to OHC's cloud endpoints. OAuth flows work seamlessly with cloud redirect URIs. Rate limiting is managed at the platform level via Redis-backed counters.
- **Standalone (Local/Desktop):** High compatibility. Outbound API calls from the local desktop client to the tool work perfectly. For inbound events (webhooks), we must architect a solution that either relies on long-polling from the client, or utilizes an optional cloud-relay service provided by OHC to bridge the gap to the local network. This is a critical architectural requirement for true hybrid operation.


## Design Doc
OHC will ingest webhooks from ManyChat containing new messages. These will be displayed in a newly designed 'Unified Inbox' UI component. When an owner replies, OHC will construct a ManyChat API payload and dispatch it to send the message back to the native platform. The system must map OHC Contacts to ManyChat Subscribers.

### Architectural Overview & Data Flow
The integration will utilize a standard event-driven architecture to ensure OHC remains responsive and fault-tolerant.

1. **Authentication Flow:**
   - User navigates to the 'Integrations' panel within their OHC dashboard.
   - User clicks 'Connect' for the respective service.
   - OHC initiates an OAuth 2.0 flow (or prompts for secure API key entry if OAuth is unavailable for this specific tool).
   - Upon successful authorization, access and refresh tokens are securely stored (encrypted at rest in PostgreSQL for Cloud mode, or within the encrypted SQLite SIPDB for Standalone mode).

2. **Data Ingestion (Inbound Events):**
   - The third-party tool generates an event based on user or customer action.
   - An HTTP POST request (webhook payload) is sent to OHC's configured webhook listener endpoint.
   - OHC verifies the webhook signature using the configured shared secret to ensure authenticity and prevent spoofing.
   - The payload is parsed, validated against our internal schemas, and normalized into standard OHC internal models.
   - Real-time updates are pushed to the user's active UI session via WebSockets, ensuring they see the new data immediately without refreshing the page.

3. **Action Execution (Outbound Requests):**
   - The user performs an action in the OHC UI (e.g., clicking 'Approve', sending a message).
   - OHC translates this UI action into the appropriate REST API payload required by the third-party service.
   - OHC dispatches the call asynchronously via the background task queue.
   - The system handles rate limits, temporary network failures, and 5xx errors by employing an exponential backoff and retry strategy.

4. **Error Handling & State Synchronization:**
   - If an integration falls out of sync (e.g., missed webhooks during downtime), OHC will initiate a background reconciliation job upon startup or at daily intervals to fetch missing state.
   - Token expiration events will automatically attempt a silent refresh. If that fails, the user will be notified in the UI to re-authenticate.
   - Detailed audit logs are maintained for all cross-platform synchronization events.

5. **Entity Mapping Strategy:**
   - External IDs will be mapped to internal UUIDs via a dedicated `integration_mappings` table to allow robust bi-directional sync without tight coupling.
   - Soft deletes will be utilized when a linked entity is removed in the third-party system to maintain historical audit trails.


## Implementation Prompt
Create a simple integration card in the settings menu. Implement the OAuth flow. Build the backend handlers for the ManyChat webhooks and the outbound message sending. Ensure the UI clearly shows which platform a message originated from (e.g., an Instagram icon next to the message).

### User Experience (UX) Considerations & Design Principles
- **Zero-Jargon Interface:** The settings page must strictly avoid terms like 'Webhooks', 'REST API', 'Endpoints', or 'OAuth'. Use business-friendly phrases like 'Automatic Sync', 'Connection Status', and 'Real-time Updates'. This adheres to the 'Plain Language Only' constraint.
- **Frictionless Onboarding:** The connection process should ideally be a single click (OAuth flow). If manual configuration is unavoidable, provide a clear, step-by-step visual guide with screenshots indicating exactly where to find the required credentials.
- **Clear State Visibility:** The user should always know the status of the connection at a glance. Provide clear visual indicators (e.g., a green dot and 'Connected' text; a red dot with actionable advice like 'Connection lost. Click here to re-link your account').
- **Graceful Error Handling:** If the third-party service experiences an outage, OHC must not crash or display raw JSON error codes. Instead, display a friendly, empathetic message: "We're having trouble connecting to [Tool Name] right now. We'll keep trying in the background so you don't have to worry."
- **Progressive Disclosure:** Hide advanced configuration options (like manual sync triggers, detailed logs, or fine-grained event filtering) by default to avoid overwhelming the non-technical user. Allow power users to toggle an 'Advanced Settings' view.
- **Accessibility:** All UI components associated with this integration must pass WCAG AA standards, including proper ARIA labels, contrast ratios, and keyboard navigability.

### Onboarding UX Flow
1. User clicks 'Connect ManyChat' in the Integration Library.
2. User is redirected to ManyChat to authorize OHC.
3. Upon return, OHC confirms connection and immediately begins syncing the last 24 hours of conversations.
4. A helpful tooltip guides the user to the new 'Unified Inbox' tab.

### Detailed Acceptance Criteria
- [ ] User can connect the tool via the UI without any developer assistance or reading technical documentation.
- [ ] Integration status accurately reflects the real-time connection state.
- [ ] Data correctly syncs in both directions according to the specified data flow architecture without duplication or loss.
- [ ] Error states (e.g., revoked tokens, rate limits, network timeouts) are handled gracefully, logged internally, and communicated clearly to the user without jargon.
- [ ] The feature functions correctly in both OHC Cloud (Multi-tenant Postgres/Redis) and OHC Standalone (Local SQLite) modes, respecting the data isolation requirements of each.
- [ ] Comprehensive unit tests (100% coverage on new integration logic) and at least 5 Playwright E2E tests are implemented to cover the full connection lifecycle and a typical customer journey.
- [ ] All new UI components adhere strictly to OHC Premium Design Standards (Outfit font for headings, Inter for body, appropriate easing for animations, mobile-first responsive layout).
- [ ] Documentation is updated in the Help Center outlining the new capability and providing a simple troubleshooting guide.
- [ ] The sync logic is fully idempotent, ensuring no duplicate records are created if a webhook is processed twice.

### Testing Strategy
1. Write mock webhooks to simulate incoming IG/FB messages and verify they appear in the UI.
2. Write Playwright tests confirming the user can reply to a message and the correct API payload is generated.
3. Simulate API rate limits and verify the background job queues and retries the message delivery.


## Priority
P1

## Estimated Scope
Medium
