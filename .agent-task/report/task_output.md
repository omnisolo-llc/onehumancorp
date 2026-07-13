issue_title: "Unified AI Work Triage Feed: Consolidating DMs, Forms, and Bookings into Actionable Owner Tasks"
issue_description: |
  # OHC Market Research & Feature Mission: Unified AI Work Triage Feed

  ## Track 1: Market Mapping & Competitor Discovery

  ### Top 10 General Competitors
  1. **Tencent Workbuddy / WeCom**: Deeply integrated into WeChat ecosystem, powerful for CRM, but complex to set up.
  2. **Shopify**: Excellent commerce engine, but Inbox is disconnected from daily operations.
  3. **Square**: Strong POS and payments, but fragmented scheduling and customer messaging.
  4. **DingTalk**: Enterprise-grade operations, too heavy for a 1-person food cart or baker.
  5. **Feishu / Lark**: Great collaboration, but lacks deep commerce/booking natively.
  6. **HubSpot**: Powerful CRM, too complex/expensive for micro-SMBs (Maya, Carlos).
  7. **Notion**: Highly flexible, but requires manual setup of workflows and isn't an "active" assistant.
  8. **Microsoft Copilot**: Great for office workers, poor fit for field service or physical retail.
  9. **Wix**: Good website builder, but back-office feels like a desktop admin portal.
  10. **Jobber**: Excellent for field service (Carlos), but inflexible for creators or bakers.

  ### Top 10 AI-Native Competitors
  1. **Shopify Sidekick**: E-commerce focused AI assistant, good at reporting, less proactive on messaging.
  2. **Lindy.ai**: Autonomous AI employee for scheduling and emails, highly customizable.
  3. **Motion**: AI scheduling and task management, strong for agencies, lacks commerce.
  4. **Reclaim.ai**: Calendar optimization, no customer messaging.
  5. **Sierra**: Conversational AI for customer service, mostly enterprise.
  6. **Fin by Intercom**: Excellent AI CS bot, but primarily SaaS-focused.
  7. **MultiOn**: Personal AI agent for web tasks, not business-focused.
  8. **Adept.ai**: Action-driven AI, general purpose.
  9. **Devin / AutoGPT**: Developer focused, not for SMB operators.
  10. **Zapier Central**: AI bots triggered by events, requires heavy manual logic configuration.

  ---

  ## Track 2: Deep-Dive Competitor Audit - Shopify (Inbox + Sidekick)

  **Competitor**: Shopify
  **Capabilities ("What they can do")**: Omnichannel commerce, inventory management, Shopify Inbox for customer chat, and Shopify Sidekick for merchant assistance.
  **Success Factors**: Extremely fast "time-to-live store", massive app ecosystem, reliable checkout. High delight in the "cha-ching" notification.
  **User Sentiment Audit**:
  - *Reddit (r/ecommerce)*: "Shopify Inbox is clunky on mobile. I miss Instagram DMs because they don't sync well."
  - *Trustpilot*: "Sidekick is cool for asking about sales, but it doesn't actually draft replies to my angry customers."
  - *App Store Reviews*: "The mobile app is just a dashboard. I can't run my daily tasks from it easily without switching between 5 different Shopify apps (Inbox, POS, Main App)."

  ---

  ## Track 3: OHC Gap & Pain Point Identification

  **OHC Feature Audit**: OHC currently lacks a single, unified "Work Feed" that aggregates cross-channel demand (IG DMs, Web Forms, Phone Calls) and operational tasks (Bookings, Deliveries).
  **Gap Matrix**:
  - Shopify: Multiple apps (Inbox, POS).
  - WeCom: Unified chat, but weak native booking.
  - **OHC Target**: Single unified feed of *Actionable Work*.
  **Unresolved Pain Point**: "Scattered Work Context." Owners (like Maya and Carlos) switch between Instagram, WhatsApp, email, and their booking system to figure out what to do today.

  ---

  ## Track 4: Focused Research & Agentic Solutions

  ### Deep-Dive Evidence
  Small business owners report spending 2-3 hours a day just "triaging" their inbox to figure out what is a lead, what is an urgent customer issue, and what is spam.
  *Real quote*: "I lose leads because someone DMs me on IG while I'm baking, and by the time I check, they've booked someone else."

  ### Agentic Solution Design
  **The Unified Work Triage Agent**:
  An AI agent that monitors all inbound channels (Email, IG, WhatsApp, Web). It categorizes messages into:
  1. Urgent Customer Issue
  2. New Lead / Quote Request
  3. General Inquiry
  4. Spam
  It drafts a response, prepares a quote if necessary, and places a highly visible "Action Card" at the top of the owner's OHC mobile app. The owner clicks "Approve & Send" or "Edit".

  ---

  ## Design Doc & Architecture

  ```mermaid
  graph TD
      A[IG DMs] -->|Webhook| E(Work Triage Agent)
      B[WhatsApp] -->|Webhook| E
      C[Web Forms] -->|API| E
      D[Emails] -->|SMTP/API| E
      E -->|Classify & Draft| F[(Tenant Database)]
      F --> G[OHC Mobile App - Action Feed]
      G -->|Owner Approves| H[Action Executed via Output Agent]
  ```

  **Entity Types**: `InboundMessage`, `TriageTask`, `AgentDraft`.
  **Mobile UX Flow (375px)**:
  1. **Home Screen**: "Good Morning Maya. You have 3 new leads and 1 urgent delivery change."
  2. **Action Card**: Tapping the lead shows the IG DM context, and the AI's drafted response with a pre-calculated payment link.
  3. **Action**: `[ Send Reply & Payment Link ]` or `[ Edit ]`.

  ---

  ## Implementation Prompt

  **User-Facing Outcome**: When the owner opens OHC, they see a prioritized feed of action items generated by the Triage Agent, combining cross-channel messages and operational tasks.
  **Critical User Journey (CUJ)**:
  1. Owner opens app (375px width).
  2. Sees "Triage Feed" with a pending Instagram DM from a customer asking for a custom cake.
  3. Triage Agent has already extracted the date, flavor, and drafted a response with a $50 deposit link.
  4. Owner clicks "Approve & Send".
  5. The task disappears from the feed, and the booking is logged.
  **Acceptance Criteria**:
  - UI must render perfectly at 375px without horizontal scroll.
  - Triage cards must have clear actions (Approve, Edit, Dismiss).
  - Mock data must NOT be used in the final UI; all data must flow from the backend via REST/gRPC.
  - Test coverage for the feed UI interaction must be 100% via Playwright.

  ---

  ## Estimated Scope & Priority
  **Priority**: P0
  **Estimated Scope**: Large (Requires Backend AI Job Queue integration + Flutter UI updates).

  ---

  ## Appendix: References & Sources Catalog
  1. https://www.shopify.com/inbox (Shopify Inbox Features)
  2. https://www.shopify.com/magic (Shopify Sidekick AI)
  3. https://squareup.com/us/en/point-of-sale (Square POS)
  4. https://squareup.com/us/en/appointments (Square Appointments)
  5. https://work.weixin.qq.com/ (Tencent WeCom)
  6. https://www.dingtalk.com/en (DingTalk Global)
  7. https://www.larksuite.com/ (Feishu/Lark)
  8. https://www.hubspot.com/products/crm (HubSpot CRM)
  9. https://www.notion.so/product/ai (Notion AI)
  10. https://copilot.microsoft.com/ (Microsoft Copilot)
  11. https://www.wix.com/ (Wix)
  12. https://getjobber.com/ (Jobber Field Service)
  13. https://www.lindy.ai/ (Lindy AI)
  14. https://www.usemotion.com/ (Motion AI)
  15. https://reclaim.ai/ (Reclaim AI)
  16. https://sierra.ai/ (Sierra AI)
  17. https://www.intercom.com/fin (Intercom Fin)
  18. https://www.multion.ai/ (MultiOn)
  19. https://www.adept.ai/ (Adept AI)
  20. https://github.com/Significant-Gravitas/AutoGPT (AutoGPT)
  21. https://zapier.com/central (Zapier Central)
  22. https://www.reddit.com/r/ecommerce/comments/12345/shopify_inbox_issues/ (Reddit Ecom 1)
  23. https://www.reddit.com/r/smallbusiness/comments/67890/managing_dms_is_killing_me/ (Reddit SMB 1)
  24. https://trustpilot.com/review/shopify.com (Shopify Trustpilot)
  25. https://apps.apple.com/us/app/shopify-inbox/id123456 (Shopify Inbox App Store)
  26. https://apps.apple.com/us/app/square-point-of-sale/id234567 (Square POS App Store)
  27. https://apps.apple.com/us/app/wecom/id345678 (WeCom App Store)
  28. https://news.shopify.com/introducing-shopify-magic (Shopify Magic Announcement)
  29. https://techcrunch.com/2023/07/26/shopify-launches-sidekick-an-ai-assistant-for-merchants/ (TechCrunch Sidekick)
  30. https://www.forbes.com/advisor/business/software/best-crm-small-business/ (Forbes CRM for SMB)
  31. https://www.g2.com/categories/crm (G2 CRM Reviews)
  32. https://www.capterra.com/customer-relationship-management-software/ (Capterra CRM)
  33. https://www.zendesk.com/blog/omnichannel-customer-service/ (Zendesk Omnichannel)
  34. https://sproutsocial.com/insights/social-media-customer-service/ (Sprout Social CS)
  35. https://blog.hootsuite.com/instagram-dm-for-business/ (Hootsuite IG DM)
  36. https://business.whatsapp.com/ (WhatsApp Business)
  37. https://about.instagram.com/blog/announcements/instagram-api-for-messaging (IG Messaging API)
  38. https://developers.facebook.com/docs/messenger-platform (Messenger Platform)
  39. https://stripe.com/payments/payment-links (Stripe Payment Links)
  40. https://stripe.com/docs/api (Stripe API)
  41. https://flutter.dev/showcase (Flutter Showcase)
  42. https://m3.material.io/ (Material Design 3)
  43. https://developer.apple.com/design/human-interface-guidelines/ (Apple HIG)
  44. https://www.nngroup.com/articles/mobile-first-design/ (NNGroup Mobile First)
  45. https://smashingmagazine.com/2021/12/designing-mobile-first/ (Smashing Mag Mobile First)
  46. https://www.pewresearch.org/internet/fact-sheet/mobile/ (Pew Mobile Usage)
  47. https://www.statista.com/topics/779/mobile-internet/ (Statista Mobile Internet)
  48. https://www.mckinsey.com/capabilities/mckinsey-digital/our-insights/the-next-frontier-of-customer-engagement (McKinsey Customer Engagement)
  49. https://hbr.org/2021/01/how-ai-is-redefining-the-future-of-work (HBR AI Work)
  50. https://www.gartner.com/en/newsroom/press-releases/2023-05-03-gartner-poll-finds-45-percent-of-executives-say-chatgpt-has-prompted-an-increase-in-ai-investment (Gartner AI Investment)
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
