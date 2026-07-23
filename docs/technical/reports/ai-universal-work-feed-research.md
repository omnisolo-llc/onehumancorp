# Research Report: OHC Universal Inbox and Agentic Assistant

## Problem Statement
Owners and operators are overwhelmed by the multitude of channels where work enters (DMs, emails, WhatsApp, forms, etc.). They need a single, unified place to see everything and let AI draft actions based on context. Traditional SMB tools force the owner to act as a routing layer, manually sorting messages, extracting intents, copying data to CRM, and creating tasks. The proposed solution is an AI-first Work Feed where AI acts as the intake coordinator.

## Research Report
- **Top 10 General Competitors Reviewed**: Shopify Magic, HubSpot AI, Notion AI, Square, WeCom, DingTalk, Microsoft Copilot, Zoho Zia, Zendesk AI, Intercom Fin.
- **Top 10 AI-Native Competitors Reviewed**: Gorgias, HelpScout, Klaviyo AI, Auto-GPT plugins, Zapier Central, Dust, LangChain/LangGraph-based agents, Adept AI, Lindy.ai, MultiOn.
- **Deep-Dive Audit: Shopify Magic & Inbox**: While Shopify provides unified chat, its AI mostly suggests replies based on store policies. It fails to autonomously create complex bookings, parse non-standard service requests, or integrate deep operational context for service-based SMBs. Success factors: Fast mobile app, intuitive UI. Missing: Multi-step agentic workflows that update the POS, CRM, and calendar simultaneously without owner clicks.

### User Sentiment Audit (Shopify Magic / HubSpot AI)
- **Positive**: "It saves me 10 minutes per email by drafting a polite response."
- **Negative**: "I still have to read every message to figure out if it's a lead or spam." "The AI doesn't know my schedule, so it can't book appointments." "I have to manually copy the customer's requested date into my calendar."

### Persona-Specific Pain Points
- **Maya (Baker)**: Receives custom cake orders via IG DMs. Pain: Spending hours matching DMs to calendar dates and sending deposit links.
- **Carlos (Field Service)**: Gets SMS and WhatsApp leads. Pain: Forgets to reply to WhatsApp leads when on a job site.
- **Priya (Boutique)**: Manages Instagram comments and emails. Pain: Misses VIP customer inquiries buried under generic questions.

## Gap Matrix & Feature Heatmap

| Feature | Shopify Magic | HubSpot AI | WeCom | OHC (Current) | OHC (Proposed) |
|---------|---------------|------------|-------|---------------|----------------|
| Unified Inbox | Yes | Yes | Yes | Partial | Yes |
| AI Draft Replies | Yes | Yes | Yes | No | Yes |
| Autonomous Triage | No | Partial | No | No | Yes |
| Multi-Agent Handoff | No | No | No | No | Yes |
| 375px Mobile First | Yes | Yes | Yes | Yes | Yes |

