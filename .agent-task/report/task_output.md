issue_title: "Implement AI Unified Inbox & Triage Agent for SMB Operators"
issue_description: |
  ## Problem Statement
  Operators like Maya (Instagram baker) and Carlos (field service) are overwhelmed by incoming demands across fragmented channels (DMs, emails, forms, phone calls). They lack a single interface that triages messages, identifies actionable work (quotes, bookings, orders), drafts contextual replies using historical tenant memory, and coordinates backend operations autonomously. Instead of managing a business, they are stuck managing inboxes and manual data entry across disconnected systems.

  ## Research Report
  ### Track 1: Market Mapping & Competitor Discovery
  We mapped the landscape of SMB/Creator tools:
  - **General Work/Commerce Platforms:** Shopify, Square, Wix, HubSpot, Calendly, Notion, Salesforce, Slack, DingTalk/Lark, Zoho One.
  - **AI-Native Assistants:** Notion AI, Shopify Sidekick, Harvey, Sierra, Devin, Lindy.ai, MultiOn, AgentOps.

  *Finding:* General platforms are building "copilots" (Shopify Magic/Sidekick) that sit alongside the UI, but they are not the primary interface. They require the user to open dashboards, navigate menus, and ask the AI for help. AI-native tools (Lindy.ai) handle tasks but lack deep, native e-commerce/booking primitives.

  ### Track 2: Deep-Dive Competitor Audit (Shopify + Sidekick)
  **Capabilities:** Shopify is the gold standard for e-commerce, offering inventory, checkout, POS, and apps. Sidekick provides AI chat to modify store settings, write product descriptions, and analyze sales.
  **Success Factors:** Massive app ecosystem, robust APIs, reliable checkout, quick time-to-live for a basic store.
  **User Sentiment Audit:**
  - *Trustpilot/Reddit Reviews:* "Shopify is too complex for my simple service business." "I hate jumping between Instagram DMs and Shopify to create draft orders." "It feels like I'm running an enterprise tool for a one-person bakery."
  - *Core Insight:* Users love the reliability but hate the operational overhead. They want the system to *do the work*, not just provide a dashboard to *manage the work*.

  ### Track 3: OHC Gap Matrix
  | Feature | Shopify (Sidekick) | Notion (AI) | OHC (Current) | OHC (Proposed Vision) |
  |---------|--------------------|-------------|---------------|-----------------------|
  | **Unified Triage** | No (Relies on Inbox App) | No (Doc based) | Emerging | **Yes (Agent-First)** |
  | **Auto-Draft Quotes** | Manual Draft Orders | No | Manual | **Autonomous** |
  | **Contextual Memory** | Customer profiles | Workspace search | Yes | **Deep Tenant Memory** |
  | **Mobile Assistant** | Standard Dashboard App | Mobile Doc Editor | PWA/Flutter | **375px Agent Feed** |

  **Unresolved Pain Point:** OHC lacks a unified, agent-driven inbox where inbound messages from various sources are automatically triaged, summarized, and turned into actionable "Next Step Cards" (e.g., "Maya: 3 new cake inquiries. Drafted 2 quotes. 1 requires date confirmation.").

  ### Track 4: Deeper Focused Research & Agentic Solutions
  **Evidence Gathering:** Small business subreddits show operators spending 2-3 hours daily answering repetitive inquiries and copying details into scheduling/invoicing tools. They use tools like Zapier to bridge gaps, but Zapier is brittle and lacks contextual understanding.
  **Agentic Solution:** The **Triage Agent**. An invisible AI agent that listens to inbound webhooks (email, form, DM), classifies the intent, extracts structured data (dates, items, budget), checks tenant availability/inventory, and drafts a proposed response and action card. The owner simply taps "Approve & Send".


  ### Visual Excellence: Mermaid Diagrams
  #### Competitive Landscape Architecture
  ```mermaid
  graph TD
    subgraph Market
      A[SMB Operator]
      B[General Work/Commerce]
      C[AI-Native Assistants]
      D[OHC: AI Work Assistant]
    end
    A -->|Uses for Everything| B
    A -->|Frustrated, adds| C
    B -->|Provides Complex Dashboards| A
    C -->|Provides Narrow Task execution| A
    D -.->|Unified Triage & Action| A
    style D fill:#4A90E2,stroke:#333,stroke-width:2px,color:#fff
  ```

  #### Triage Agent User Journey
  ```mermaid
  sequenceDiagram
    participant Customer
    participant TriageAgent
    participant OHCMemory
    participant OwnerFeed
    Customer->>TriageAgent: Inquiry (Instagram DM/Form)
    TriageAgent->>OHCMemory: Fetch past context & preferences
    OHCMemory-->>TriageAgent: Returns Customer Profile
    TriageAgent->>TriageAgent: Classify intent, extract dates/budget
    TriageAgent->>TriageAgent: Draft proposed reply & action (e.g., Quote)
    TriageAgent->>OwnerFeed: Push Triage Card
    OwnerFeed->>OwnerFeed: Owner Reviews Card on 375px Screen
    OwnerFeed->>TriageAgent: Taps "Approve & Send"
    TriageAgent->>Customer: Delivers final response
  ```

  ## Design Doc
  **High-Level Architecture:**
  - **Entity Types:** `InboundMessage`, `TriageCard`, `SuggestedAction`.
  - **Relationships:** `TriageCard` belongs to `Tenant` and references `Customer` and `InboundMessage`.
  - **Mobile UX Flow (375px first):**
    1. **Home Feed:** A prioritized list of `TriageCard`s. Translucent glass styling, minimal text.
    2. **Card View:** Shows the user intent ("Wants a custom cake for June 12th"), the extracted details, and the AI's proposed reply.
    3. **Action:** A primary "Approve & Send" button, and a secondary "Edit" button.
  - **AI Integration:** A new `triage_agent` routine in the AI Job Queue that runs on `InboundMessage` creation.

  ## Implementation Prompt
  **Outcome:** The owner logs into OHC and sees a "Today's Triage" feed. Instead of raw messages, they see AI-generated action cards that summarize what the customer wants and propose the next step (e.g., drafting a quote, scheduling a visit).
  **Critical User Journey (CUJ):**
  1. An inbound form submission arrives via API.
  2. The Triage Agent automatically processes it and creates a Triage Card.
  3. The owner opens the OHC mobile view, sees the card, reviews the AI-drafted reply and proposed quote, and taps "Approve".
  4. The system sends the reply and creates the quote in the background.
  **Acceptance Criteria:**
  - The UI must render at 375px without horizontal scroll.
  - The feed must use premium UniFi/Apple-style hierarchical design.
  - The Triage Agent must correctly identify the intent of the message and draft a relevant response based on tenant memory.
  - The action must be resilient to flaky networks, showing pending states.

  ## Appendix: References & Sources Catalog
  1. https://about.instagram.com/features/instagram-shops
  2. https://business.instagram.com/instagram-post
  3. https://www.shopify.com/
  4. https://www.shopify.com/tour
  5. https://www.shopify.com/pricing
  6. https://www.shopify.com/pos
  7. https://www.shopify.com/magic
  8. https://www.shopify.com/blog
  9. https://apps.shopify.com/
  10. https://community.shopify.com/c/shopify-community/ct-p/en
  11. https://www.trustpilot.com/review/www.shopify.com
  12. https://squareup.com/us/en
  13. https://squareup.com/us/en/pos
  14. https://squareup.com/us/en/online-store
  15. https://squareup.com/us/en/appointments
  16. https://squareup.com/us/en/pricing
  17. https://www.trustpilot.com/review/squareup.com
  18. https://notion.so/
  19. https://notion.so/product/ai
  20. https://www.notion.so/pricing
  21. https://www.wix.com/
  22. https://www.wix.com/ecommerce/website
  23. https://www.wix.com/pricing
  24. https://www.wix.com/studio
  25. https://www.trustpilot.com/review/www.wix.com
  26. https://www.hubspot.com/
  27. https://www.hubspot.com/products/crm
  28. https://www.hubspot.com/pricing/crm
  29. https://www.trustpilot.com/review/www.hubspot.com
  30. https://www.dingtalk.com/en
  31. https://www.larksuite.com/
  32. https://www.larksuite.com/pricing
  33. https://www.wecom.com/
  34. https://www.salesforce.com/products/small-business/
  35. https://www.salesforce.com/einstein/
  36. https://slack.com/
  37. https://slack.com/features/ai
  38. https://www.microsoft.com/en-us/microsoft-365/copilot
  39. https://workspace.google.com/solutions/small-business/
  40. https://workspace.google.com/solutions/ai/
  41. https://calendly.com/
  42. https://calendly.com/pricing
  43. https://www.honeybook.com/
  44. https://www.honeybook.com/pricing
  45. https://www.trustpilot.com/review/honeybook.com
  46. https://www.thryv.com/
  47. https://www.thryv.com/pricing/
  48. https://www.gohighlevel.com/
  49. https://www.gohighlevel.com/pricing
  50. https://www.trustpilot.com/review/gohighlevel.com
  51. https://www.zoho.com/one/
  52. https://www.zoho.com/crm/
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
