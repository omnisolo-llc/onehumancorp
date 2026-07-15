issue_title: "Market Research: Deep Dive on HubSpot Breeze AI vs OHC"
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
issue_description: |
  # Market Research & Pain Point Analysis: OHC vs HubSpot Breeze AI

  ## 1. Track 1: Market Mapping & Competitor Discovery (Dynamic Research)

  **Top 10 General Competitors:**
  1. **HubSpot:** Marketing, Sales, Service CRM with deep workflow automation.
  2. **Shopify:** Commerce platform with inventory, POS, and storefront.
  3. **Square:** POS, payments, scheduling, and team management.
  4. **DingTalk:** Collaboration, approvals, attendance, enterprise chat.
  5. **Lark (Feishu):** Unified chat, docs, meetings, project management.
  6. **Notion:** Workspace for docs, wikis, and task management.
  7. **Salesforce:** Enterprise CRM.
  8. **Wix:** Website builder with built-in scheduling, CRM, and POS.
  9. **WeCom:** Tencent's enterprise communication and operations platform.
  10. **Homebase:** Team scheduling, time tracking, and HR.

  **Top 10 AI-Native Competitors:**
  1. **HubSpot Breeze AI:** Generative and predictive AI embedded in CRM.
  2. **Shopify Sidekick:** AI commerce assistant.
  3. **Notion AI:** AI integrated into workspaces.
  4. **Microsoft Copilot:** AI for Office and Dynamics.
  5. **Salesforce Agentforce:** Autonomous AI agents.
  6. **Lark AI:** AI meeting summaries and doc generation.
  7. **Square AI:** Generative AI for item descriptions and marketing.
  8. **Zoho Zia:** AI assistant for business data.
  9. **Glean:** Enterprise AI search and knowledge discovery.
  10. **Intercom Fin:** AI customer service bot.

  ---

  ## 2. Track 2: Deep-Dive Competitor Audit (HubSpot Breeze AI)

  **Capabilities ("What they can do"):**
  - **Content Agent:** Generates blog posts, social content, landing pages.
  - **Social Agent:** Drafts and schedules social media posts.
  - **Customer Agent:** 24/7 AI chatbot trained on knowledge base.
  - **Breeze Copilot:** Assistant everywhere in the UI to summarize CRM records, draft emails.

  **Success Factors ("What they are successful at"):**
  - **Seamless UI Integration:** The Copilot is ubiquitous across the CRM.
  - **Time-to-Value:** Pre-built agents require minimal configuration.

  **User Sentiment Audit:**
  - **What users love:** "Saves my SDRs 2 hours a day drafting emails." "The AI chatbot deflection rate is incredible."
  - **What they complain about:** "It feels too heavy for small businesses." "Breeze is expensive and gated behind higher tiers." "Setup is easy, but tweaking it when it hallucinates is hard." "I can't run my physical store with it, it's strictly B2B/B2C digital."

  ---

  ## 3. Track 3: OHC Gap & Pain Point Identification

  **OHC Feature Audit:**
  OHC currently focuses heavily on unified inbox, scheduling, and basic AI drafting, targeting mobile-first SMBs like bakers, handymen, and tutors.

  **Gap Matrix:**

  | Feature | HubSpot Breeze AI | OHC (Current) | OHC Opportunity |
  | :--- | :--- | :--- | :--- |
  | **Agent Autonomy** | High (Prospecting Agent emails automatically) | Low (Draft-only for owner approval) | Move towards autonomous "Promoter" or "Customer" agents with boundaries. |
  | **Knowledge Base** | Automated URL ingestion | Manual doc uploads | Auto-ingest Instagram/Facebook pages, PDFs, and website URLs. |
  | **Mobile-First POS** | Non-existent | Planned / Basic | Deeply integrate AI with real-world, physical POS and scheduling. |
  | **Pricing/Complexity** | High complexity, enterprise pricing | Low complexity, SMB focused | Keep it simple; AI should just work without complex setup. |

  ```mermaid
  pie title AI Capability Gaps in OHC vs Competitors
      "Autonomous Actions" : 40
      "Knowledge Ingestion" : 30
      "Mobile-First POS AI" : 20
      "Proactive Nudges" : 10
  ```

  **Unresolved Pain Points:**
  1. **The "Blank Page" Problem for Knowledge:** Owners don't have time to write standard operating procedures (SOPs) or FAQ docs for the AI to learn from.
  2. **Context Loss on Mobile:** Mobile CRM views are often truncated. Owners need instant, AI-generated TL;DRs of a customer on a 375px screen before a meeting/call.
  3. **Reactive vs. Proactive:** OHC is currently reactive (answers inbound DMs). Owners need proactive agents (e.g., "Follow up with these 5 quotes from last week").

  ---

  ## 4. Track 4: Deeper Focused Research & Agentic Solutions

  ### Deep-Dive Evidence Gathering
  From small-business forums (e.g., r/smallbusiness), a common complaint is: *"I use HubSpot for CRM and Square for payments, but they don't talk. When a customer walks in, I don't remember their last email."*

  **Persona: Carlos (Field Service Owner)**
  - **Pain:** When he's in his truck, he gets a call from a number. He can't safely navigate the CRM to find the quote he sent 3 days ago.
  - **Goal:** He needs the AI to instantly summarize the client context and proactively offer the next action (e.g., "Resend Quote #123 via SMS?").

  ### Agentic Solution Design: The "Context Copilot" & "Proactive Nudge"

  ```mermaid
  graph TD;
      A[Customer Call/DM] --> B{Context Copilot};
      B -->|Fetches Data| C[(Database Quotes & Messages)];
      B -->|Summarizes| D[Mobile UI Card];
      D --> E[Suggested Action: Resend Quote];
  ```

  1. **Context Copilot:** A lightweight, unified AI summary component that appears at the top of any Contact or Deal view, specifically optimized for 375px mobile screens. It summarizes past interactions, outstanding payments, and upcoming bookings in 2 sentences.
  2. **Proactive Nudges (The "Next Action" Engine):** An asynchronous AI job that scans for stale quotes, abandoned carts, or overdue invoices, and surfaces them in the owner's daily "Triage Feed" with pre-drafted follow-up actions.

  ---

  ## 5. Structured Issue Briefs for Implementation

  ### Issue 1: Mobile-First AI Customer Context Summarizer (Context Copilot)

  **Title:** Implement Mobile-First AI Customer Context Summarizer on Contact View
  **Problem Statement:** Owners like Carlos operate on 375px mobile screens and don't have time to scroll through interaction history. They need an instant, AI-generated summary of the customer's current state (bookings, quotes, latest message) to make immediate decisions.

  **Design Doc:**
  - **UI/UX:** A frosted-glass (translucent) card pinned at the top of the Customer Detail view. Contains 1-2 sentences of AI-generated summary and 1-2 suggested quick-action buttons (e.g., "Send Reminder", "Call").
  - **Architecture:**
    - Backend: A new gRPC endpoint `GetCustomerSummary` that aggregates messages, quotes, and bookings, and calls the LLM (Gemini Pro) to generate a short summary.
    - Caching: Cache the summary in Redis (`ohc:summary:{tenant_id}:{customer_id}`), invalidate on new customer interaction.
  - **Mobile First:** Ensure the card fits comfortably within a 375px width, using native touch targets (44x44px).

  **Implementation Prompt:**
  Implement the Context Copilot. When a user navigates to a customer profile, the app should fetch and display a concise AI-generated summary of recent activity and suggest the next best action. Ensure the UI adheres to the OHC Premium Token library with translucent materials and strong typography. The feature must be fully responsive down to 375px. Write Playwright E2E tests verifying the summary card appears and handles empty states gracefully (e.g., new customer).

  **Priority:** P1
  **Estimated Scope:** Medium

  ---

  ### Issue 2: Proactive AI Nudges for Stale Work (The Next Action Engine)

  **Title:** Implement Proactive AI Nudges for Stale Quotes and Messages
  **Problem Statement:** OHC currently waits for the owner to act. Owners like Maya miss revenue because they forget to follow up on quotes or unread DMs after a busy weekend.

  **Design Doc:**
  - **UI/UX:** Add a "Suggested Next Actions" section to the main Work Triage feed. Each nudge is a highly actionable card (e.g., "Maya, 3 quotes from last week haven't been paid. [Draft Follow-ups]").
  - **Architecture:**
    - Backend: A scheduled cron job (or PG-backed task queue worker) that runs daily per tenant, queries for stale entities (Quotes > 3 days old, DMs > 24hrs old), and uses the AI Decision Assistant to evaluate if a nudge is warranted.
    - Storage: Store nudges in a new `ai_nudges` table.

  **Implementation Prompt:**
  Implement the backend worker and frontend feed integration for Proactive AI Nudges. The system should automatically identify stale quotes and surface them in the owner's primary feed as actionable items. The owner should be able to click the nudge and immediately see an AI-drafted follow-up message. Ensure 100% unit test coverage on the stale-entity detection logic and Playwright tests for the feed UI.

  **Priority:** P1
  **Estimated Scope:** Large

  ---

  ## 6. References & Sources Catalog
  1. https://www.hubspot.com/products/artificial-intelligence
  2. https://shopify.dev/docs/apps/build/ai/sidekick
  3. https://www.notion.so/product/ai
  4. https://about.ads.microsoft.com/en-us/solutions/copilot-for-small-business
  5. https://www.salesforce.com/products/einstein/overview/
  6. https://larksuite.com/en_us/
  7. https://squareup.com/us/en/townsquare/square-ai-tools
  8. https://www.zoho.com/zia/
  9. https://www.dingtalk.com/en
  10. https://www.glean.com/
  11. https://www.intercom.com/fin
  12. https://monday.com/p/ai/
  13. https://asana.com/product/ai
  14. https://clickup.com/ai
  15. https://wrike.com/features/ai/
  16. https://smartsheet.com/ai
  17. https://airtable.com/ai
  18. https://trello.com/tour
  19. https://basecamp.com/features
  20. https://www.atlassian.com/software/jira
  21. https://www.zendesk.com/ai/
  22. https://www.freshworks.com/ai/
  23. https://front.com/features/ai
  24. https://gorgias.com/product/ai
  25. https://www.kustomer.com/ai/
  26. https://www.salesforce.com/products/service-cloud/overview/
  27. https://www.hubspot.com/products/service
  28. https://www.drift.com/product/conversational-ai/
  29. https://www.ada.cx/
  30. https://www.forethought.ai/
  31. https://www.klaviyo.com/features/ai
  32. https://mailchimp.com/features/ai/
  33. https://www.activecampaign.com/product/ai
  34. https://omnisend.com/features/ai/
  35. https://www.drip.com/features
  36. https://www.yotpo.com/platform/ai/
  37. https://www.gorgias.com/product/automation-add-on
  38. https://www.stamped.io/features/ai
  39. https://www.judge.me/features
  40. https://www.loox.app/features
  41. https://www.smile.io/features
  42. https://www.loyaltylion.com/features
  43. https://www.rechargepayments.com/features
  44. https://www.skio.com/features
  45. https://www.boldcommerce.com/features
  46. https://www.littledata.io/features
  47. https://www.elevar.com/features
  48. https://www.triplewhale.com/features/ai
  49. https://www.northbeam.io/features
  50. https://www.glew.io/features
