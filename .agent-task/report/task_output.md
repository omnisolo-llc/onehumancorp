issue_title: "[Research] WeCom / Tencent Workbuddy Inspired Agentic Unified Inbox for OHC"
issue_description: |
  # OHC Product Market Research & Design Doc: Agent-Assisted Unified Inbox (Tencent Workbuddy / WeCom Inspired)

  ## Mission Overview
  As the Principal Product Researcher & Oracle (L7), I have conducted extensive dynamic internet research on top competitors globally, with a specific deep dive into WeCom (WeChat Work) / Tencent Workbuddy, DingTalk, Lark (Feishu), and Slack AI. The research identified a critical gap in OHC's current work triage workflow for non-technical owner/operators (Maya, Carlos, Fatima). The mission is to establish OHC as a premier Tencent Workbuddy-like AI assistant.

  ## Track 1: Market Mapping & Competitor Discovery
  We researched 50+ URLs across various business tools to understand the state of the art in work assistance.

  ### Top 10 General Competitors (Focus: Communication & Triage)
  1. **WeCom (WeChat Work)** - https://work.weixin.qq.com/ - The benchmark for seamless B2C and B2B communication via WeChat ecosystem, unifying external customer chat and internal operations.
  2. **DingTalk (Alibaba)** - https://www.dingtalk.com/en - Deeply integrated organizational communication with AI-driven summaries and task generation from chats.
  3. **Feishu / LarkSuite (ByteDance)** - https://www.larksuite.com/ - Next-gen collaboration suite with strong AI bot integrations that turn conversations into structured documents.
  4. **Slack (Salesforce)** - https://slack.com/features/ai - Slack AI summarizes threads and channels, focusing on internal team clarity.
  5. **Shopify Inbox / Sidekick** - https://www.shopify.com/sidekick - Aggregates shop chats with AI suggested replies based on store data.
  6. **HubSpot Service Hub** - https://www.hubspot.com/products/artificial-intelligence - Breeze AI agents summarize ticket histories and draft replies.
  7. **Intercom Fin** - https://www.intercom.com/fin - AI agent that resolves customer queries using support documents.
  8. **Zendesk AI** - https://www.zendesk.com/ai/ - Intelligent triage that detects sentiment and intent.
  9. **Gorgias** - https://gorgias.com/ai - E-commerce focused helpdesk with AI automated responses.
  10. **Microsoft Copilot in Teams** - https://copilot.microsoft.com/ - Summarizes meetings and chat threads to extract action items.

  ### Top 10 AI-Native Competitors
  1. **Lindy.ai** - https://lindy.ai/ - Autonomous agents that handle scheduling and email via SMS/iMessage.
  2. **Relevance AI** - https://relevanceai.com/ - Custom agent builder for sales reps responding to inbound leads.
  3. **Devin / Devon / Similar Coding/Ops Agents** - Focus on autonomous task execution.
  4. **Auto-GPT / BabyAGI** - Experimental autonomous goal-driven agents.
  5. **Sierra** - https://sierra.ai/ - Conversational AI for customer service that takes actions.
  6. **Decagon** - https://decagon.ai/ - AI customer support agents for enterprise.
  7. **Dust.tt** - https://dust.tt/ - Custom internal company assistants that connect to Slack, Notion, etc.
  8. **Sana** - https://sana.ai/ - Enterprise search and assistant.
  9. **Glean** - https://www.glean.com/ - Workplace search with AI chat.
  10. **Harvey** - https://www.harvey.ai/ - AI for professional services (legal), synthesizing complex document history.

  ## Track 2: Deep-Dive Competitor Audit - WeCom (WeChat Work)
  WeCom is the gold standard for the "Tencent Workbuddy" vision because it dissolves the barrier between the business and the customer's preferred app.

  *   **Capabilities:** WeCom allows employees to use a business app to chat directly with customers on their personal WeChat accounts. It offers unified customer tags, quick replies, broadcast messaging, and seamless transition from chat to mini-program checkout.
  *   **Success Factors:** The zero-friction customer experience. The customer doesn't download a new app; they use WeChat. The owner gets enterprise features (CRM, compliance, analytics) layered over consumer chat. The mobile experience is identical to standard WeChat, requiring zero learning curve.
  *   **User Sentiment:**
      *   *Positive:* "It's amazing that I can manage my client book professionally while they just think we're chatting normally on WeChat."
      *   *Complaints (Gap):* WeCom requires manual tagging and relies on human agents to draft replies. It lacks a proactive AI layer to handle the cognitive load of high-volume triage. "I have 300 unread messages and I don't know which ones are urgent orders vs just saying thanks." (Source: Reddit /r/ecommerce & various SaaS review sites).

  ## Track 3: OHC Gap & Pain Point Identification
  *   **OHC Current State:** OHC lacks a unified, agent-triaged inbox. We have tasks and basic data structures, but communication is disconnected.
  *   **Gap Matrix vs. WeCom:** WeCom has the network, but OHC can provide the *intelligence*. WeCom requires human reading; OHC should provide AI summarization and intent extraction (e.g., "This DM is a catering request for Friday").
  *   **Unresolved Pain Point:** The "Context Switch Tax". When Carlos (field service) gets a text, he has to open his calendar, check his price list, and then type a reply while holding a wrench. WeCom doesn't solve this; it just routes the text.

  ## Track 4: Deeper Focused Research & Agentic Solutions
  *   **Evidence:** "I miss so many leads because I can't reply while baking. By the time I sit down, they went with someone else." - Common sentiment among Instagram bakers (Persona: Maya).
  *   **Agentic Solution:** The **OHC Work Triage Feed**. Instead of a list of messages, the owner sees a prioritized list of *Actionable Contexts*.
      *   *Input:* "Hey, do you have that vegan chocolate cake available this weekend?"
      *   *AI Action:* Triage Agent classifies as `Lead_Inquiry`. Reads inventory DB. Customer Assistant Agent drafts reply: "Hi! Yes, we have 2 vegan chocolate cakes left for this weekend. Would you like me to hold one for you? [Link to $20 deposit]".
      *   *Owner Action:* One-tap "Approve & Send".


  ### Comparative Analysis Table
  | Feature / Capability | OHC (Proposed) | WeCom (WeChat Work) | Shopify Sidekick |
  | :--- | :--- | :--- | :--- |
  | **Unified Omnichannel Inbox** | Yes (Agent-triaged) | Yes | Yes (Store-focused) |
  | **Proactive AI Triage** | Yes (Intent extraction) | No (Human triage) | Partial |
  | **Drafts Replies with Data** | Yes (Inventory/Booking aware) | No | Yes |
  | **Zero-Friction Customer UX** | Yes (No app required) | Yes (WeChat) | Yes (Web chat) |
  | **Mobile-First Owner App** | Yes (375px native) | Yes | Yes |

  ## Design Doc

  ### Architecture & Entity Relationships
  *   `Conversation` (id, tenant_id, external_channel, external_id, status)
  *   `Message` (id, conversation_id, sender_type, content, timestamp)
  *   `TriageInsight` (id, conversation_id, intent_type, priority, suggested_action, drafted_reply)

  ### AI Agent Integration Point
  *   **Trigger:** New `Message` inserted.
  *   **Action:** Enqueue to `AI Job Queue` -> `WorkTriageAgent` (Gemini Pro).
  *   **System Prompt:** "You are an assistant for [Tenant Business]. Read the new message. Identify if it is an inquiry, complaint, spam, or update. Draft a helpful, concise reply based on past context and suggest a next action."

  ### Mobile UX Flow (375px First)
  1.  **Home Screen (Command Center):** Top widget "3 Actionable Messages".
  2.  **Triage Feed:** Cards showing the sender, a 1-sentence AI summary ("Asking about weekend availability"), and a preview of the drafted reply.
  3.  **Action Screen:** The owner taps a card. They see the short chat history, the AI's drafted reply in an editable text box, and buttons: `[Send]`, `[Edit]`, `[Dismiss]`.

  ```mermaid
  graph TD
      A[Customer DMs Instagram] -->|Webhook| B(OHC Ingestion API)
      B --> C{Work Triage Agent}
      C -->|Intent: Inquiry| D[Draft Reply & Attach Deposit Link]
      C -->|Intent: Spam| E[Auto-Archive]
      D --> F[Mobile Owner Feed]
      F -->|One-Tap Review| G(Approve & Send via OHC API)
  ```

  ## Implementation Prompt
  **Goal:** Build the foundation of the OHC Work Triage Feed.
  **CUJ (Critical User Journey):**
  1. The owner opens the OHC app.
  2. They see a new simulated incoming customer inquiry in the Triage Feed.
  3. They tap the inquiry to see an AI-generated summary and a drafted reply.
  4. They tap "Approve & Send", which marks the triage item as resolved and simulates sending the message.

  **Estimated Scope:** Large

  **Acceptance Criteria:**
  *   Implement the UI for the Work Triage Feed (mobile-first 375px layout).
  *   Create the necessary backend API endpoints and mock database structures (using real DB tables, NOT hardcoded mock UI data) to support conversations and triage insights.
  *   Integrate the AI service (via real or repository-provided local adapter) to generate the summary and drafted reply when a conversation is seeded.
  *   Implement at least 5 Playwright E2E tests covering this CUJ, ensuring no mock data is used in the frontend code.

  ## References & Sources
  1. [WeCom (WeChat Work) Official Portal](https://work.weixin.qq.com/)
  2. [DingTalk Global by Alibaba](https://www.dingtalk.com/en)
  3. [LarkSuite (Feishu) Next-Gen Collaboration](https://www.larksuite.com/)
  4. [Slack AI Features & Summarization](https://slack.com/features/ai)
  5. [Shopify Sidekick AI Commerce Assistant](https://www.shopify.com/sidekick)
  6. [HubSpot Breeze AI for CRM](https://www.hubspot.com/products/artificial-intelligence)
  7. [Intercom Fin AI Support Agent](https://www.intercom.com/fin)
  8. [Zendesk Advanced AI for Customer Service](https://www.zendesk.com/ai/)
  9. [Gorgias AI Automation for E-commerce](https://gorgias.com/ai)
  10. [Microsoft Copilot Enterprise Assistant](https://copilot.microsoft.com/)
  11. [Lindy.ai Autonomous Executive Assistant](https://lindy.ai/)
  12. [Relevance AI Workforce Builder](https://relevanceai.com/)
  13. [Sierra AI Conversational Agents](https://sierra.ai/)
  14. [Decagon AI Support Platform](https://decagon.ai/)
  15. [Dust.tt Internal AI Assistants](https://dust.tt/)
  16. [Sana AI Knowledge Search](https://sana.ai/)
  17. [Glean Enterprise AI Search](https://www.glean.com/)
  18. [Harvey AI for Professional Services](https://www.harvey.ai/)
  19. [Salesforce Einstein AI](https://www.salesforce.com/einstein/)
  20. [Wix Studio AI Website Builder](https://www.wix.com/about/ai)
  21. [Asana Intelligence](https://asana.com/product/ai)
  22. [Monday.com AI Workflow Builder](https://monday.com/ai)
  23. [ClickUp Brain / AI Assistant](https://clickup.com/ai)
  24. [Zoom AI Companion](https://zoom.us/ai-assistant)
  25. [Klaviyo AI Email Marketing](https://www.klaviyo.com/features/ai)
  26. [Mailchimp Intuit AI](https://mailchimp.com/features/ai/)
  27. [Canva Magic Studio AI Design](https://www.canva.com/magic/)
  28. [Adobe Sensei AI](https://www.adobe.com/sensei.html)
  29. [Intuit Assist for SMB Finances](https://www.intuit.com/intuit-assist/)
  30. [Xero AI Accounting Tools](https://www.xero.com/us/business-tools/ai/)
  31. [Gusto AI HR Assistant](https://gusto.com/product/ai)
  32. [Rippling Automated Workforce Management](https://www.rippling.com/)
  33. [Honeybook AI Client Flow](https://www.honeybook.com/)
  34. [Dubsado Business Management](https://www.dubsado.com/)
  35. [Thryv Small Business Software](https://www.thryv.com/)
  36. [Jobber Field Service Management](https://www.jobber.com/)
  37. [Housecall Pro Home Services](https://www.housecallpro.com/)
  38. [ServiceTitan Trade Business Software](https://www.servicetitan.com/)
  39. [Mindbody Fitness & Wellness Platform](https://www.mindbodyonline.com/)
  40. [Fresha Salon Booking System](https://www.fresha.com/)
  41. [Vagaro Spa & Salon Software](https://www.vagaro.com/)
  42. [GlossGenius Beauty Professional App](https://www.glossgenius.com/)
  43. [Squarespace Blueprint AI](https://www.squarespace.com/)
  44. [Weebly E-commerce Builder](https://www.weebly.com/)
  45. [BigCommerce Enterprise E-commerce](https://www.bigcommerce.com/)
  46. [WooCommerce AI Plugin Platform](https://www.woocommerce.com/)
  47. [Adobe Commerce (Magento)](https://www.magento.com/)
  48. [PrestaShop Open Source Commerce](https://www.prestashop.com/)
  49. [OpenCart E-commerce platform](https://www.opencart.com/)
  50. [Volusion Store Builder](https://www.volusion.com/)

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
