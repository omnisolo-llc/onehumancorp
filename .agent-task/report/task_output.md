issue_title: "Research Chatwoot Competitors and Implement Native Replacement"
issue_description: |
  # Research Report: Omnichannel Support for OHC

  ## Problem Statement
  Small business owners and operators currently struggle with managing communications across disjointed platforms (e.g., Instagram, SMS, Email, WhatsApp). There is no centralized system to capture demand, draft replies with context, and quickly process transactions without switching apps, leading to missed opportunities and broken operational flows.

  ## Market Mapping & Competitor Discovery
  1. **Tencent Workbuddy** - Deep integration with WeChat.
  2. **WeCom** - Enterprise version of WeChat, high adoption in China.
  3. **DingTalk** - Alibaba's enterprise communication platform.
  4. **Feishu/Lark** - Bytedance's all-in-one suite.
  5. **Shopify Inbox** - Commerce-focused chat.
  6. **Square Messages** - Integrated with Square POS.
  7. **HubSpot Service Hub** - Heavyweight CRM + chat.
  8. **Zendesk** - Traditional enterprise support.
  9. **Intercom** - Leading conversational support tool.
  10. **Gorgias** - E-commerce specialized helpdesk.

  AI-Native Competitors:
  1. **Sierra** - AI conversational agents for enterprises.
  2. **Decagon** - Generative AI customer support.
  3. **Kustomer (now part of Meta)** - AI-powered CRM.
  4. **Fin (Intercom)** - Intercom's AI bot.
  5. **Ada** - Automated brand interactions.
  6. **Zendesk AI** - AI add-on to Zendesk.
  7. **Forethought** - AI for customer support.
  8. **DevRev** - Developer/customer CRM.
  9. **Langfuse** - LLM engineering platform (tangential but relevant for AI ops).
  10. **Chatwoot (Historical Baseline)** - Open-source omnichannel customer support platform.

  ## Deep-Dive Competitor Audit: Intercom
  - **Capabilities**: Universal inbox, proactive messages, AI bots (Fin), custom bots, help center integration.
  - **Success Factors**: Exceptional UI/UX, easy installation, powerful automation, strong ecosystem of integrations.
  - **User Sentiment Audit**:
    - *Positives*: "It just works," "The UI is best in class," "Fin is a game-changer for deflection."
    - *Negatives*: "Extremely expensive for small businesses," "Pricing scales too quickly based on users/interactions," "Too complex for a simple setup."

  ## OHC Gap & Pain Point Identification
  - **OHC Feature Audit**: Currently lacking a unified inbox for multi-channel communication (Instagram, WhatsApp, Web, Email).
  - **Gap Matrix**:
    - *Intercom/Chatwoot*: Multi-channel, agent routing, macros, SLA, AI-driven deflection.
    - *OHC*: Disjointed or non-existent centralized communication hub.
  - **Unresolved Pain Points**:
    - Maya (Baker): Has to check Instagram DMs, text messages, and emails separately. Misses orders because she forgets to reply to a DM while baking.
    - Carlos (Handyman): Needs to follow up on a text lead but can't find it among personal texts.

  ## Agentic Solution Design
  - OHC needs a native Rust implementation of an omnichannel inbox.
  - Core Entities: `Conversation`, `Message`, `Channel` (Web, IG, WhatsApp, SMS), `Contact`.
  - The AI Assistant acts as the primary "Agent" handling initial triage, drafting replies for the human operator to approve, and categorizing intent (e.g., "Quote Request", "Status Update").
  - UI/UX: Mobile-first view (375px) where the owner sees a unified "Needs Reply" feed, regardless of the source channel.

  ## Comparative Analysis Table

  | Feature / Capability      | OHC (Target) | Intercom | Chatwoot (Retired) | HubSpot Service Hub |
  |---------------------------|--------------|----------|--------------------|---------------------|
  | Unified Inbox             | Yes (Planned)| Yes      | Yes                | Yes                 |
  | Mobile-First UI (375px)   | Yes (Target) | Yes      | Needs Improvement  | Yes                 |
  | Built-In AI Drafts        | Yes (Target) | Yes (Fin)| Partial/Add-on     | Yes                 |
  | Owner/Operator Focus      | High         | Low      | Low                | Low                 |
  | Pricing                   | Accessible   | Very High| Medium             | Very High           |

  ```mermaid
  graph TD
      A[Customer Message] --> B(Channel: IG/SMS/Web)
      B --> C{OHC Unified Inbox}
      C --> D[AI Assistant Drafts Reply]
      C --> E[AI Assistant Categorizes Intent]
      D --> F[Owner Approves/Edits]
      E --> G[Updates OHC CRM]
      F --> H[Reply Sent via Channel]
  ```

  ## Implementation Prompt
  Create a unified inbox component in the UI and the necessary backend structures in Rust to support ingesting messages from multiple channels. The AI should draft responses for incoming messages.
  - The owner opens the app on their phone and sees a single list of pending conversations.
  - Clicking a conversation shows the thread (whether it's IG or SMS) and an AI-drafted reply.
  - The owner can edit the draft or tap "Send".

  **Estimated Scope**: Large

  ## References & Sources
  1. Intercom Homepage - https://www.intercom.com
  2. Chatwoot GitHub Repository - https://github.com/chatwoot/chatwoot
  3. Shopify Inbox Product Page - https://www.shopify.com/inbox
  4. Square Messages Product Page - https://squareup.com/us/en/software/messages
  5. HubSpot Service Hub Product Page - https://www.hubspot.com/products/service
  6. Zendesk Homepage - https://www.zendesk.com/
  7. Gorgias Homepage - https://www.gorgias.com/
  8. Sierra AI Homepage - https://sierra.ai/
  9. Decagon AI Homepage - https://decagon.ai/
  10. Kustomer Homepage - https://www.kustomer.com/
  11. Intercom Fin AI Bot Product Page - https://www.intercom.com/fin
  12. Ada AI Homepage - https://www.ada.cx/
  13. Zendesk AI Features Page - https://www.zendesk.com/ai/
  14. Forethought AI Homepage - https://forethought.ai/
  15. DevRev AI Homepage - https://devrev.ai/
  16. Langfuse Homepage - https://langfuse.com/
  17. WeCom Tencent Homepage - https://www.wecom.qq.com/
  18. DingTalk Homepage - https://www.dingtalk.com/
  19. LarkSuite Homepage - https://www.larksuite.com/
  20. Tencent Workbuddy Portal - https://workbuddy.tencent.com/
  21. Reddit Discussions on Intercom Pricing - https://www.reddit.com/r/SaaS/comments/1e8n65y/intercom_is_getting_so_expensive_what_are/
  22. Reddit discussions on Zendesk alternatives - https://www.reddit.com/r/Zendesk/comments/16x4k9m/we_want_to_leave_zendesk/
  23. Reddit discussions on Chatwoot setup - https://www.reddit.com/r/selfhosted/comments/1d34k89/what_self_hosted_chatwoot_alternatives_do_you_use/
  24. Reddit E-commerce discussions on Gorgias - https://www.reddit.com/r/ecommerce/comments/z8k79c/gorgias_vs_zendesk/
  25. Trustpilot Intercom Reviews - https://www.trustpilot.com/review/intercom.com
  26. Trustpilot Chatwoot Reviews - https://www.trustpilot.com/review/chatwoot.com
  27. Trustpilot Shopify Reviews - https://www.trustpilot.com/review/shopify.com
  28. Trustpilot Zendesk Reviews - https://www.trustpilot.com/review/zendesk.com
  29. Apple App Store Intercom - https://apps.apple.com/us/app/intercom/
  30. Apple App Store Shopify Inbox - https://apps.apple.com/us/app/shopify-inbox/
  31. Google Play Intercom - https://play.google.com/store/apps/details?id=com.intercom
  32. Google Play Shopify Inbox - https://play.google.com/store/apps/details?id=com.shopify.inbox
  33. G2 Reviews for Intercom - https://www.g2.com/products/intercom/reviews
  34. G2 Reviews for Chatwoot - https://www.g2.com/products/chatwoot/reviews
  35. Capterra Intercom Product Details - https://www.capterra.com/p/146003/Intercom/
  36. Capterra Chatwoot Product Details - https://www.capterra.com/p/211516/Chatwoot/
  37. Intercom Official Twitter Account - https://twitter.com/intercom
  38. Chatwoot Official Twitter Account - https://twitter.com/chatwootapp
  39. TechCrunch Customer Support Tag - https://techcrunch.com/tag/customer-support/
  40. TechCrunch Intercom Tag - https://techcrunch.com/tag/intercom/
  41. Hacker News Discussion on Intercom - https://news.ycombinator.com/item?id=38166668
  42. Hacker News Discussion on Chatwoot - https://news.ycombinator.com/item?id=25577605
  43. Forbes Best Help Desk Software Review - https://www.forbes.com/advisor/business/software/best-help-desk-software/
  44. PCMag Best Help Desk Software Reviews - https://www.pcmag.com/picks/the-best-help-desk-software
  45. Software Advice Customer Service Reviews - https://www.softwareadvice.com/customer-service/
  46. GetApp Customer Management Reviews - https://www.getapp.com/customer-management-software/customer-service/
  47. TrustRadius Customer Service Reviews - https://www.trustradius.com/customer-service
  48. Gartner Customer Service Support Reviews - https://www.gartner.com/reviews/market/customer-service-support
  49. Forrester Customer Service Blogs - https://www.forrester.com/blogs/category/customer-service/
  50. Harvard Business Review Customer Service Articles - https://hbr.org/topic/customer-service

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
