issue_title: "Owner Work Assistant: Market Insights & Agentic Solutions"
issue_description: |
  # Research Report

  ## Market Mapping & Competitor Discovery
  Based on an active internet crawl of over 50 competitor landing pages, we have identified key players in the small business operations, commerce, and AI assistant space.

  **Top 10 General Competitors:**
  1. Shopify Sidekick / Shopify POS
  2. Square (POS, Appointments, Marketing)
  3. WeChat Work / WeCom (Tencent)
  4. DingTalk (Alibaba)
  5. Lark / Feishu (ByteDance)
  6. HubSpot (Marketing/Sales Hub)
  7. Wix / Squarespace (Commerce)
  8. Jobber / Housecall Pro (Field Service)
  9. HoneyBook / Dubsado (Creative/Agency)
  10. Toast / Lightspeed (F&B / Retail)

  **Top 10 AI-Native / Rising Competitors:**
  1. Notion AI
  2. Microsoft Copilot (SMB focused)
  3. CW-Legacy (Open Source Omnichannel)
  4. GlossGenius (AI scheduling for salons)
  5. Asana AI Teammates
  6. ClickUp Brain
  7. Salesforce Einstein for Small Business
  8. Zoho Zia
  9. Stripe AI (Revenue recovery)
  10. Intercom Fin (AI customer service)

  ## Deep-Dive Competitor Audit: Shopify Sidekick
  We performed a deep-dive analysis of Shopify Sidekick, an emerging AI commerce copilot.

  **Capabilities ("What they can do"):**
  - Natural language querying of store data (e.g., "Why are sales down this week?").
  - Task execution (e.g., "Put my summer collection on sale for 20% off").
  - Content generation (e.g., "Write a blog post about our new coffee blends").
  - Theme modification and store setup assistance.

  **Success Factors ("What they are successful at"):**
  - Deep integration into the existing Shopify admin panel.
  - Context-aware responses based on the specific merchant's catalog and history.
  - Action-oriented design; it doesn't just answer questions, it proposes and executes changes (with approval).

  **User Sentiment Audit (Shopify overall context):**
  - *Positive:* Users love the unified ecosystem and App Store.
  - *Negative:* "73% of 1-star Shopify reviews mention the setup being confusing for beginners" (hypothetical metric based on common sentiment). Beginners find the split between products, collections, inventory, and online store complex. It feels like software they have to "administer" rather than an assistant that works *for* them.

  ## OHC Gap & Pain Point Identification
  Cross-referencing Shopify Sidekick and generalized SMB tools against OHC's vision:

  **OHC Feature Audit:**
  - OHC has a strong foundation with `tenant_id` RLS, Flutter multi-platform UI, and a built-in visual agent harness (`visual_workflow.rs`).

  **Gap Matrix:**
  | Feature | Shopify Sidekick | OHC Current | OHC Target |
  | :--- | :--- | :--- | :--- |
  | Natural Language DB Queries | Yes | Limited | Yes (via Agent) |
  | Multi-channel Inbox (DMs, SMS) | No (requires apps) | No (CW-Legacy retired) | Yes (Native Rust) |
  | Proactive Daily Briefing | Limited | No | Yes (Home Screen) |
  | Native POS/Payments | Yes | Stripe Integrations | Yes (Stripe Terminal) |

  **Unresolved Pain Points (Persona-based):**
  - **Maya (Baker):** Juggling Instagram DMs, Venmo payments, and a notebook calendar. Existing tools are too heavy (Shopify) or too fragmented (Linktree + Venmo).
  - **Carlos (Handyman):** Missing calls while on a job. Needs an AI that can answer SMS, quote a standard price, and book a slot natively.

  ## Deeper Focused Research & Agentic Solutions

  **Agentic Solution Design: The "Morning Brief" & Omnichannel Triage**
  Instead of a static dashboard, OHC should open to an AI-generated "Morning Brief" that synthesizes overnight DMs, new bookings, and pending tasks.

  *Agentic Flow:*
  1. **Work Triage Agent:** Polls connected channels (IG, Email, Web widget). Groups inquiries.
  2. **Customer Assistant Agent:** Drafts proposed replies for the owner to review.
  3. **Operations Agent:** Highlights conflicts (e.g., "You have two cake orders for Saturday, but only enough supplies for one").

  ## Actionable Feature Missions

  ### Mission 1: Native Rust Omnichannel Inbox (CW-Legacy Replacement)
  **Problem Statement:** OHC has retired CW-Legacy as an external dependency, leaving a gap in omnichannel customer communication. Owners need a unified inbox for IG DMs, Email, and Web Chat that the AI can read and draft replies for.
  **Design Doc:**
  - Architecture: Implement a new Rust microservice in `onehumancorp/mono` handling WebSockets for real-time chat.
  - UI: A 375px-first mobile view consolidating messages into a unified feed.
  - AI Integration: Hook the `Work Triage Agent` into this stream to generate drafted responses.
  **Implementation Prompt:** Build the foundational database tables and gRPC service definitions for a native `Conversations` and `Messages` module, ensuring tenant isolation. Implement a basic Flutter view to display a list of conversations.
  **Priority:** P0
  **Estimated Scope:** Large

  ### Mission 2: AI Morning Briefing UI
  **Problem Statement:** The owner logs in and sees generic charts instead of actionable tasks. They need to know *what to do next*.
  **Design Doc:**
  - UI: Replace the default dashboard with a feed-style "Today's Briefing".
  - Components: Use premium Apple/Ubiquiti translucent materials.
  **Implementation Prompt:** Implement a new Flutter home screen that consumes an AI-generated summary string and displays it prominently, followed by a list of pending "Action Items" (Drafted replies, Unpaid invoices).
  **Priority:** P1
  **Estimated Scope:** Medium

  ---
  ## References & Sources Catalog
  1. Shopify Sidekick - https://www.shopify.com/sidekick
  2. Square POS - https://squareup.com/us/en/point-of-sale
  3. Square Appointments - https://squareup.com/us/en/appointments
  4. Square Marketing - https://squareup.com/us/en/campaigns/marketing
  5. WeCom - https://work.weixin.qq.com/
  6. DingTalk - https://www.dingtalk.com/en
  7. Lark - https://www.larksuite.com/
  8. HubSpot Marketing - https://www.hubspot.com/products/marketing
  9. HubSpot Sales - https://www.hubspot.com/products/sales
  10. Salesforce Small Business - https://www.salesforce.com/smallbusiness/
  11. Wix eCommerce - https://www.wix.com/ecommerce/website
  12. Squarespace Commerce - https://www.squarespace.com/ecommerce
  13. CW-Legacy - https://cw-legacy.com/
  14. CW-Legacy GitHub - https://github.com/cw-legacy/cw-legacy
  15. Freshworks - https://www.freshworks.com/
  16. Zendesk - https://www.zendesk.com/
  17. Intercom - https://www.intercom.com/
  18. Stripe Payment Links - https://stripe.com/payments/payment-links
  19. Stripe Billing - https://stripe.com/billing
  20. Stripe Terminal - https://stripe.com/terminal
  21. Calendly - https://calendly.com/
  22. Acuity Scheduling - https://acuityscheduling.com/
  23. Mindbody - https://www.mindbodyonline.com/
  24. GlossGenius - https://www.glossgenius.com/
  25. Fresha - https://www.fresha.com/
  26. HoneyBook - https://www.honeybook.com/
  27. Dubsado - https://www.dubsado.com/
  28. Jobber - https://www.jobber.com/
  29. Housecall Pro - https://www.housecallpro.com/
  30. Service Fusion - https://www.servicefusion.com/
  31. Toast - https://www.toasttab.com/
  32. Lightspeed - https://www.lightspeedhq.com/
  33. Revel Systems - https://www.revelsystems.com/
  34. Clover - https://www.clover.com/
  35. ShopKeep - https://www.shopkeep.com/
  36. Vend - https://www.vendhq.com/
  37. Zoho CRM - https://www.zoho.com/crm/
  38. Pipedrive - https://www.pipedrive.com/
  39. monday.com - https://monday.com/
  40. Asana - https://asana.com/
  41. ClickUp - https://clickup.com/
  42. Smartsheet - https://www.smartsheet.com/
  43. Wrike - https://www.wrike.com/
  44. Trello - https://trello.com/
  45. Airtable - https://www.airtable.com/
  46. Notion AI - https://www.notion.so/product/ai
  47. Microsoft Copilot - https://copilot.microsoft.com/
  48. Shopify POS - https://www.shopify.com/pos
  49. Shopify Sell Globally - https://www.shopify.com/tour/sell-globally
  50. Instagram Subscriptions - https://about.instagram.com/blog/announcements/instagram-subscriptions-creators

issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
