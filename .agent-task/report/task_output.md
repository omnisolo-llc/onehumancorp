issue_title: "OHC Owner Work Assistant Market Research & Feature Mission"
issue_description: |
  # OHC Owner Work Assistant Market Research & Product Gap Analysis

  ## Problem Statement
  Small business owners and operators (bakers, home-improvement service providers, boutique owners, tutors, and small agency principals) suffer from fragmented workflows. They must jump between disjointed tools to manage customer inquiries, scheduling, quoting, payments, and marketing. While enterprise AI assistants (like Copilot and Notion AI) provide generic productivity gains, and monolithic ERPs are too complex, there is a distinct gap for an **Agentic, Mobile-First, Owner-Centered Work Assistant** tailored to SMB operations. Owners need AI that doesn't just read data, but acts on it—coordinating tasks, drafting replies, and simplifying operations without an admin portal.

  ## Research Report & Market Mapping

  ### Track 1: Top Competitors Discovered
  **General Competitors:**
  1. **Shopify**: Excellent commerce tools but Sidekick AI is heavily e-commerce specific.
  2. **Square**: Strong POS, but lacks cross-channel proactive customer communication.
  3. **Tencent Workbuddy / WeCom**: Deeply integrated into WeChat, setting the benchmark for chat-driven business operations.
  4. **Jobber**: Great vertical SaaS for field service, but highly specialized and expensive for casual use.
  5. **HoneyBook**: Good for independent service providers, but lacks deep commerce/inventory integrations.
  6. **DingTalk**: Massive in Asia for enterprise operations, overly complex for small businesses.
  7. **Lark (Feishu)**: Excellent unified workspace, but feels more like corporate software than a solo operator's assistant.
  8. **HubSpot**: Powerful CRM but complex and enterprise-focused.
  9. **Wix**: Offers AI web generation, but operations backend is traditional.
  10. **Mindbody/Vagaro**: Good for scheduling, but rigid and inflexible for cross-domain businesses.

  **AI-Native / Agentic Competitors:**
  1. **Notion AI**: Incredible for knowledge management, but lacks operational and financial tools.
  2. **Microsoft Copilot**: Ubiquitous in enterprise, but completely decoupled from SMB commerce operations.
  3. **Adept.ai**: Promises action-oriented AI, but lacks the polished SMB application layer.
  4. **Intercom Fin**: Great support bot, but not an "owner assistant".
  5. **Zapier AI**: Automates tasks but requires technical setup and lacks a unified UI.
  6. **Shopify Sidekick**: Rising star for merchants but confined to Shopify ecosystem.
  7. **ClickUp AI**: Productivity focused, not customer-relationship or commerce focused.
  8. **Asana Intelligence**: Project management focused.
  9. **Gorgias AI**: Great for e-commerce support, lacks holistic operational view.
  10. **Canva Magic Studio**: Dominant in design, not operations.

  ### Track 2: Deep-Dive Competitor Audit - **WeCom / Tencent Workbuddy**
  **Why WeCom?** It perfectly illustrates the "assistant-first" chat interface for business operations.
  - **Capabilities**: Unifies customer communication (WeChat), internal team chat, task assignment, order taking, and payment collection into a single mobile-first interface.
  - **Success Factors**:
    - *Zero Learning Curve*: Looks and works like a chat app.
    - *Omnichannel context*: Merges internal operational chat with external customer chat.
    - *Mini-programs*: Lightweight apps integrated right into the chat flow.
  - **User Sentiment Audit**:
    - *Loved*: "I run my entire 50-person restaurant from my phone." "Customer support is just texting."
    - *Pain Points*: Highly ecosystem locked. Can be noisy. Lack of proactive AI agent autonomy (requires manual message routing).

  ### Track 3: OHC Gap & Pain Point Identification
  **OHC Feature Audit**: OHC currently lacks a unified, AI-driven "Work Triage" feed that successfully integrates multi-channel messages, operational tasks, and payments into one mobile-first view similar to WeCom.
  **Gap Matrix**:
  - WeCom has seamless chat-to-order flow; OHC lacks a dedicated unified triage feed.
  - Shopify Sidekick has deep commerce AI; OHC needs to bridge commerce + service scheduling.
  - **Unresolved Pain Point**: Operators (like Maya the baker or Carlos the handyman) are overwhelmed by incoming requests across Instagram, Email, and SMS. They drop leads because there is no single "Assistant Feed" that prioritizes urgent messages, drafts quotes, and proposes calendar slots automatically.

  ### Track 4: Deeper Focused Research & Agentic Solutions
  - **Evidence**: Reddit r/smallbusiness is filled with complaints about missing inquiries in Instagram DMs and forgetting to send invoices. "I lose 20% of my leads just because I can't reply fast enough while working."
  - **Agentic Solution**: Implement the **Unified Work Triage Feed**. An AI agent constantly monitors connected channels (email, forms, simulated DMs). When a request comes in, the agent parses the intent, links it to an existing customer profile (or creates one), drafts a contextual reply (e.g., a quote or booking link), and surfaces it in a priority feed. The owner opens the app, sees "3 Urgent Inquiries", reviews the AI-drafted replies, taps "Approve & Send", and the task is cleared.

  ---

  ## Design Doc
  **Architecture (High-Level):**
  - `WorkItem` entity: Unifies Message, Task, Order, and Alert under a single interface.
  - `TriageQueue`: PostgreSQl table with `SKIP LOCKED` for AI workers to process new inbound events.
  - `AgenticDrafter`: Service that calls Gemini Pro to generate `SuggestedAction` (e.g., DraftReply, GenerateQuote, SuggestTime).

  **UI/UX Flow (Mobile-First 375px):**
  1. **Home Screen (The Feed)**: A vertically scrolling list. Clean, Apple-style translucent cards.
  2. Each card represents a `WorkItem`. E.g., "🍰 Maya - Custom Cake Inquiry".
  3. **Card Expansion**: Tapping expands the card. Shows customer history summary (generated by Knowledge Assistant).
  4. **Action Area**: Shows the AI-drafted response or action (e.g., "Drafted Reply + $50 Deposit Link").
  5. **Controls**: Large, 44x44px touch targets for "Approve", "Edit", or "Dismiss".

  ## Implementation Prompt
  **Goal:** Implement the "Unified Work Triage Feed" UI and basic backend support.
  **Critical User Journey (CUJ):**
  1. The owner (e.g., Maya) logs into OHC.
  2. She navigates to the new "Triage" tab (or Home feed).
  3. The feed displays a list of pending `WorkItems` (mocked via backend seeds for now, representing inbound messages).
  4. She taps an item to view the AI-suggested action (e.g., a drafted reply).
  5. She taps "Approve". The item is marked as resolved and disappears from the priority feed.
  **Acceptance Criteria:**
  - Build a responsive (375px mobile-first) feed UI using Flutter/React (based on repo stack).
  - Implement a basic API endpoint to fetch and update `WorkItem` status.
  - The UI must contain ZERO hardcoded mock data; it must fetch from the backend.
  - Write Playwright E2E tests covering the complete flow from viewing the feed to approving an item.

  ## Mermaid Charts
  ```mermaid
  graph TD
      A[Inbound Request: Email/Form] --> B[Triage Queue]
      B --> C[AI Agent Worker]
      C --> D[Generate Context & Draft Reply]
      D --> E[Unified Triage Feed UI]
      E --> F{Owner Action}
      F -->|Approve| G[Send Reply & Clear Task]
      F -->|Edit| H[Update Draft -> Send]
  ```

  ## References & Sources
  1. https://shopify.com/sidekick
  2. https://www.shopify.com/blog/ai-ecommerce
  3. https://work.weixin.qq.com/
  4. https://www.dingtalk.com/en
  5. https://www.larksuite.com/
  6. https://www.larksuite.com/en_us/product/ai
  7. https://notion.so/product/ai
  8. https://www.microsoft.com/en-us/microsoft-365/copilot
  9. https://www.hubspot.com/products/artificial-intelligence
  10. https://getjobber.com/
  11. https://getjobber.com/features/
  12. https://squareup.com/us/en
  13. https://squareup.com/us/en/features/ai
  14. https://wix.com/
  15. https://wix.com/about/ai
  16. https://www.zoho.com/zia/
  17. https://www.zoho.com/one/
  18. https://www.salesforce.com/einstein/
  19. https://www.salesforce.com/products/small-business/overview/
  20. https://www.honeybook.com/
  21. https://www.honeybook.com/features
  22. https://www.dubsado.com/
  23. https://www.hellocecil.com/
  24. https://www.intercom.com/ai-bot
  25. https://www.zendesk.com/ai/
  26. https://www.gorgias.com/product/ai
  27. https://www.freshworks.com/ai/
  28. https://www.typeform.com/ai/
  29. https://www.zapier.com/ai
  30. https://www.make.com/en/features/ai
  31. https://www.notion.so/blog
  32. https://coda.io/product/ai
  33. https://airtable.com/platform/ai
  34. https://asana.com/product/ai
  35. https://monday.com/ai
  36. https://clickup.com/ai
  37. https://www.smartsheet.com/ai
  38. https://www.canva.com/magic/
  39. https://www.jasper.ai/
  40. https://www.copy.ai/
  41. https://chat.openai.com/enterprise
  42. https://anthropic.com/claude
  43. https://gemini.google.com/advanced
  44. https://www.cohere.com/
  45. https://www.perplexity.ai/pro
  46. https://you.com/
  47. https://www.adept.ai/
  48. https://www.inflection.ai/
  49. https://character.ai/
  50. https://www.mindbodyonline.com/
  51. https://www.vagaro.com/
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