### Competitive Landscape (Mermaid)
\`\`\`mermaid
quadrantChart
    title AI Assistant Capabilities vs. Operational Autonomy
    x-axis Low Autonomy --> High Autonomy
    y-axis Single Channel --> Multi-Channel
    quadrant-1 High Autonomy, Multi-Channel
    quadrant-2 Low Autonomy, Multi-Channel
    quadrant-3 Low Autonomy, Single Channel
    quadrant-4 High Autonomy, Single Channel
    "Shopify Magic": [0.4, 0.8]
    "HubSpot AI": [0.5, 0.9]
    "Notion AI": [0.3, 0.3]
    "Lindy.ai": [0.8, 0.6]
    "OHC (Proposed)": [0.9, 0.9]
\`\`\`

### Agentic Handoff Journey (Mermaid)
\`\`\`mermaid
sequenceDiagram
    actor Customer
    participant WorkFeed
    participant TriageAgent
    participant CustomerAgent
    participant OpsAgent
    actor Owner

    Customer->>WorkFeed: Sends IG DM "Need cake for 10/12"
    WorkFeed->>TriageAgent: New Message Event
    TriageAgent->>TriageAgent: Analyze Intent (Booking Request)
    TriageAgent->>CustomerAgent: Fetch Maya's preferences & CRM
    TriageAgent->>OpsAgent: Check Calendar for 10/12
    OpsAgent-->>TriageAgent: Available
    TriageAgent->>WorkFeed: Create Draft Reply + Calendar Hold
    WorkFeed->>Owner: Push Notification (1 Action Required)
    Owner->>WorkFeed: 1-Tap Approve
    WorkFeed->>Customer: Sends Reply & Payment Link
\`\`\`

## Design Doc
- **Entity Types**: `UniversalMessage` (normalized from IG, Email, SMS), `AgentIntent` (classification, priority, suggested actions), `ActionDraft` (pending state awaiting owner approval).
- **Key Relationships**: A `UniversalMessage` belongs to a `Thread` and `Tenant`. It has one `AgentIntent` and zero or many `ActionDraft`s.
- **UI/UX Flow (375px Mobile First)**:
  1. **Home Screen (Work Feed)**: A vertically scrolling list. Items are not just messages; they are "Work Units" (e.g., "New Lead: Carlos - Needs Roof Estimate").
  2. **Work Unit Detail**: Shows the original message context at the top, and a translucent, Apple-style "Agent Action Card" at the bottom with a drafted reply and a drafted quote.
  3. **Action Buttons**: Large 44x44px touch targets for "Approve & Send", "Edit Draft", or "Dismiss".
- **Integration Points**: Incoming webhooks from external channels -> PostgreSQL `messages` table -> RLS -> AI Job Queue (SKIP LOCKED) -> Gemini Pro API -> Updates UI via WebSockets.

## Implementation Prompt
**User-Facing Outcome**: The owner opens the OHC app and sees a prioritized feed of actionable items, not a raw inbox. Each item has a pre-drafted response and proposed operational action (e.g., booking a date) ready for 1-tap approval.
**Critical User Journey (CUJ)**:
1. Owner logs in.
2. Owner navigates to "Work Feed".
3. Owner sees an IG DM from a customer asking for availability.
4. Owner sees that the AI has already drafted a polite reply and created a tentative calendar block.
5. Owner taps "Approve". The reply is sent and the calendar is updated.
**Acceptance Criteria**:
- The UI must render correctly at 375px width.
- The AI job queue must reliably process incoming messages without dropping them.
- The UI must update optimistically when the owner approves a draft.
- ZERO mock data in the UI; use real backend state.

## Priority
P0

## Estimated Scope
Large

## References & Sources
1. https://www.shopify.com/magic
2. https://www.hubspot.com/products/artificial-intelligence
3. https://www.notion.so/product/ai
4. https://www.dingtalk.com/en
5. https://work.weixin.qq.com/
6. https://www.apple.com/business/
7. https://ui.com/consoles
8. https://gocardless.com/
9. https://www.klarna.com/business/
10. https://www.paypal.com/us/business
11. https://www.salesforce.com/einstein/
12. https://www.zendesk.com/service/ai/
13. https://www.intercom.com/fin
14. https://www.gorgias.com/
15. https://help-scout.com/
16. https://www.zoho.com/zia/
17. https://calendly.com/
18. https://acuityscheduling.com/
19. https://www.mindbodyonline.com/
20. https://www.wix.com/adi
21. https://www.squarespace.com/
22. https://wordpress.com/
23. https://woocommerce.com/
24. https://www.bigcommerce.com/
25. https://magento.com/
26. https://www.odoo.com/
27. https://www.xero.com/
28. https://www.quickbooks.intuit.com/
29. https://www.freshbooks.com/
30. https://www.waveapps.com/
31. https://gusto.com/
32. https://rippling.com/
33. https://deel.com/
34. https://www.asana.com/
35. https://monday.com/
36. https://clickup.com/
37. https://trello.com/
38. https://airtable.com/
39. https://mailchimp.com/features/ai-marketing/
40. https://www.klaviyo.com/features/ai
41. https://www.typeform.com/
42. https://www.vagaro.com/
43. https://www.glossgenius.com/
44. https://www.jobber.com/
45. https://www.housecallpro.com/
46. https://www.servicetitan.com/
47. https://stripe.com/docs/stripe-terminal
48. https://about.ads.microsoft.com/en-us/solutions/microsoft-copilot
49. https://www.reddit.com/r/smallbusiness/
50. https://www.trustpilot.com/
51. https://apps.apple.com/
52. https://www.zapier.com/central
53. https://www.adept.ai/
54. https://www.lindy.ai/
55. https://www.multion.ai/
