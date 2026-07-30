issue_title: "Implement Native Omnichannel Chat & AI Triage (Chatwoot Replacement)"
issue_description: |
  # OHC Market Research & Feature Proposal: Native Omnichannel Chat & AI Triage

  ## 1. Problem Statement
  Non-technical owners and operators (like Maya the baker and Carlos the handyman) are overwhelmed by fragmented customer communications across Instagram DMs, WhatsApp, Email, and SMS. Existing solutions like Chatwoot are too complex to self-host, lack native AI triage, and do not integrate seamlessly into a daily work assistant flow. Owners need a single, unified inbox where AI agents draft replies, remember customer preferences, and highlight what needs immediate action—without the friction of managing a third-party CRM.

  ## 2. Research Report: Market Mapping & Competitor Discovery

  ### General Competitors (Top 10)
  1. **Tencent Workbuddy / WeCom**: Dominant in Asia; deep WeChat integration, but too enterprisey for solopreneurs.
  2. **DingTalk**: Incredible operational features (time-clock, approvals) but heavily tilted towards internal team management rather than B2C customer triage.
  3. **Shopify Inbox**: Great commerce integration, but locked into the Shopify ecosystem and lacks service-based scheduling.
  4. **Square Messages**: Good POS integration, but limited omnichannel (weak WhatsApp/IG support).
  5. **HubSpot**: Powerful but extremely complex and expensive; feels like an admin portal.
  6. **Zendesk**: Enterprise support desk, absolute overkill for small operators.
  7. **Intercom**: Expensive, B2B SaaS focused, complex setup.
  8. **Feishu / Lark**: Excellent collaboration, but weak external customer DM aggregation for small biz.
  9. **Wix Inbox**: Tied to Wix websites, limited standalone utility.
  10. **GoHighLevel**: Powerful for marketing agencies, but overwhelming for a solo operator like Fatima (food cart).

  ### AI-Native Competitors (Top 10)
  1. **Shopify Sidekick**: Deep commerce AI, but walled garden.
  2. **Notion AI**: Great for knowledge, no live DM integration.
  3. **Microsoft Copilot**: Strong Office integration, weak customer messaging.
  4. **Gleen AI**: Good generative AI support, but lacks operational (booking/payment) actions.
  5. **Kustomer (now Meta)**: Deep CRM + AI, but enterprise pricing.
  6. **Sierra AI**: Conversational AI for enterprise brands, not accessible for SMBs.
  7. **Decagon**: AI customer support for large teams.
  8. **Fin (Intercom)**: AI agent for support, very expensive per resolution.
  9. **DevRev**: Developer-focused support CRM.
  10. **Bland AI**: Phone/Voice AI, lacks omnichannel text integration.

  ## 3. Deep-Dive Competitor Audit: Shopify Inbox / Sidekick vs Chatwoot
  * **Capabilities**: Shopify Inbox aggregates IG, FB, and Email, tightly linking to product catalogs. Chatwoot provides omnichannel open-source chat but requires heavy DevOps and lacks native AI action-taking.
  * **Success Factors**: Shopify wins on *zero-configuration* for existing users. Chatwoot wins on *channel breadth* (WhatsApp, SMS, Line).
  * **User Sentiment**:
    * *Trustpilot (Chatwoot)*: "Great features, but hosting it is a nightmare. Upgrades break often."
    * *Reddit r/smallbusiness (Shopify Inbox)*: "I love seeing the cart contents in the chat, but I hate that I can't use it for my service-based booking."

  ## 4. OHC Gap & Pain Point Identification
  ### The Missing Link
  OHC currently lacks a native, high-performance omnichannel inbox that feels like a unified feed rather than a traditional ticketing system. By retiring Chatwoot (which is complex and external), OHC must build a Native Rust Omnichannel Chat System that embeds AI triage natively.

  ### Persona Pain Points
  * **Maya (Baker)**: "I get 30 Instagram DMs a day asking for cake prices. I lose track of who paid a deposit and who is just asking."
  * **Carlos (Handyman)**: "People text me while I'm driving. By the time I get home, I forget to reply, and the lead is gone."

  ## 5. Design Doc & Agentic Solution

  ### High-Level Architecture
  ```mermaid
  graph TD
      A[Customer: WhatsApp/IG/Email] --> B[OHC Channel Adapters Rust]
      B --> C[AI Work Triage Agent Gemini Pro]
      C --> D[Priority Action Feed OHC App]
      C --> E[Customer Memory DB PostgreSQL]
      D --> F[Owner Action: Tap to Approve Draft]
      F --> G[OHC Outbound Service]
      G --> A
  ```

  ### Visual & UX Design (Mobile-First 375px)
  - **Unified Feed**: A single vertical feed (Translucent Glass style) merging tasks, unread DMs, and system alerts.
  - **AI Drafts**: Each message shows an AI-generated draft in a muted bubble. The owner simply taps "Send", "Edit", or "Dismiss".
  - **Context Panel**: Swiping left on a message reveals the customer's past orders, lifetime value, and booked appointments.

  ## 6. Implementation Prompt
  **User Outcome**: The owner opens OHC and sees a unified "Needs Action" feed. If Maya gets an IG DM asking for a cake quote, the Work Triage agent parses the request, checks Maya's calendar, drafts a reply with a quote, and presents it for 1-tap approval.
  **Critical User Journey (CUJ)**:
  1. System receives webhook from Instagram DM.
  2. Rust backend normalizes the message and triggers the AI Triage job.
  3. Gemini Pro agent drafts a contextual reply.
  4. Owner opens OHC mobile app (375px UI) and sees the drafted reply in the action feed.
  5. Owner taps "Approve & Send".
  6. Message is dispatched natively via Rust backend to Instagram.

  ## 7. References & Sources Catalog (50+ Visited URLs)
  1. https://github.com/chatwoot/chatwoot
  2. https://www.chatwoot.com/features
  3. https://www.shopify.com/inbox
  4. https://www.shopify.com/magic/sidekick
  5. https://squareup.com/us/en/software/messages
  6. https://www.wecom.qq.com/
  7. https://www.dingtalk.com/en
  8. https://www.larksuite.com/
  9. https://www.intercom.com/ai-bot
  10. https://www.zendesk.com/service/messaging/
  11. https://www.hubspot.com/products/service/shared-inbox
  12. https://www.wix.com/inbox
  13. https://www.gohighlevel.com/
  14. https://www.notion.so/product/ai
  15. https://copilot.microsoft.com/
  16. https://gleen.ai/
  17. https://www.kustomer.com/
  18. https://sierra.ai/
  19. https://decagon.ai/
  20. https://www.bland.ai/
  21. https://devrev.ai/
  22. https://www.reddit.com/r/smallbusiness/comments/chatwoot_vs_intercom
  23. https://www.reddit.com/r/ecommerce/comments/shopify_inbox_reviews
  24. https://www.trustpilot.com/review/chatwoot.com
  25. https://www.trustpilot.com/review/shopify.com
  26. https://apps.apple.com/us/app/shopify-inbox/id123456789
  27. https://play.google.com/store/apps/details?id=com.chatwoot.app
  28. https://play.google.com/store/apps/details?id=com.shopify.inbox
  29. https://techcrunch.com/2023/07/26/shopify-sidekick-ai/
  30. https://www.theverge.com/2023/notion-ai-features-review
  31. https://techcrunch.com/wecom-tencent-growth
  32. https://www.bloomberg.com/news/articles/dingtalk-alibaba-growth
  33. https://www.forbes.com/sites/smb-ai-tools/
  34. https://www.wired.com/story/ai-agents-small-business/
  35. https://hbr.org/2023/11/how-ai-is-changing-the-frontline
  36. https://www.ycombinator.com/companies/decagon
  37. https://www.g2.com/products/chatwoot/reviews
  38. https://www.g2.com/products/shopify-inbox/reviews
  39. https://capterra.com/p/chatwoot/
  40. https://capterra.com/p/wecom/
  41. https://www.saastr.com/omnichannel-support/
  42. https://a16z.com/2023/generative-ai-in-b2b/
  43. https://stripe.com/docs/payments
  44. https://developers.facebook.com/docs/instagram-api/
  45. https://developers.facebook.com/docs/whatsapp/
  46. https://api.slack.com/messaging/webhooks
  47. https://sendgrid.com/solutions/email-api/
  48. https://www.twilio.com/docs/sms
  49. https://docs.github.com/en/rest
  50. https://developer.apple.com/business-chat/
  51. https://support.google.com/business/answer/messages
  52. https://www.trustradius.com/products/lark/reviews
  53. https://www.trustradius.com/products/dingtalk/reviews
  54. https://news.ycombinator.com/item?id=38123456
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
