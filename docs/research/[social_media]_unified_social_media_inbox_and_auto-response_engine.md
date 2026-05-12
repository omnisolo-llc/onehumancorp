# Unified Social Media Inbox and Auto-Response Engine

## Problem Statement
Small business owners struggle to keep up with customer inquiries scattered across Instagram DMs, Facebook Messenger, WhatsApp, and TikTok. Missing a message often means losing a sale. They need a single, easy-to-use inbox that aggregates all messages and provides automated, helpful replies without sounding robotic.

### Target Personas
- **Maria, local baker: Receives 50+ cake inquiries via Instagram and WhatsApp daily. Cannot bake and reply simultaneously.**
- **David, fitness coach: Gets TikTok comments asking for rates, needs to convert them to DMs quickly.**
- **Sarah, boutique owner: Wants to run Facebook ads that click to Messenger, but misses messages while managing the physical store.**

## Research Report
We conducted a comprehensive analysis of the available tools in the market to solve this specific challenge for small businesses.

### Competitive Tool Analysis

#### Intercom
- **Ease of Use**: High for users, but setup can be complex.
- **Pricing Model**: $74/month starter, scaling up quickly based on seats and features.
- **Market Reputation**: Enterprise-grade, highly reliable.
- **Key Advantages**: Incredible shared inbox features, powerful chatbot capabilities, robust APIs, excellent SLA.
- **Identified Risks**: Price can be prohibitive for SMBs; too many features might overwhelm non-technical users.
- **Architecture Compatibility**: Cloud-only SaaS.

#### Zendesk Sunshine
- **Ease of Use**: Medium. Requires some technical setup for custom channels.
- **Pricing Model**: $55/agent/month.
- **Market Reputation**: Industry standard for customer support.
- **Key Advantages**: Connects almost any messaging app. Highly customizable workflows. Comprehensive reporting.
- **Identified Risks**: Steep learning curve for the business owner. Expensive for teams. UI feels dated.
- **Architecture Compatibility**: Cloud-only.

#### Hootsuite Inbox
- **Ease of Use**: High. Very user-friendly interface.
- **Pricing Model**: $99/month.
- **Market Reputation**: Pioneer in social media management.
- **Key Advantages**: Combines social posting with inbox. Great for businesses active on multiple networks.
- **Identified Risks**: Limited advanced chatbot features compared to dedicated customer support tools.
- **Architecture Compatibility**: Cloud-only.

#### ManyChat
- **Ease of Use**: High. Visual flow builder is intuitive.
- **Pricing Model**: $15/month Pro plan.
- **Market Reputation**: Dominant in Instagram/Messenger marketing.
- **Key Advantages**: Excellent for automated sales funnels directly in DMs. Very affordable.
- **Identified Risks**: Primarily focused on marketing automation rather than a pure support inbox.
- **Architecture Compatibility**: Cloud SaaS.

### Market Context
The unified communications market for SMBs is growing at 15% CAGR. Customers now expect replies within 10 minutes on social media.

## Design Doc
The Unified Social Inbox module will appear as a new 'Messages' tab in the OHC dashboard. Users authorize their social accounts via standard OAuth flows. When a customer sends a message on any connected platform, a webhook triggers OHC to ingest the message. The user sees a single, unified chat interface where they can reply. OHC routes the reply back to the native platform via their respective APIs.

### Security & Compliance
OAuth tokens must be securely stored. Need to handle rate limits gracefully to avoid API bans from Meta.

### Resilience Strategy
Implement a webhook ingestion queue. If OHC is down, webhook retries from Meta must be successfully processed later.

## Implementation Prompt
Implement a Unified Inbox feature in the OHC dashboard. The user should be able to connect their Instagram and WhatsApp accounts with a single click. When a message is received, it should appear in a real-time feed. The user should be able to type a reply and hit send, and the message should be delivered back to the customer on their original platform. Include read receipts and unread badges.

### Acceptance Criteria
- [ ] User can authenticate Instagram via OAuth.
- [ ] Incoming Instagram DM appears in OHC within 5 seconds.
- [ ] User reply from OHC appears in customer's Instagram DM.
- [ ] UI updates optimistically to show message sent state.
- [ ] Errors from Meta API are displayed gracefully to the user.

## Priority
P0

## Estimated Scope
Large

## Extended Architectural Considerations

When implementing social_media, developers must consider the implications for both the multi-tenant Cloud deployment of OHC and the self-hosted Standalone mode.

In Cloud mode, API rate limiting is a shared concern. A sudden spike in activity from one tenant must not exhaust the API quota for the entire platform. This necessitates a robust queueing system, such as RabbitMQ or AWS SQS, to process outbound requests and ingest incoming webhooks efficiently.

In Standalone mode, the business owner might not have the technical expertise to configure complex OAuth apps or webhook receivers. The UI must guide them through this process with extreme clarity, perhaps utilizing a proxy service maintained by OHC to simplify the webhook routing to dynamic IP addresses typical of self-hosted setups.

Furthermore, data privacy is paramount. Any PII (Personally Identifiable Information) synced from social_media tools must be encrypted at rest within the OHC database. Retention policies should automatically purge transient data (like raw webhook payloads) after successful processing to minimize the attack surface.

The user interface must remain mobile-first. Small business owners operate primarily from their smartphones. Therefore, the settings pages, dashboards, and daily interaction elements designed for this integration must be fully responsive and pass the 'Grandmother Test' for usability.

By carefully considering these architectural, security, and usability constraints, we can deliver an integration that not only functions reliably but empowers the user to grow their business without friction.
