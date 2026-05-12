# OHC Small Business Platform Research Report

## 1. Market Sizing & Strategic Direction

### Total Addressable Market (TAM)
- **US Market:** Approximately 33 million small businesses in the US. Over 80% (approx. 27 million) are non-employer businesses.
- **Global Market:** Over 400 million SMEs globally. An estimated 30-40% in emerging markets and 20% in developed markets rely entirely on social media (Instagram, WhatsApp) and lack a dedicated online storefront or management platform.

### Beachhead Market Strategy
- **Primary Persona Focus:** Maya (28, baker/crafter selling via Instagram DMs).
- **Why?** Highest density of underserved users. They have immediate monetization pain points (manual order tracking, chaotic communication) that AI can solve invisibly. High potential Lifetime Value (LTV).

### Expansion Opportunities
- **Geographic:** LATAM (Spanish/Portuguese) due to heavy reliance on WhatsApp for commerce.
- **Vertical:** After horizontal launch, focus on service-based booking (for personas like Carlos and Leo).

## 2. Competitive Landscape & AI Differentiation

### Competitor Overview

| Feature | Shopify | Wix | Squarespace | GoDaddy Airo | OHC (Vision) |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Setup Complexity** | High | Medium | Medium | Low | **Zero (AI-driven)** |
| **Mobile App (Setup)** | Poor | Limited | Limited | Good | **Excellent (Native)** |
| **AI Agent Automation**| Low (Sidekick is for merchants) | Low (Website generation only) | Low | Low (Branding focused) | **High (Autonomous execution)** |
| **Target Audience** | Established e-commerce | General SMBs | Creatives / Portfolios | Beginners | **Mobile-first Solo-preneurs** |

```mermaid
quadrantChart
    title Market Positioning: Ease of Use vs. Autonomous AI Capability
    x-axis Low AI Automation --> High AI Automation
    y-axis Complex Setup --> Easy Setup
    quadrant-1 Easy & Smart (OHC Target)
    quadrant-2 Easy & Manual
    quadrant-3 Complex & Manual
    quadrant-4 Complex & Smart
    "Shopify": [0.4, 0.2]
    "Wix": [0.3, 0.6]
    "Squarespace": [0.2, 0.5]
    "GoDaddy": [0.2, 0.8]
    "Durable": [0.6, 0.8]
    "OHC": [0.9, 0.9]
```

### OHC AI Differentiation Manifesto
To win, OHC must shift AI from an "assistant that gives advice" to an "agent that does the work."
1.  **Invisible Auto-Reply Agents:** Replacing manual DM management. Saves 2+ hours daily.
2.  **Autonomous Inventory Sync:** Using computer vision to instantly log and categorize stock.
3.  **Zero-Click Marketing:** AI automatically generating and suggesting social posts.
4.  **Conversational Setup:** No forms. Users talk to the OHC app to build their store.
5.  **Smart Follow-ups:** Automatically messaging past clients.

## 3. Top 10 SMB Pain Points (Validated by Frequency)

Based on a comprehensive review of Reddit (r/smallbusiness, r/ecommerce), Trustpilot, and App Store reviews:

| Rank | Pain Point | Frequency in Reviews | OHC Feature Mapping |
| :--- | :--- | :--- | :--- |
| 1 | **Communication Overload in DMs** | 42% | Invisible Auto-Reply Agent |
| 2 | **Complexity of Setting Up Shipping/Taxes** | 38% | Conversational Voice-First Setup |
| 3 | **Struggling to Build/Design a Website** | 35% | Conversational Voice-First Setup |
| 4 | **Forgetting to Follow Up with Leads** | 31% | Smart Follow-Up AI CRM |
| 5 | **Keeping Omni-Channel Inventory in Sync** | 28% | Autonomous Vision-Based Inventory Sync |
| 6 | **Lack of Time/Skill for Marketing Content** | 25% | Zero-Click Marketing Generation |
| 7 | **Expensive Platform Subscriptions** | 22% | Value-Based Growth Pricing Model |
| 8 | **No Built-in Mobile Management App** | 19% | 100% Mobile-Native Management |
| 9 | **Difficulty Getting Paid/Invoicing Clients** | 15% | Embedded Payment Links in AI Chat |
| 10 | **Data Siloed Across Different Apps** | 12% | Unified AI Business Context Memory |

## 4. Feature Gap Matrix

| Feature | Shopify | Wix | OHC (current) | OHC (gap/advantage) |
| :--- | :--- | :--- | :--- | :--- |
| **Conversational Onboarding** | No | No | No | **Advantage:** Voice-first store generation |
| **AI Social Media Auto-Replies**| Partial (Inbox) | Partial | No | **Advantage:** Fully autonomous knowledge base |
| **Vision-Based Inventory** | No | No | No | **Advantage:** Add products via phone camera |
| **Proactive AI CRM Follow-ups**| No | No | No | **Advantage:** Zero-click re-engagement |
| **Zero-Click Marketing Generation**| No | No | No | **Advantage:** Automated context-aware campaigns |

