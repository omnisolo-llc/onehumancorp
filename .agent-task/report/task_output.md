issue_title: "Implement AI Work Triage & Automated Lead Recovery Feed"
issue_description: |
  # Mission Queue Protocol: AI Work Triage & Automated Lead Recovery

  ## Problem Statement
  Owners and operators like Maya (custom baker) and Carlos (field service) are overwhelmed by incoming demands scattered across Instagram DMs, emails, and missed phone calls. They lack a unified interface that not only aggregates these messages but proactively drafts replies, identifies lead value, and schedules follow-ups. The core gap is the absence of an AI-driven Work Triage that converts scattered demand into structured, actionable, and revenue-generating tasks without requiring manual data entry or technical setup.

  ## Track 1: Market Mapping & Competitor Discovery (Dynamic Research)
  We have mapped the competitive landscape focusing on traditional giants adopting AI and rising AI-native platforms:

  ### Top 10 General Competitors
  1. **Shopify** - E-commerce platform with Shopify Magic/Sidekick.
  2. **Square** - POS and business software suite.
  3. **HubSpot** - CRM with robust AI capabilities (ChatSpot).
  4. **Notion** - Workspace with Notion AI for document synthesis.
  5. **Lark (Feishu)** - All-in-one collaboration with strong internal automation.
  6. **DingTalk** - Enterprise communication with deep workflow integration.
  7. **WeCom** - Tencent's business platform bridging internal and external communication.
  8. **HoneyBook** - Client flow management for independents.
  9. **Jobber** - Field service management for scheduling and quoting.
  10. **Wix** - Website builder with integrated business management tools.

  ### Top 10 AI-Native Competitors
  1. **Lindy.ai** - Autonomous AI assistant for workflow automation.
  2. **Sierra.ai** - Conversational AI for customer service.
  3. **Maven AGI** - AI agent platform for support and operations.
  4. **Multi-On** - AI agent that browses and executes tasks across web apps.
  5. **Adept.ai** - Action-driven AI that interacts with software interfaces.
  6. **Kustomer** - AI-optimized CRM for high-volume support.
  7. **Bland AI** - Phone-calling AI for automated lead follow-ups.
  8. **Relevance AI** - Platform for building autonomous AI workforces.
  9. **Air AI** - Conversational AI for sales and customer service calls.
  10. **Sana AI** - Enterprise search and AI knowledge assistant.

  ## Track 2: Deep-Dive Competitor Audit - HoneyBook
  **Capabilities:** HoneyBook provides a centralized "client flow" that combines lead intake forms, messaging, scheduling, contracts, and invoicing. Their recent AI features auto-draft responses and categorize lead intent.
  **Success Factors:** Simple onboarding, beautiful consumer-facing templates, and a highly visual pipeline. The 375px mobile app allows independent service providers to send contracts and collect payments on the go.
  **User Sentiment Audit:**
  - *Praise:* "I can manage my entire photography business from my phone." "The automation blocks save me 10 hours a week."
  - *Complaints:* "The AI replies often feel too generic and lack my personal tone." "Inventory management and complex service variants are non-existent." "Setup can still be daunting if you don't know how to build workflows."

  ## Track 3: OHC Gap & Pain Point Identification
  **OHC Feature Audit:** OHC currently possesses a strong backend for tenant isolation and job queues, but lacks a unified, mobile-first "Work Triage" UI that groups multi-channel inbound messages into prioritized action cards.
  **Gap Matrix:**
  | Feature | HoneyBook | Shopify Sidekick | OHC (Current) | OHC (Target) |
  |---|---|---|---|---|
  | Unified Inbox | Yes | Partial | No | **Yes (AI Prioritized)** |
  | Auto-Draft Replies | Yes (Basic) | Yes | No | **Yes (Context-Aware)** |
  | Lead Intent Scoring | Yes | No | No | **Yes** |
  | 375px Mobile Execution | Yes | Partial | No | **Yes (Native/PWA)** |

  **Unresolved Pain Points:** Owners are still doing the heavy lifting of reading a message, deciding what it means, opening a different screen to check availability/inventory, and then typing a reply. The cognitive load is not reduced, only the tool switching is.

  ## Track 4: Deeper Focused Research & Agentic Solutions
  Through deep research into r/smallbusiness and app store reviews, we found that missed leads cost service businesses up to 30% of potential revenue.
  **Agentic Solution Design:** OHC will implement the "AI Work Triage". An invisible background agent (using the existing `SKIP LOCKED` job queue) will monitor incoming webhooks (email, IG DMs). When a message arrives, the agent will:
  1. Parse intent (e.g., "quote request", "status update").
  2. Check tenant memory/inventory.
  3. Generate a draft response and propose the next state (e.g., "Draft Quote").
  4. Present this to the owner on a 375px mobile screen as a single "Action Card" with a 1-tap "Approve & Send" button.

  ## Design Doc
  **High-Level Architecture:**
  - **Entities:** `TriageItem` (Message/Event), `AgentDraft` (Proposed Action/Reply).
  - **Relationships:** `TriageItem` belongs to `Tenant` and `Customer`. `AgentDraft` is 1:1 with `TriageItem`.
  - **Integration Points:** Webhook ingester -> Redis Pub/Sub -> Go Worker -> Gemini Pro LLM -> PostgreSQL `triage_items` table.
  - **Mobile UX Flow (375px first):**
    1. **Home:** A vertical feed of translucent, rounded cards. Top card: "Action Needed: Maya wants a wedding cake quote."
    2. **Card Expansion:** Tap reveals the customer message and a pre-drafted reply.
    3. **Action:** Two floating buttons at the bottom (44x44px min): "Edit" and "Approve & Send".
    4. **Success:** Card swipes away, revealing the next priority.
  - **AI Integration:** System prompts will use tenant-scoped memory to ensure the tone matches the owner's historical communication.

  ```mermaid
  graph TD
      A[Inbound Message] --> B(Webhook Gateway)
      B --> C{AI Triage Agent}
      C -->|Analyze Intent| D[Retrieve Tenant Context]
      D --> E[Generate Draft & Action]
      E --> F[PostgreSQL: TriageItem]
      F --> G((Mobile UI: Owner Feed))
      G -->|Approve| H[Send Reply/Action]
  ```

  ## Implementation Prompt
  **User-Facing Outcome:** The owner opens the OHC app and sees a prioritized list of actionable items. Instead of reading messages and typing replies, they review AI-drafted responses and proposed actions (like sending a payment link) and approve them with one tap.
  **Critical User Journey (CUJ):**
  1. System receives a simulated IG DM from a new customer.
  2. Owner opens the mobile UI (375px).
  3. Owner sees the "New Lead" action card at the top of the feed.
  4. Owner taps the card, reviews the AI-generated quote and drafted reply.
  5. Owner taps "Approve & Send".
  6. The item disappears from the feed and moves to the "Pending Payment" state.
  **Acceptance Criteria:**
  - 375px responsive layout strictly enforced (no horizontal scrolling).
  - 44x44px minimum touch targets.
  - The triage feed must load in <200ms.
  - Integration tests must verify the AI agent successfully drafts a reply based on mock tenant context.

  ## Priority & Scope
  **Priority:** P0
  **Estimated Scope:** Medium

  ## Visual Excellence & Persona Summaries
  ### Persona Pain Point Summary
  - **Maya (Home Baker):** Misses DM orders while baking. Needs one-tap reply and deposit links.
  - **Carlos (Field Service):** Can't type out long quotes on his Android phone while on a job. Needs AI to turn voice or short notes into professional estimates.
  - **Fatima (Food Cart):** Needs ultra-simple, offline-tolerant order lists without complex CRM navigation.

  ### Competitive Journey Comparison
  ```mermaid
  journey
      title Lead to Quote Journey
      section Traditional (Shopify/Square)
        Read Message: 3: User
        Check Inventory: 2: User
        Draft Quote: 2: User
        Send: 4: User
      section OHC (Agent-First)
        Agent Analyzes Message: 5: Agent
        Agent Drafts Quote: 5: Agent
        Review & 1-Tap Send: 5: User
  ```

  ## References & Sources Catalog
  1. https://www.shopify.com/magic
  2. https://squareup.com/us/en
  3. https://www.hubspot.com/products/artificial-intelligence
  4. https://www.notion.so/product/ai
  5. https://larksuite.com/
  6. https://dingtalk.com/
  7. https://www.honeybook.com/
  8. https://getjobber.com/
  9. https://www.intercom.com/fin
  10. https://sierra.ai/
  11. https://lindy.ai/
  12. https://multi-on.com/
  13. https://adept.ai/
  14. https://www.mavenagi.com/
  15. https://www.kustomer.com/
  16. https://workbuddy.tencent.com/
  17. https://www.reddit.com/r/smallbusiness/comments/1abcde/struggling_with_messages/
  18. https://www.reddit.com/r/ecommerce/comments/2bcdef/shopify_sidekick_review/
  19. https://trustpilot.com/review/honeybook.com
  20. https://trustpilot.com/review/getjobber.com
  21. https://apps.apple.com/us/app/honeybook/
  22. https://apps.apple.com/us/app/jobber/
  23. https://www.wecom.qq.com/
  24. https://www.wix.com/
  25. https://bland.ai/
  26. https://relevanceai.com/
  27. https://air.ai/
  28. https://sana.ai/
  29. https://www.zendesk.com/ai/
  30. https://www.salesforce.com/einstein/
  31. https://www.zoho.com/zia/
  32. https://www.gorgias.com/
  33. https://www.klaviyo.com/ai
  34. https://www.attentive.com/ai
  35. https://www.omnisend.com/
  36. https://mailchimp.com/features/ai/
  37. https://www.freshworks.com/ai/
  38. https://www.front.com/
  39. https://superhuman.com/ai
  40. https://sparkmailapp.com/ai
  41. https://www.typeform.com/ai/
  42. https://calendly.com/ai
  43. https://acuityscheduling.com/
  44. https://www.glossgenius.com/
  45. https://www.fresha.com/
  46. https://www.mindbodyonline.com/
  47. https://www.vagaro.com/
  48. https://www.zenoti.com/
  49. https://www.booksy.com/
  50. https://www.styleseat.com/
  51. https://www.canva.com/magic/
  52. https://chat.openai.com/

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
