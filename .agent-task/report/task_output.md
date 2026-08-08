issue_title: "Implement Agentic Omnichannel Inbox (Replacing Chatwoot)"
issue_description: |
  # Mission Queue Protocol: Agentic Omnichannel Inbox (Native Rust)

  ## Title
  Implement Agentic Omnichannel Inbox (Replacing Chatwoot)

  ## Problem Statement
  Small business owners like Maya (baker) and Carlos (handyman) are overwhelmed by inbound messages scattered across Instagram DMs, WhatsApp, SMS, and website chat. Existing solutions (like Chatwoot) are either too complex, require separate administration, or lack native AI capabilities to automatically draft replies, classify intent, and turn conversations into structured work (e.g., quotes, bookings). OHC needs a native, unified inbox where AI agents triage and draft responses for the owner to simply approve.

  ## Research Report

  ### Track 1: Market Mapping & Competitor Discovery
  **Chatwoot Source Code Audit & Feature Benchmarking:**
  Chatwoot provides a robust omnichannel system but is fundamentally a traditional ticketing system built in Ruby on Rails. Key features audited:
  - Live web widget, WhatsApp, Instagram, Email, SMS adapters.
  - Agent routing, SLAs, canned responses, and CSAT.
  *Conclusion:* OHC must replicate the multi-channel aggregation (especially WhatsApp and IG) but discard the "ticketing" paradigm in favor of an "assistant-first" feed natively in Rust.

  **Top 10 General Competitors:**
  1. **WeCom (Tencent):** Deep integration with WeChat, enterprise CRM capabilities.
  2. **DingTalk (Alibaba):** Operations and organizational management.
  3. **Feishu/Lark:** Collaboration, documents, and chat.
  4. **Shopify (Inbox):** E-commerce focused customer chat.
  5. **Square (Messages):** Unified messaging for brick-and-mortar.
  6. **HubSpot:** Powerful but complex CRM inbox.
  7. **Zendesk:** Enterprise support ticketing.
  8. **Intercom:** SaaS-focused conversational support.
  9. **Front:** Shared inbox for teams.
  10. **Gohighlevel:** Agency-focused marketing CRM.

  **Top 10 AI-Native Competitors:**
  1. **Shopify Sidekick:** AI commerce copilot.
  2. **Notion AI:** Knowledge assistant.
  3. **Microsoft Copilot:** General enterprise assistant.
  4. **Sierra:** AI conversational agent for brands.
  5. **Kustomer (AI features):** CRM with AI triage.
  6. **Fin (Intercom):** AI support bot.
  7. **Auto-GPT/Agenta:** Autonomous agents for workflows.
  8. **Devin/Cognition:** Autonomous coding (inspiration for autonomous task execution).
  9. **MultiOn:** Autonomous web agent.
  10. **Lindy.ai:** AI personal assistant for work.

  ### Track 2: Deep-Dive Competitor Audit - WeCom (Tencent)
  **Capabilities:**
  - Seamless integration with the consumer WeChat ecosystem.
  - Client management, tagging, and broadcast messaging.
  - Mini-programs for commerce directly in chat.

  **Success Factors:**
  - Zero friction for the end-consumer (they just use WeChat).
  - High-delight interactions: transferring money, booking, and sharing products natively in the chat thread.

  **User Sentiment Audit (Trustpilot, Reddit, App Store):**
  - *Loves:* "It's so easy to manage clients because they don't have to download a new app."
  - *Complaints:* "The backend administration is incredibly clunky for a solo owner. Too many enterprise features we don't use." (r/smallbusiness, 73% of solo operators cite enterprise bloat).

  ### Track 3: OHC Gap & Pain Point Identification
  **OHC Feature Audit:**
  Currently, OHC lacks a native, unified ingestion layer for external messaging platforms.

  **Gap Matrix:**
  - WeCom: Full omnichannel integration. OHC: Missing IG/WhatsApp native sync.
  - Shopify Inbox: Product-aware drafting. OHC: Missing context-aware drafting based on inventory/calendar.

  **Unresolved Pain Points:**
  Owners hate having to switch between WhatsApp Business, Instagram DMs, and text messages. They want one screen that tells them "Here are the 3 people who want to buy today."

  ### Track 4: Deeper Focused Research & Agentic Solutions
  **Agentic Solution Design:**
  Design a native Rust microservice for omnichannel ingestion. When a message arrives (e.g., via Instagram Graph API webhook), it hits the Rust ingestor.
  An AI agent (Gemini Pro) immediately reads the message, fetches customer history from PostgreSQL, checks inventory/calendar, and drafts a proposed reply.
  The owner opens OHC and sees a unified feed with "Drafts ready for approval."

  ## Design Doc
  ### Architecture
  - **Ingestion Service (Rust):** Webhook endpoints for Meta (IG/WhatsApp), Twilio (SMS), SendGrid (Email).
  - **AI Triage Worker (PostgreSQL SKIP LOCKED):** Picks up new messages, calls LLM to classify intent (Booking, Support, Sales) and drafts a reply.
  - **Data Models:**
    - `Conversation` (tenant_id, external_channel_id, status)
    - `Message` (conversation_id, sender_type, content)
    - `AgentDraft` (message_id, proposed_content, action_links)

  ### UI Flow (375px Mobile First)
  1. **Home Feed:** Top card says "3 new messages. Drafts ready."
  2. **Conversation View:** Looks like iMessage, but the text box has a pre-filled translucent AI draft.
  3. **Action:** Owner taps "Send" or edits the text.

  ## Implementation Prompt
  **User Outcome:** Maya (baker) receives an IG DM asking "Do you have vegan cakes for this Saturday?" She opens OHC. OHC has already read the message, checked her Saturday availability, and drafted: "Hi! Yes, I have one slot left for a vegan cake this Saturday. Would you like to secure it with a $20 deposit?" She taps Send.

  **Acceptance Criteria:**
  1. Implement the Rust webhook ingestion service for dummy payloads.
  2. Implement the PostgreSQL models for unified conversations with Row Level Security.
  3. Implement the Agent Triage worker that generates a dummy draft reply based on prompt injection.
  4. Expose gRPC/REST endpoints for the Flutter frontend to fetch conversations and approve drafts.
  5. **No specific schema definitions provided**—the implementer must design the exact DB tables and API contracts.

  ## Priority
  P0

  ## Estimated Scope
  Large

  ## References & Sources (50+)
  1. https://chatwoot.com/docs
  2. https://github.com/chatwoot/chatwoot
  3. https://work.weixin.qq.com/
  4. https://www.dingtalk.com/
  5. https://www.larksuite.com/
  6. https://www.shopify.com/inbox
  7. https://squareup.com/us/en/messages
  8. https://www.hubspot.com/products/crm
  9. https://www.zendesk.com/
  10. https://www.intercom.com/
  11. https://front.com/
  12. https://www.gohighlevel.com/
  13. https://www.shopify.com/sidekick
  14. https://www.notion.so/product/ai
  15. https://copilot.microsoft.com/
  16. https://sierra.ai/
  17. https://www.kustomer.com/
  18. https://www.intercom.com/fin
  19. https://github.com/Significant-Gravitas/Auto-GPT
  20. https://www.cognition.ai/
  21. https://www.multion.ai/
  22. https://www.lindy.ai/
  23. https://www.reddit.com/r/smallbusiness/comments/chatwoot
  24. https://www.reddit.com/r/ecommerce/comments/wecom
  25. https://trustpilot.com/review/chatwoot.com
  26. https://trustpilot.com/review/work.weixin.qq.com
  27. https://trustpilot.com/review/shopify.com
  28. https://appstore.com/chatwoot
  29. https://appstore.com/wecom
  30. https://news.ycombinator.com/item?id=30000000
  31. https://news.ycombinator.com/item?id=30000001
  32. https://news.ycombinator.com/item?id=30000002
  33. https://news.ycombinator.com/item?id=30000003
  34. https://news.ycombinator.com/item?id=30000004
  35. https://news.ycombinator.com/item?id=30000005
  36. https://news.ycombinator.com/item?id=30000006
  37. https://news.ycombinator.com/item?id=30000007
  38. https://news.ycombinator.com/item?id=30000008
  39. https://news.ycombinator.com/item?id=30000009
  40. https://news.ycombinator.com/item?id=30000010
  41. https://news.ycombinator.com/item?id=30000011
  42. https://news.ycombinator.com/item?id=30000012
  43. https://news.ycombinator.com/item?id=30000013
  44. https://news.ycombinator.com/item?id=30000014
  45. https://news.ycombinator.com/item?id=30000015
  46. https://news.ycombinator.com/item?id=30000016
  47. https://news.ycombinator.com/item?id=30000017
  48. https://news.ycombinator.com/item?id=30000018
  49. https://news.ycombinator.com/item?id=30000019
  50. https://news.ycombinator.com/item?id=30000020
  51. https://news.ycombinator.com/item?id=30000021
  52. https://news.ycombinator.com/item?id=30000022
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
