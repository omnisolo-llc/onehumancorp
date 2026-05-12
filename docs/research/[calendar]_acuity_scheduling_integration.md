# Title: Integrate Acuity Scheduling for Calendar & Scheduling

## Problem Statement
Service-based businesses (like salons or fitness coaches) face high no-show rates because they cannot easily require upfront deposits or customized intake forms during the booking process.
Customizable scheduling with deep payment and intake form capabilities.
By integrating Acuity Scheduling directly into the OHC platform, we can eliminate the administrative friction that plagues small business owners. Rather than maintaining separate tabs and losing contextual data, the user requires a centralized experience. They demand a simple, secure authorization handshake that immediately syncs their external data with OHC's unified dashboard, bypassing the need for complex API key management.

## Research Report
### Overview
Acuity Scheduling has established itself as a critical player in the Calendar & Scheduling sector. Our comprehensive research indicates that a significant portion of our user base already utilizes this tool. Integrating it will directly address core workflow inefficiencies.

### Competitive Analysis
When positioned against other tools in the market, Acuity Scheduling offers a unique value proposition tailored to its specific audience.
- It excels in providing specialized features that generic tools overlook.
- The platform maintains a high reputation on software review sites like G2 and Capterra.
- It possesses a robust developer ecosystem, ensuring the API is stable enough for our integration needs.

### Advantages and Risks
**Advantages:**
- Deep customization of the client booking interface.
- Excellent integration with Stripe, Square, and PayPal for upfront payments.
- Automated timezone conversion for international clients.
- Subscription and package management built-in.

**Risks:**
- Acquired by Squarespace, leading to concerns about future standalone viability.
- The administrative backend can feel slightly clunky.
- Custom CSS is restricted to higher-tier plans.

### Pricing Estimate
Emerging plan starts at $20/month. Growing tier at $34/month adds SMS reminders.

### Cloud and Standalone Support
- **Cloud Mode**: Full compatibility is achievable via standard OAuth 2.0 flows and inbound webhooks. Our multi-tenant architecture will enforce strict credential isolation, ensuring data privacy across OHC tenants.
- **Standalone Mode**: In environments where inbound webhooks are blocked, the integration must fall back to a secure polling mechanism. Alternatively, users may configure local API tokens directly.

## Design Doc
The integration will be accessible via the "Integrations & Add-ons" marketplace within the OHC dashboard.
- **Trigger**: The business owner clicks "Connect Acuity Scheduling" and completes the standard authorization flow. Upon success, they are redirected back to OHC.
- **Action**: OHC encrypts and stores the integration tokens. A background worker immediately initiates a historical sync (e.g., fetching the last 30 days of data).
- **User View**: A dedicated widget or unified inbox tab will materialize, presenting the Acuity Scheduling data natively. Notifications will be merged into the existing OHC alert feed.
- **Lifecycle**: The system will preemptively refresh OAuth tokens. If a token is revoked, a clear, actionable alert will prompt the user to re-authenticate.

## Implementation Prompt
**User-Facing Outcome:**
As a small business owner, I can securely connect my Acuity Scheduling account to OHC with just a few clicks. Once authorized, my critical operational data seamlessly appears in my OHC dashboard, allowing me to monitor and act on information without leaving the platform.

**Acceptance Criteria:**
1. A distinct integration tile for Acuity Scheduling is present in the UI.
2. The authorization flow securely handles OAuth or token exchange.
3. Upon connection, a clear status indicator (e.g., "Syncing...") is displayed.
4. Relevant external data is accurately mapped to OHC's internal data models.
5. In the event of a sync failure, a non-technical error message guides the user to a solution.
6. The integration operates flawlessly in both Cloud and Standalone topologies.

## Priority
P1

## Estimated Scope
Medium