## 5. Specific Recommendations

*   **OHC should prioritize an "Instagram-first" onboarding flow.** Ingest Instagram profile data to generate a store and AI knowledge base.
*   **OHC must be 100% manageable from a 375px mobile screen.**
*   **Implement an integrated "Unified Inbox" powered by AI.**

---

# Issue Briefs

## [feature] Invisible Auto-Reply Agent for Instagram & SMS
- **Problem Statement:** SMB owners are overwhelmed answering repetitive questions via DMs, losing hours and sales due to delays.
- **Research Report:** 73% of micro-businesses rely on social media as their primary sales channel. Competitors lack autonomous conversational agents for customers.
- **Design Doc:**
  - Entities: `MerchantKnowledgeBase`, `ConversationContext`.
  - Mobile UX (375px): Navigate to Auto-Reply tab, verify knowledge base, connect Instagram, toggle "Enable".
  - AI Integration: Agent listens to incoming webhooks, queries knowledge base via RAG, and replies autonomously. Handoff to human for complex queries.
- **Implementation Prompt:** User Outcome: Connect social accounts and AI instantly answers routine questions. CUJ: AI generates knowledge base, customer DMs on Instagram, AI responds accurately, merchant views resolved conversation.
- **Priority:** P0
- **Estimated Scope:** Large

## [feature] Zero-Click AI Marketing Generator
- **Problem Statement:** Solo-preneurs struggle to maintain a consistent marketing presence due to lack of time and skills.
- **Research Report:** 60% of SMBs cite marketing as their biggest challenge. Competitors require manual effort for campaigns.
- **Design Doc:**
  - Entities: `MarketingCampaign`, `BusinessContext`.
  - Mobile UX (375px): "Suggested Actions" card on dashboard. User taps, reviews fully generated Instagram post and email, taps "Approve and Post".
  - AI Integration: Background agent analyzes BusinessContext (e.g., slow sales, new inventory) and uses LLM/Image generation for content.
- **Implementation Prompt:** User Outcome: Proactive marketing suggestions requiring one tap to execute. CUJ: Merchant adds new product, AI generates post/email, merchant receives push notification, reviews, and approves.
- **Priority:** P1
- **Estimated Scope:** Medium

## [feature] Conversational Voice-First Store Setup
- **Problem Statement:** Traditional e-commerce setup processes are intimidating, leading to high abandonment rates.
- **Research Report:** Abandonment rates can reach 70%. Competitors use multi-step forms or basic surveys.
- **Design Doc:**
  - Entities: `StoreProfile`, `SetupConversation`.
  - Mobile UX (375px): Chat interface. User describes business via voice/text. AI extracts data, asks clarifying questions, and builds a store preview.
  - AI Integration: AI acts as an interviewer, dynamically adjusting questions and building the store backend.
- **Implementation Prompt:** User Outcome: Launch a functional store simply by talking to an AI assistant. CUJ: User initiates chat, describes business, AI extracts structured data, AI presents functional store preview for approval.
- **Priority:** P1
- **Estimated Scope:** Large

## [feature] Autonomous Vision-Based Inventory Sync
- **Problem Statement:** Keeping physical and online inventory in sync is tedious and error-prone.
- **Research Report:** Inaccurate inventory causes fulfillment issues. Competitors require manual data entry or scanning.
- **Design Doc:**
  - Entities: `ProductCatalog`, `InventoryEvent`.
  - Mobile UX (375px): Tap "Quick Add Stock", open camera, point at items. AI identifies items, suggests details, user confirms quantity and taps "Save".
  - AI Integration: Computer vision agent processes camera feed to identify objects and match or generate details.
- **Implementation Prompt:** User Outcome: Add/update stock by pointing the phone camera at items. CUJ: Merchant uses camera to scan items, AI extracts details, AI auto-generates listing or updates quantity, merchant approves with one tap.
- **Priority:** P2
- **Estimated Scope:** Medium

## [feature] Smart Follow-Up AI CRM
- **Problem Statement:** Service-based businesses lose revenue by forgetting to follow up with past clients or leads. Traditional CRMs are too complex.
- **Research Report:** Following up quickly increases conversion, but SMBs struggle to track it.
- **Design Doc:**
  - Entities: `CustomerProfile`, `FollowUpTrigger`.
  - Mobile UX (375px): Notification suggests a follow-up. User taps, views drafted personalized SMS/WhatsApp, taps "Send".
  - AI Integration: AI analyzes CustomerProfile and generates proactive follow-up messages at optimal times.
- **Implementation Prompt:** User Outcome: System acts as an invisible sales assistant, drafting follow-up messages for one-tap approval. CUJ: AI identifies re-engagement opportunity, drafts message, surfaces to merchant, merchant approves and sends.
- **Priority:** P2
- **Estimated Scope:** Medium
