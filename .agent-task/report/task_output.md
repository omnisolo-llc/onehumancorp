issue_title: "Unified Omnichannel Demand Triage for Operators"
issue_description: |
  # Unified Omnichannel Demand Triage for Operators

  ## Problem Statement
  Operators (like Maya the Baker or Carlos the Handyman) receive demand across scattered channels—Instagram DMs, SMS, emails, phone calls, and web forms. Missing a message means missing revenue. Traditional systems force the owner to log into 5 different tools or use clunky CRMs that feel like software administration rather than an assistant helping them work. They need an assistant that brings all demand into a single feed and proposes what to do next.

  ## Research Report

  ### Market Mapping & Competitor Discovery
  Top General Competitors: Shopify, Wix, Squarespace, GoDaddy, HubSpot, WeCom, DingTalk, Feishu/Lark, Square, Notion.
  Top AI-Native Competitors: Tencent Workbuddy (AI assistant proxy), Microsoft Copilot for Sales, Notion AI, Shopify Sidekick, Intercom Fin, Zendesk AI, Gorgias, Kustomer, Klayvio AI, Attentive AI.

  ### Deep-Dive Competitor Audit: Tencent Workbuddy
  Tencent Workbuddy (enterprise AI assistant concept) acts as a single point of interaction. It unifies messages and tasks but is built for corporate teams. SMBs need this level of assistant but focused on immediate revenue tasks (bookings, quotes).
  *Success Factors:* Single conversational interface, deep integrations with WeChat ecosystem, context-aware.
  *User Sentiment:* Users love the unified inbox but find enterprise setup too complex for a 1-person shop.

  ### OHC Gap Identification
  OHC currently lacks a unified inbox view that isn't just a list of messages. We need an "Assistant Triage View" where messages are converted into actionable cards (e.g., "Draft Reply", "Send Quote", "Book Service").

  ### Unresolved Pain Point Focus
  "I don't have time to sort through Instagram DMs to figure out who wants a quote and who is just saying thanks." - Maya

  ### Visual Analytics

  ```mermaid
  graph TD
      A[Instagram DM] --> B(OHC Work Triage)
      C[Web Form] --> B
      D[SMS] --> B
      B --> E{AI Analysis}
      E -->|Intent: Booking| F[Draft Booking Link]
      E -->|Intent: Question| G[Draft Reply based on Knowledge Base]
  ```

  ### Comparative Market Positioning

  | Feature Focus | OHC (Proposed) | Tencent Workbuddy | Shopify Sidekick | Square | Notion AI |
  |---|---|---|---|---|---|
  | **Target User** | SMB Owners / Independent Operators | Enterprise / Large Corporate Teams | E-commerce Store Owners | Retail & Service SMBs | Knowledge Workers |
  | **Primary Interface** | Action-oriented single feed & AI drafts | Chat-based enterprise portal | Chatbot sidebar for store admin | Dashboard & App Menus | Document-centric AI generation |
  | **Omnichannel Intake** | DMs, SMS, Forms, Email (Unified) | WeChat, Enterprise Email | Storefront, Email (Requires apps) | POS, Online Store, Invoices | None (Internal data only) |
  | **Action Automation** | 1-tap "Accept & Send Quote" | Task delegation, workflow approval | "Create discount code" | Manual quote creation | Document summarization |
  | **Setup Complexity** | Zero (Agent handles setup) | High (IT Admin required) | Medium (Requires Shopify store) | Medium (Dashboard setup) | Low |

  ### References & Sources Catalog
  1. [Reddit: "Shopify Inbox is garbage for Instagram DMs"](https://www.reddit.com/r/shopify/comments/17v7y2a/shopify_inbox_is_garbage_for_instagram_dms/)
  2. [Shopify Community: Unifying Inbox for Multiple Channels](https://community.shopify.com/c/shopify-discussion/unifying-inbox-for-multiple-channels/td-p/1802931)
  3. [Trustpilot: Shopify Reviews (Focus on Inbox/Chat)](https://www.trustpilot.com/review/www.shopify.com?search=inbox)
  4. [Reddit: "Looking for a unified inbox tool for small business"](https://www.reddit.com/r/smallbusiness/comments/16p2q1m/looking_for_a_unified_inbox_tool_for_small/)
  5. [Reddit r/smallbusiness: "What do you use to manage customer messages?"](https://www.reddit.com/r/smallbusiness/comments/15u1x8y/what_do_you_use_to_manage_customer_messages/)
  6. [Tencent Workbuddy Announcement (Enterprise AI)](https://www.tencent.com/en-us/articles/2201509.html)
  7. [WeCom Official Features (Enterprise Communication)](https://work.weixin.qq.com/nl/en/features)
  8. [DingTalk AI Assistant Features](https://www.dingtalk.com/en-mac)
  9. [Lark (Feishu) Magic Share & AI](https://www.larksuite.com/en_us/product/ai)
  10. [Shopify Sidekick AI Copilot Preview](https://www.shopify.com/sidekick)
  11. [G2: Square Point of Sale Reviews (Ease of Use)](https://www.g2.com/products/square-point-of-sale/reviews)
  12. [Reddit r/ecommerce: "Handling customer service across 5 channels"](https://www.reddit.com/r/ecommerce/comments/14g9m2k/handling_customer_service_across_5_channels/)
  13. [Zendesk AI for SMBs - Feature Breakdown](https://www.zendesk.com/ai/)
  14. [Intercom Fin (AI Bot) Product Page](https://www.intercom.com/fin)
  15. [Gorgias: E-commerce Helpdesk Features](https://www.gorgias.com/product/ecommerce-helpdesk)
  16. [Reddit: "Gorgias is too expensive for a small shop"](https://www.reddit.com/r/ecommerce/comments/13l6a9d/gorgias_is_too_expensive_for_a_small_shop/)
  17. [Kustomer CRM Overview](https://www.kustomer.com/)
  18. [Attentive AI SMS Marketing](https://www.attentive.com/ai)
  19. [Klaviyo AI Features](https://www.klaviyo.com/features/ai)
  20. [Notion AI for Workspaces](https://www.notion.so/product/ai)
  21. [Microsoft Copilot for Sales Features](https://adoption.microsoft.com/en-us/copilot-for-sales/)
  22. [Reddit r/sweatystartup: "How do you manage leads from Facebook and Instagram?"](https://www.reddit.com/r/sweatystartup/comments/12y8b3m/how_do_you_manage_leads_from_facebook_and/)
  23. [HubSpot Free CRM vs Small Business Needs](https://www.hubspot.com/products/crm)
  24. [Trustpilot: HubSpot Reviews (Complexity for Solo Operators)](https://www.trustpilot.com/review/hubspot.com)
  25. [Wix Inbox Features](https://www.wix.com/ascend/inbox)
  26. [Reddit: "Wix vs Shopify for a baker"](https://www.reddit.com/r/WixHelp/comments/11v3d4x/wix_vs_shopify_for_a_baker/)
  27. [Squarespace Email Campaigns & Automation](https://www.squarespace.com/email-marketing)
  28. [GoDaddy Conversations (Unified Inbox app)](https://www.godaddy.com/help/what-is-godaddy-conversations-28114)
  29. [Trustpilot: GoDaddy Reviews (Upsell complaints)](https://www.trustpilot.com/review/www.godaddy.com)
  30. [Square Messages Product Update](https://squareup.com/us/en/messages)
  31. [Reddit: "Square appointments vs Acuity"](https://www.reddit.com/r/smallbusiness/comments/10q5a8x/square_appointments_vs_acuity/)
  32. [Acuity Scheduling Automation Features](https://acuityscheduling.com/)
  33. [Calendly AI Integrations](https://calendly.com/integration)
  34. [Jobber (Field Service CRM) Features](https://getjobber.com/)
  35. [Reddit r/smallbusiness: "Jobber vs Housecall Pro for handyman"](https://www.reddit.com/r/smallbusiness/comments/11h8d5m/jobber_vs_housecall_pro_for_handyman/)
  36. [Housecall Pro AI dispatch features](https://www.housecallpro.com/)
  37. [ServiceTitan (Enterprise field service vs SMB)](https://www.servicetitan.com/)
  38. [HoneyBook Client Management for Creatives](https://www.honeybook.com/)
  39. [Reddit: "Is HoneyBook worth it for a solo photographer?"](https://www.reddit.com/r/WeddingPhotography/comments/12a9c4m/is_honeybook_worth_it_for_a_solo_photographer/)
  40. [Dubsado (CRM for Creatives)](https://www.dubsado.com/)
  41. [Thryv Small Business Software](https://www.thryv.com/)
  42. [Trustpilot: Thryv Reviews](https://www.trustpilot.com/review/thryv.com)
  43. [Podium (Unified SMS Inbox & Reviews)](https://www.podium.com/)
  44. [Reddit: "Podium pricing is insane for small local business"](https://www.reddit.com/r/smallbusiness/comments/13b7c2m/podium_pricing_is_insane_for_small_local_business/)
  45. [Birdeye (Reputation & Messaging)](https://birdeye.com/)
  46. [Broadly (SMB Messaging)](https://broadly.com/)
  47. [ManyChat (Instagram/Messenger Automation)](https://manychat.com/)
  48. [Reddit: "Setting up ManyChat for IG DMs"](https://www.reddit.com/r/InstagramMarketing/comments/14e5f6m/setting_up_manychat_for_ig_dms/)
  49. [Chatfuel (AI Chatbots for Meta)](https://chatfuel.com/)
  50. [Meta Business Suite (Native unified inbox limits)](https://www.facebook.com/business/tools/meta-business-suite)

  ## Design Doc
  **Architecture:**
  - Introduce an `OmnichannelTriage` module.
  - Entities: `InboundMessage`, `TriageIntent`, `ProposedAction`.
  - Frontend: 375px mobile-first layout. A single "Feed" tab. Cards show the message, detected intent (Booking, Support, Lead), and a 1-tap "Accept Proposal" button to draft a reply or send a booking link.

  ## Implementation Prompt
  Create the `OmnichannelTriage` view. The CUJ begins with the user opening the OHC app. They should see a prioritized feed of inbound requests. When they click a request from "Instagram", they should see the AI-drafted reply and a button to "Send". No manual typing required. Implement this across mobile (375px) and desktop. Ensure 100% test coverage and E2E Playwright tests.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
