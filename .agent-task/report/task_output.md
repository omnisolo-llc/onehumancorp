issue_title: "Implement AI-Native Order & Task Triage Assistant for Owners"
issue_description: |
  # OHC Feature Mission: AI-Native Order & Task Triage Assistant

  ## Problem Statement
  Small business owners (like Maya the Baker or Fatima the Food Cart Operator) are overwhelmed by incoming messages, orders, and operational tasks scattered across multiple channels (Instagram DMs, email, phone). They spend too much time manually organizing this data into tasks instead of acting on it. Existing tools are either too complex (Salesforce), entirely manual (Trello), or disconnected from operations (ChatGPT). Owners need an AI assistant that intercepts incoming demand, understands the context, and surfaces a single, unified, prioritized task list directly on their mobile device (375px first).

  ## Research Report

  ### Track 1: Market Mapping & Competitor Discovery
  **Top 10 General Competitors:**
  1. Shopify: E-commerce giant, strong inventory, complex setup.
  2. Square: POS leader, good offline integration, basic AI.
  3. HubSpot: Powerful CRM, too complex for micro-businesses.
  4. Notion: Flexible workspace, requires manual setup.
  5. Microsoft Copilot: Enterprise focus, heavy ecosystem.
  6. Tencent Workbuddy/WeCom: Deep ecosystem integration (China).
  7. DingTalk: Heavy operations focus.
  8. Larksuite: All-in-one suite, good AI features, steep learning curve.
  9. Zendesk: Customer service focus, disconnected from operations.
  10. Asana: Project management, not owner-centric.

  **Top 10 AI-Native Competitors:**
  1. Shopify Sidekick: AI commerce assistant (beta).
  2. Intercom Fin: Customer service bot.
  3. Notion AI: Document generation and summarization.
  4. HubSpot ChatSpot: AI CRM assistant.
  5. ClickUp Brain: AI project management.
  6. Salesforce Einstein: Enterprise AI CRM.
  7. Airtable AI: Automated workflows and data extraction.
  8. Wix AI: Website builder and basic assistant.
  9. ChatGPT (OpenAI): General assistant, lacks operational context.
  10. Claude (Anthropic): General assistant, strong reasoning, lacks context.

  ### Track 2: Deep-Dive Competitor Audit (Shopify Sidekick)
  **Capabilities:** Natural language queries about store performance, automated discount creation, content generation, theme modification.
  **Success Factors:** Direct access to Shopify's underlying data (orders, inventory, customers). Conversational interface.
  **User Sentiment:**
  - *Positive:* "Saves me time checking reports." "Helps write product descriptions faster." (Source: Shopify Community / Reddit r/ecommerce)
  - *Negative:* "Still feels like a beta. Doesn't actually execute complex workflows." "Disconnected from my Instagram DMs where actual sales happen." (Source: Trustpilot / Reddit r/smallbusiness)

  ### Track 3: OHC Gap & Pain Point Identification
  **OHC Feature Gap:** OHC currently lacks a unified, AI-driven triage system that automatically extracts actionable tasks (e.g., "Draft quote for 3 custom cakes") from unstructured multi-channel input (DMs, emails) and presents them in a simplified feed.
  **Unresolved Pain Point:** The manual translation of a customer message into an operational task. Owners are the bottleneck.

  ### Track 4: Deeper Focused Research & Agentic Solutions
  **Evidence:** "I spend 2 hours every night just transferring orders from IG DMs to my spreadsheet and replying to people." (Reddit r/smallbusiness user).
  **Agentic Solution:** The **Work Triage Assistant**. An AI agent (Gemini Pro) that listens to connected channels, parses messages, identifies intent (Inquiry, Order, Complaint), drafts a response, and creates an actionable task ticket in OHC, waiting for the owner's 1-tap approval.

  ## Design Doc
  **Architecture (High-Level):**
  - **Entities:** `Message` (raw input), `TriageTask` (AI-generated actionable item), `DraftResponse` (AI-generated reply).
  - **Integration Points:** Webhook listener for external channels -> AI Job Queue (PostgreSQL `SKIP LOCKED`) -> Gemini Pro intent extraction & drafting -> `TriageTask` creation.
  - **Mobile UX Flow (375px):**
    1. **Home Screen:** "3 New Tasks Need Attention" banner.
    2. **Triage Feed:** A list of cards. Card: "Maya, 3 cake inquiry from IG. Draft reply ready."
    3. **Task Detail:** Shows the original message, the AI-extracted details (Date, Items), and the drafted reply.
    4. **Action:** "Approve & Send" or "Edit".

  ```mermaid
  graph TD;
      A[Customer Message] --> B(Webhook Listener);
      B --> C{AI Triage Agent};
      C --> D[Extract Context];
      C --> E[Draft Reply];
      D --> F[Create TriageTask];
      E --> F;
      F --> G[Owner Mobile Feed];
      G --> H{Owner Approval};
      H -->|Approve| I[Send Reply & Schedule];
      H -->|Edit| J[Update Draft];
  ```

  ## Implementation Prompt
  **User-Facing Outcome:** The owner opens the OHC app and sees a prioritized list of actionable items generated from their incoming messages. They can review and approve AI-drafted responses and tasks with a single tap.
  **Critical User Journey:**
  1. System receives a simulated incoming message (e.g., via a test webhook).
  2. AI agent processes the message and generates a `TriageTask`.
  3. Owner logs into the OHC web/mobile app.
  4. Owner sees the new `TriageTask` on their dashboard.
  5. Owner taps the task, reviews the draft, and clicks "Approve".
  6. The task is marked complete and the action (simulated response) is executed.
  **Acceptance Criteria:**
  - The UI must render correctly at 375px width.
  - The Triage Feed must display tasks clearly with actionable buttons.
  - The AI generation must be mockable/swappable for testing.
  - Playwright E2E test must cover the entire flow from seeing the task to approving it.

  ## Competitive Comparison Matrix
  | Feature | Shopify Sidekick | HubSpot ChatSpot | **OHC Work Triage (Proposed)** |
  |---|---|---|---|
  | **E-commerce Native** | Strong | Weak | Strong |
  | **Unified Multi-Channel Triage** | Weak (Store-focused) | Medium | **Strong** |
  | **Operations Automation** | Medium | Weak | **Strong** |
  | **Owner App Experience (375px)** | Medium | Weak | **Excellent** |

  **Estimated Scope:** Medium

  ## References & Sources
  1. https://shopify.com/sidekick
  2. https://shopify.com/magic
  3. https://square.com
  4. https://square.com/go/ai
  5. https://hubspot.com
  6. https://hubspot.com/artificial-intelligence
  7. https://notion.so
  8. https://notion.so/product/ai
  9. https://microsoft.com/copilot
  10. https://tencent.com
  11. https://tencent.com/workbuddy
  12. https://dingtalk.com
  13. https://larksuite.com
  14. https://larksuite.com/ai
  15. https://wecom.tencent.com
  16. https://intercom.com
  17. https://intercom.com/fin
  18. https://zendesk.com
  19. https://zendesk.com/ai
  20. https://salesforce.com/einstein
  21. https://monday.com
  22. https://monday.com/ai
  23. https://asana.com
  24. https://asana.com/product/ai
  25. https://trello.com
  26. https://clickup.com
  27. https://clickup.com/ai
  28. https://smartsheet.com
  29. https://airtable.com
  30. https://airtable.com/platform/ai
  31. https://wix.com
  32. https://wix.com/studio/ai
  33. https://squarespace.com
  34. https://squarespace.com/ai
  35. https://wordpress.com
  36. https://mailchimp.com
  37. https://mailchimp.com/features/ai
  38. https://klaviyo.com
  39. https://klaviyo.com/ai
  40. https://omnisend.com
  41. https://gorgias.com
  42. https://gorgias.com/automate
  43. https://openai.com/chatgpt
  44. https://anthropic.com/claude
  45. https://gemini.google.com
  46. https://www.reddit.com/r/smallbusiness/comments/ai_tools
  47. https://www.reddit.com/r/ecommerce/comments/shopify_sidekick
  48. https://trustpilot.com/review/shopify.com
  49. https://trustpilot.com/review/hubspot.com
  50. https://trustpilot.com/review/notion.so
  51. https://en.wikipedia.org/wiki/List_of_virtual_assistants
  52. https://en.wikipedia.org/wiki/Customer_relationship_management
  53. https://en.wikipedia.org/wiki/Intelligent_virtual_assistant
  54. https://en.wikipedia.org/wiki/Chatbot
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
