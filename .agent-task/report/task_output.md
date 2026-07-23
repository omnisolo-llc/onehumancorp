issue_title: "Implement Native Rust Omnichannel Chat to Replace Chatwoot"
issue_description: |
  # Mission Queue Protocol Brief

  **Problem Statement**:
  OHC currently relies on external systems (like Chatwoot) for omnichannel customer messaging. Small business owners like Maya (Baker) and Fatima (Food Cart) struggle with fragmented messaging across Instagram, WhatsApp, and Web Widgets. The external dependencies break the seamless agentic experience and conflict with our strict requirement to replace Chatwoot with a high-performance native Rust implementation in `onehumancorp/mono`.

  **Research Report**:
  - **Market Mapping**: Competitors like Shopify use distinct apps (Shopify Inbox) for chatting, creating friction. Operational tools like Jobber manage jobs well but lack deeply integrated AI chat. Chatwoot provides a strong architectural reference for omnichannel routing but as a Ruby on Rails monolith, it is too resource-heavy and decoupled from our core AI agents.
  - **Deep Dive (Shopify Sidekick & Chatwoot)**:
    - *Capabilities*: Chatwoot excels at multi-channel ingestion (WhatsApp, Web, API) and agent routing. Shopify Sidekick excels at context-aware commerce suggestions.
    - *Success Factors*: Unified UI, quick reply macros, and native API integration.
    - *User Sentiment*: Users love having all messages in one place but hate switching contexts between their "chat app" and their "store admin app" to complete actions (like creating an order).

  ## Persona-Specific Pain Points
  - **Maya (Home Baker)**: Maya currently sells custom cakes through Instagram DMs. She struggles with managing custom-order deposits because she has to manually switch between Instagram and her payment app. **Pain Point:** Fragmented DMs and manual payment requests.
  - **Fatima (Food Cart Operator)**: Fatima handles pre-orders for pickup. She operates with limited English and slow mobile data. **Pain Point:** No English-first tool works for her, and she lacks a unified inbox that automatically links WhatsApp/Instagram DMs to new or existing orders without deep setup.
  - **Carlos (Field Service Owner)**: Carlos runs his repair service entirely from his phone. **Pain Point:** Misses leads when busy because he cannot quickly quote or schedule from a chat interface without switching to a complex CRM app.
  - **Leo (Creator and Tutor)**: Leo handles online and in-person lessons. **Pain Point:** Manual booking chaos across multiple chat platforms; needs an AI to follow up on casual interest and turn it into recurring bookings directly within the chat thread.

  ## Actionable Recommendations (OHC should do X because Y evidence)
  - **OHC should build a Unified Inbox Agent because** evidence from Chatwoot and Shopify Inbox users indicates extreme friction when switching between chat and operational apps.
  - **OHC should implement one-tap agentic actions (like "Send Quote" or "Book Appointment") within the chat stream because** personas like Carlos and Leo are fully mobile and lose leads when forced to navigate complex CRM menus on a 375px screen.
  - **OHC should completely remove Chatwoot and build a native Rust multi-tenant system because** the current Ruby on Rails monolith consumes excessive resources and prevents deep, localized AI context integration across tenant schemas.

  **Design Doc**:
  - **Architecture**:
    - **Entity Types**: `Conversation`, `Message`, `Channel` (WhatsApp, Instagram, WebWidget), `Participant` (Customer, Agent/Owner, AI Bot).
    - **Integration Points**: Native Rust HTTP/WebSocket servers under `src/server/ohc/chat`. Replace all Chatwoot webhook endpoints with native Axum routes.
  - **UI/UX (Mobile-First 375px)**:
    - **The Unified Stream**: A single scrollable feed showing incoming messages tagged with their channel (e.g., an IG logo).
    - **Agentic Assistance**: Below the input bar, the AI (Gemini/MiniMax) provides 1-tap "Draft Reply", "Send Quote", or "Book Appointment" chips based on the conversation context.
    - **Translucent Styling**: Clean Apple/Ubiquiti-style hierarchy using OHC Premium Tokens.

  **Implementation Prompt**:
  - **Goal**: Build the foundational Rust APIs and database models (PostgreSQL) for a native omnichannel chat system. Implement the 375px mobile-first UI in Tauri to view conversations and send messages.
  - **Critical User Journey**:
    1. Maya receives an Instagram DM about a cake order.
    2. The message arrives in the OHC Native Inbox via WebSockets.
    3. The UI shows the message. The AI pre-drafts a response and a quote link.
    4. Maya taps "Approve & Send" on her phone, sending the reply back through the native Instagram API integration.
  - **Acceptance Criteria**:
    - Complete removal of Chatwoot dependencies.
    - Native Rust backend handling real-time WebSocket messaging.
    - Responsive Tauri UI (375px) displaying conversations.
    - E2E Playwright tests proving a message can be sent and received.

  **Priority**: P0
  **Estimated Scope**: Large

  ---

  ## Visual Excellence

  ### Competitive Landscape Matrix

  ```mermaid
  quadrantChart
      title Unified AI & Commerce vs Omnichannel Capability
      x-axis Low Omnichannel Chat --> High Omnichannel Chat
      y-axis Segmented Tools --> Unified Agentic AI
      quadrant-1 Native Agentic Powerhouses
      quadrant-2 Fragmented AI Bots
      quadrant-3 Traditional SaaS
      quadrant-4 Traditional Helpdesks
      "Shopify (+ Inbox)": [0.3, 0.4]
      "Jobber": [0.2, 0.2]
      "Chatwoot": [0.8, 0.1]
      "Microsoft Copilot": [0.4, 0.8]
      "OHC Native (Target)": [0.9, 0.9]
  ```

  ### Comparative Table: Feature Gap Analysis

  | Feature | OHC Native (Proposed) | Shopify Sidekick / Inbox | Jobber | Chatwoot |
  | :--- | :--- | :--- | :--- | :--- |
  | **Core Focus** | Agentic Work Assistant | E-commerce Storefront | Field Service Ops | General Helpdesk |
  | **Omnichannel Intake** | Unified Stream (IG, WA, Web) | Separate App (Inbox) | Minimal | Unified Stream |
  | **Agentic AI Action** | In-stream quotes/bookings | Sidekick suggestions | Manual Scheduling | Basic Macros |
  | **Architecture** | Native Rust (Micro-footprint) | Cloud Native SaaS | Cloud Native SaaS | Ruby on Rails Monolith |
  | **Mobile-First UX** | 375px Responsive PWA/Tauri | Separate Admin/Inbox apps | Dedicated Mobile App | Mobile App |

  ### References & Sources Catalog
  1. [Shopify Inbox Overview](https://www.shopify.com/inbox)
  2. [Jobber Communications](https://getjobber.com/features/client-communication/)
  3. [Chatwoot Source Repository](https://github.com/chatwoot/chatwoot)
  4. [Chatwoot Architecture Docs](https://www.chatwoot.com/docs/)
  5. [Shopify Sidekick Announcement](https://www.shopify.com/magic)
  6. [Tencent WeCom Features](https://work.weixin.qq.com/)
  7. [DingTalk Omnichannel](https://www.dingtalk.com/)
  8. [Feishu Messaging](https://www.feishu.cn/en/)
  9. [Notion AI Integration](https://www.notion.so/product/ai)
  10. [Microsoft Copilot for Sales](https://www.microsoft.com/en-us/ai/copilot-for-sales)
  11. [HubSpot Unified Inbox](https://www.hubspot.com/products/crm/shared-inbox)
  12. [Square Messages](https://squareup.com/us/en/messages)
  13. [Zendesk Omnichannel](https://www.zendesk.com/service/messaging/)
  14. [Intercom AI Bots](https://www.intercom.com/)
  15. [Front Shared Inbox](https://front.com/)
  16. [Gorgias Ecommerce Helpdesk](https://www.gorgias.com/)
  17. [Kustomer CRM](https://www.kustomer.com/)
  18. [Freshchat Multichannel](https://www.freshworks.com/live-chat-software/)
  19. [Gladly Customer Service](https://www.gladly.com/)
  20. [Tidio Live Chat](https://www.tidio.com/)
  21. [Drift Conversational AI](https://www.drift.com/)
  22. [Ada AI Chatbot](https://www.ada.cx/)
  23. [ManyChat Instagram Automation](https://manychat.com/)
  24. [Respond.io Inbox](https://respond.io/)
  25. [Trengo Omnichannel](https://trengo.com/)
  26. [MessageBird Inbox](https://messagebird.com/en/inbox/)
  27. [Twilio Flex](https://www.twilio.com/flex)
  28. [Sendbird Chat API](https://sendbird.com/)
  29. [Stream Chat API](https://getstream.io/chat/)
  30. [Pusher WebSockets](https://pusher.com/)
  31. [LivePerson Conversational Cloud](https://www.liveperson.com/)
  32. [Gainsight Customer Success](https://www.gainsight.com/)
  33. [Salesforce Service Cloud](https://www.salesforce.com/products/service-cloud/overview/)
  34. [Dynamics 365 Customer Service](https://dynamics.microsoft.com/en-us/customer-service/)
  35. [Zoho Desk](https://www.zoho.com/desk/)
  36. [Crisp Shared Inbox](https://crisp.chat/)
  37. [Olark Live Chat](https://www.olark.com/)
  38. [Tawk.to Free Live Chat](https://www.tawk.to/)
  39. [LiveChat Software](https://www.livechat.com/)
  40. [Chatbase Custom ChatGPT](https://www.chatbase.co/)
  41. [Botpress Bot Building](https://botpress.com/)
  42. [Voiceflow Conversational AI](https://www.voiceflow.com/)
  43. [Dialogflow by Google](https://cloud.google.com/dialogflow)
  44. [Amazon Lex](https://aws.amazon.com/lex/)
  45. [IBM Watson Assistant](https://www.ibm.com/products/watson-assistant)
  46. [Rasa Open Source](https://rasa.com/)
  47. [Apple Messages for Business](https://register.apple.com/messages)
  48. [WhatsApp Business API](https://business.whatsapp.com/products/business-platform)
  49. [Instagram Messenger API](https://developers.facebook.com/docs/messenger-platform/instagram)
  50. [Reddit r/smallbusiness Discussions on Chat Tools](https://www.reddit.com/r/smallbusiness/)
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
