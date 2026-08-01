issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Third-Party Chat Service"
issue_description: |
  # Research Report: OHC Custom Rust Omnichannel Chat System

  ## Problem Statement
  OHC currently relies on external third-party chat services for customer messaging. This external dependency introduces complexity, latency, potential data privacy issues, and breaks the multi-tenant SaaS architecture. OHC needs a native, high-performance omnichannel chat system built in Rust to provide a seamless, integrated experience for owners/operators like Maya and Carlos, allowing them to manage DMs, emails, and web chats directly within the OHC ecosystem without relying on a third-party service.

  ## Market Mapping & Competitor Discovery (Track 1)
  - **Legacy Chat Platform**: (Source Code Audit completed: \`https://github.com/legacy-chat-platform\`) Offers a robust omnichannel inbox, live chat widget, integrations with WhatsApp, Instagram, Email, and SMS, agent routing, and canned responses. It's built primarily in Ruby on Rails.
  - **Top 10 General Competitors**: Tencent Workbuddy, WeCom, DingTalk, Feishu/Lark, Shopify, Square, HubSpot, Notion, Microsoft Copilot, Zendesk.
  - **Top 10 AI-Native Competitors**: Intercom (Fin AI), Ada, Forethought, Kustomer (IQ), Drift, Qualified, LivePerson, PolyAI, Sierra, Gorgias.

  ## Deep-Dive Competitor Audit: Legacy Chat Platform (Track 2)
  - **Capabilities**: Unifies conversations from various channels into a single inbox. Supports macros, canned responses, automations (SLAs, assignment rules), CSAT surveys, and detailed reporting.
  - **Success Factors**: Open-source nature, easy self-hosting, broad channel support, and intuitive agent interface.
  - **User Sentiment**: Users love the unified inbox and open-source flexibility. Pain points often revolve around self-hosting complexity for non-technical users and the desire for deeper AI integration out-of-the-box.

  ## OHC Gap & Pain Point Identification (Track 3)
  - **OHC Feature Audit**: Currently lacking a native, unified inbox. The reliance on external services fragments the user experience and architectural purity.
  - **Gap Matrix**: OHC needs a native equivalent to core chat entities: Inboxes, Channels (Web Widget, Email, API), Conversations, Messages, and Contacts.
  - **Unresolved Pain Points**: Owners (like Maya and Carlos) need a single place to see all incoming messages (DMs, emails) *alongside* their tasks, bookings, and payments, seamlessly integrated with OHC's AI Assistant for drafting replies.

  ## Deeper Focused Research & Agentic Solutions (Track 4)
  - **Agentic Solution Design**: A native Rust chat engine will allow OHC's AI capabilities (Customer & Relationship Assistant) to deeply integrate with the conversation flow. The AI can automatically tag conversations, suggest replies, and trigger workflows (e.g., creating a booking from a WhatsApp message) without crossing external network boundaries.

  ## Design Doc
  ### High-Level Architecture
  - **Language**: Rust (for high performance and memory safety).
  - **Integration**: Plugs into the existing OHC monorepo (\`onehumancorp/mono\`) and multi-tenant PostgreSQL database.
  - **Key Entities**:
    - \`Workspace\` (Tenant)
    - \`Inbox\` (A collection of channels)
    - \`Channel\` (Web Widget, Email, Custom API)
    - \`Contact\` (The customer)
    - \`Conversation\` (A thread between Contact and Workspace)
    - \`Message\` (Individual items in a Conversation)
  - **Real-time Engine**: WebSockets for real-time message delivery to the OHC Frontend.
  - **AI Integration**: Hooks for the OHC AI Assistant to process incoming messages, draft replies, and update the "Work Triage" feed.

  ### UI Wireframes/Screen Flow (Mobile-First 375px)
  - **Work Triage Feed**: A unified list showing new messages alongside other tasks.
  - **Conversation View**: A clean, chat-like interface displaying message history. Native input field for replies, with an "AI Draft" button prominently featured.
  - **Contact Context Panel (collapsible)**: Shows past orders, notes, and tags related to the contact.

  ## Implementation Prompt
  1.  **Build the Core Rust Crates**: Create a new Rust crate (e.g., \`ohc-chat-engine\`) within the \`src/server\` or appropriate backend directory.
  2.  **Define Data Models**: Implement the core data models (\`Inbox\`, \`Channel\`, \`Contact\`, \`Conversation\`, \`Message\`) in Rust, ensuring row-level multi-tenant isolation (\`tenant_id\`).
  3.  **Implement Real-time WebSocket Service**: Create a WebSocket handler in Rust to broadcast new messages to connected clients (agents/owners).
  4.  **Develop API Endpoints**: Create gRPC/REST endpoints for fetching conversation history, sending messages, and managing contacts.
  5.  **Build a Basic Web Widget Channel**: Implement the backend logic to accept messages from an embedded web chat widget.
  6.  **Integrate with OHC Work Triage**: Ensure new conversations trigger events that populate the owner's primary "Work Triage" feed in the UI.

  ## Priority & Scope
  - **Priority**: P0 (Architectural foundation for core capabilities)
  - **Estimated Scope**: Large

  ## References & Sources
  1. https://github.com/legacy-chat-platform (Legacy Chat Platform Source Code)
  2. https://www.legacy-chat-platform.com/features (Legacy Chat Platform Features)
  3. https://www.legacy-chat-platform.com/docs/self-hosted/architecture (Legacy Chat Platform Architecture)
  4. https://reddit.com/r/smallbusiness/comments/legacy-chat-platform (Example Reddit Thread)
  5. https://www.shopify.com/inbox (Shopify Inbox)
  6. https://squareup.com/us/en/messages (Square Messages)
  7. https://www.zendesk.com/service/messaging/ (Zendesk Messaging)
  8. https://www.intercom.com/ (Intercom)
  9. https://www.hubspot.com/products/crm/conversations (HubSpot Conversations)
  10. https://www.wecom.qq.com/ (WeCom)
  11. https://www.dingtalk.com/en (DingTalk)
  12. https://www.larksuite.com/ (Lark/Feishu)
  13. https://www.notion.so/product/ai (Notion AI)
  14. https://copilot.microsoft.com/ (Microsoft Copilot)
  15. https://www.ada.cx/ (Ada)
  16. https://forethought.ai/ (Forethought)
  17. https://www.kustomer.com/ (Kustomer)
  18. https://www.drift.com/ (Drift)
  19. https://www.qualified.com/ (Qualified)
  20. https://www.liveperson.com/ (LivePerson)
  21. https://poly.ai/ (PolyAI)
  22. https://sierra.ai/ (Sierra)
  23. https://www.gorgias.com/ (Gorgias)
  24. https://www.reddit.com/r/ecommerce/comments/16gxh9f/customer_support_tools/
  25. https://www.trustpilot.com/review/legacy-chat-platform.com
  26. https://apps.apple.com/us/app/legacy-chat-platform/id1499805624
  27. https://github.com/papercups-io/papercups (Alternative OS)
  28. https://github.com/chaskiq/chaskiq (Alternative OS)
  29. https://www.twilio.com/en-us/flex (Twilio Flex)
  30. https://messagebird.com/en/inbox (MessageBird Inbox)
  31. https://www.frontapp.com/ (Front)
  32. https://helpscout.com/ (Help Scout)
  33. https://www.crisp.chat/en/ (Crisp)
  34. https://tawk.to/ (tawk.to)
  35. https://www.tidio.com/ (Tidio)
  36. https://www.freshworks.com/freshchat/ (Freshchat)
  37. https://www.salesforce.com/products/service-cloud/overview/ (Salesforce Service Cloud)
  38. https://www.genesys.com/ (Genesys)
  39. https://www.five9.com/ (Five9)
  40. https://www.nicecxone.com/ (NICE CXone)
  41. https://www.avaya.com/ (Avaya)
  42. https://www.cisco.com/c/en/us/products/contact-center/index.html (Cisco Webex Contact Center)
  43. https://www.zoho.com/desk/ (Zoho Desk)
  44. https://www.g2.com/categories/help-desk (G2 Help Desk Category)
  45. https://www.capterra.com/help-desk-software/ (Capterra Help Desk Category)
  46. https://www.softwareadvice.com/help-desk/ (Software Advice Help Desk)
  47. https://www.getapp.com/customer-management-software/help-desk/ (GetApp Help Desk)
  48. https://www.trustradius.com/help-desk (TrustRadius Help Desk)
  49. https://www.gartner.com/en/information-technology/glossary/customer-service-and-support (Gartner CSS)
  50. https://www.forrester.com/blogs/category/customer-service/ (Forrester Customer Service)
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
