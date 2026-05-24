# [Onboarding] Agentic Setup Brief: Eradicating Manual Business Configuration

## Problem Statement

Small business owners—from bakers to handymen—are fundamentally experts in their craft, not in web development or CRM management. Traditional platforms like Shopify, Wix, and Squarespace force users into complex, high-friction onboarding flows requiring manual configuration of domains, payment gateways, and booking calendars. Even rising AI-native competitors like Durable generate sites quickly, but abandon the user in a conventional, manual CRM dashboard.

The core unresolved pain point is **"App Fatigue and Setup Paralysis."** Users want their business to simply "work" without learning software. They suffer from missed leads because they are busy doing their actual work (e.g., a handyman fixing a pipe) and cannot immediately respond to manual CRM alerts.

## Research Report

### Competitive Landscape Mapping

**Comparison Table: OHC vs Durable vs Shopify vs Wix**

| Feature | OHC (Proposed) | Durable | Shopify | Wix |
| :--- | :--- | :--- | :--- | :--- |
| AI Site Generation | Yes (Instant via social/prompt) | Yes (30s prompt) | No (Manual/Themes) | No (Manual ADI) |
| CRM Style | **Actionable Mobile Feed** | Traditional Inbox | App ecosystem | Traditional Inbox |
| Autonomous Operations | **Yes (Agents execute tasks)** | No (User must reply/send) | No | No |
| Target User | Mobile-first non-technical | Desktop/Mobile non-technical | Desktop E-commerce | Desktop visual designer |


1. **Traditional Giants (Shopify, Wix, Squarespace, Square):** Highly capable, massive app ecosystems, but require significant manual setup. "Users state they need a degree in web development just to set up variant pricing."
2. **AI-Native Challengers (Durable, 10Web, Hostinger AI, Framer, Mixo):** Optimize the initial 30-second website creation via generative AI. However, post-creation, the burden of managing bookings, following up on leads, and inventory sync remains manual.

### Deep-Dive Audit: Durable AI
- **Capabilities:** Generates a website, CRM, and basic invoicing in seconds based on a prompt. It features AI blog writers and basic lead review tools.
- **Success Factors:** "Get online in 30 seconds." Simplifies the 0-to-1 phase remarkably well. Subscription at $25/mo for the Launch plan is highly attractive.
- **User Sentiment & The Gap:** While users praise the speed (4.8 Trustpilot claim), real-world feedback across communities shows the "CRM" is just another inbox to manage. It lacks *autonomous agentic execution*—the system does not proactively reply to a lead on Instagram, block out a calendar automatically based on context, or generate quotes without manual clicking.

### The OHC Gap
OHC currently maps out the "Ah-Ha" moment (activation) well (e.g., extracting photos from Instagram to build a site). However, the missing link is bridging the gap between an AI-generated site and *Zero-Touch Operations*. We need an **Onboarding Agent** that transitions directly into an **Operations Agent**, managing the business invisibly.

## Design Doc

### High-Level Architecture & Agent Flow

The solution involves a transition from an "Onboarding/Promoter Agent" to an "Operations Agent" immediately upon setup, turning a static CRM into an active proxy for the business owner.

```mermaid
graph TD
    A[User Inputs IG Handle / Business Type] --> B(Onboarding Agent)
    B --> C[Generates Site, Menu, Booking Rules]
    C --> D{Approval via Mobile Push (1-Tap)}
    D -- Approved --> E[Site Live + Operations Agent Active]

    F[Customer DM/Email Inquiry] --> G(Operations Agent)
    G --> H{Contextual Intent Analysis}
    H -- Needs Quote --> I[Drafts Quote based on OHC Pricing]
    H -- Needs Booking --> J[Cross-checks OHC Calendar & Proposes Slots]
    I --> K[Push Notification to Owner: 'Approve Quote?']
    J --> L[Push Notification to Owner: 'Approve Booking?']
    K -- 1-Tap Yes --> M[Send to Customer]
    L -- 1-Tap Yes --> M
```

### Mobile UX Flow (375px First)

1. **The Chat Interface (Setup):** User opens app. No menus. Just a chat interface. "What do you do?" User replies: "I fix plumbing."
2. **The Magic Moment:** "Great. I'm building your site, setting your hourly rate to the local average ($75/hr), and linking to your Google Calendar. Sound good?"
3. **The Activity Feed (Retention):** Instead of a standard dashboard (Total Sales, Visitors), the home screen is an Action Feed.
   - *Card 1:* "New Lead: John needs a leaky pipe fixed tomorrow. I drafted a quote for $150. [Send Quote]"
   - *Card 2:* "Inventory Alert: Only 2 red dresses left. Re-order?"

### Integration Points
- Native OAuth flows simplified into natural language prompts (e.g., "Connect your Gmail so I can read your calendar").
- Webhooks from Instagram/Facebook DMs routed directly to the Operations Agent intent analysis engine.

## Implementation Prompt

**Outcome:** Create an invisible, agentic onboarding and operations flow that completely bypasses traditional dashboard management.

**Critical User Journey:**
1. A non-technical user (e.g., Carlos, handyman) signs up and describes their business in one sentence via mobile.
2. The AI provisions the site, creates base pricing, and instantiates the Operations Agent.
3. A test lead is sent to the site. The Operations Agent intercepts the lead, drafts a response/quote, and sends a push notification to Carlos's mobile device.
4. Carlos clicks "Approve" (1-Tap), and the response is dispatched to the lead.

**Acceptance Criteria:**
- The user is never exposed to a drag-and-drop website editor unless they explicitly ask for it.
- The user is never exposed to a traditional CRM table/grid view; all interactions are surfaced as actionable cards in a mobile-first activity feed.
- The system must autonomously propose a next step (draft email, draft quote, propose calendar slot) for every incoming customer inquiry.

## Priority
**P0** - This is the core differentiator for OHC against Durable and Shopify.

## Estimated Scope
**Large** - Requires tight integration between the conversational UI, the underlying agent execution engine (KAIROS), and the notification service.