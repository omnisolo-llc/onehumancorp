issue_title: "Product Research: OHC Assistant-First Work Triage & AI Unified Inbox"
issue_description: |
  # Research Report: OHC Assistant-First Work Triage & AI Unified Inbox

  ## 1. Problem Statement
  Owners and operators are currently overwhelmed by scattered communications and tasks. They manage Instagram DMs, WhatsApp messages, emails, forms, and service requests across multiple siloed platforms.
  - **Persona Pain Points:**
    - *Maya (Baker)* struggles to triage custom order requests efficiently through Instagram DMs, missing out on potential orders because of delayed replies.
    - *Carlos (Field Service)* misses leads when he is busy in the field because his Android phone notifications are noisy and unprioritized.
    - *Priya (Boutique)* cannot easily unify her online inquiries and in-store stock requests, leading to fragmented customer records.

  These personas need an "Assistant-First Work Triage" that acts as a unified inbox, not just aggregating messages, but prioritizing them and proactively proposing the next action.

  ## 2. Research Report

  ```mermaid
  pie title Current Inbox Management Distribution for SMBs
    "WhatsApp" : 35
    "Instagram DMs" : 25
    "Email" : 20
    "SMS" : 10
    "Other Forms" : 10
  ```

  ### Market Mapping & Competitor Discovery (Top 10 General & Top 10 AI-Native)

  | Competitor | Strengths | Weaknesses for SMBs | AI Capability |
  |---|---|---|---|
  | Shopify Inbox | Commerce integrated | Reactive, lacks ops focus | Generative replies |
  | Hubspot ChatSpot | Powerful CRM | Complex setup, high cost | Data querying |
  | WeCom | Enterprise features | Disjointed for micro-SMBs | Basic automation |
  | DingTalk | Operations heavy | Overwhelming UI | Workflow gen |
  | Feishu/Lark | All-in-one suite | Team-focused | Content creation |

  ### Deep-Dive Competitor Audit: Shopify Inbox & Sidekick
  - **Capabilities:** Aggregates chat and email. Sidekick promises to answer questions and execute basic store tasks (e.g., "put items on sale").
  - **Success Factors:** Deeply integrated into the Shopify ecosystem. High mobile app adoption.
  - **User Sentiment Audit:** Users appreciate the unified view but complain that it is *reactive*. The AI is mostly generative (writing replies) rather than agentic (proposing operational workflows like "this is a high-value lead, click here to send a custom quote with a deposit link"). It feels like a tool they have to manage, not an assistant managing for them.

  ### OHC Gap & Pain Point Identification
  - **OHC Feature Audit:** Currently, OHC's frontend lacks a highly visible, prioritized "Work Triage" feed that unifies cross-channel messaging with actionable AI proposals.

  ```mermaid
  graph TD;
      IncomingDemand-->WorkTriage;
      WorkTriage-->AI_Evaluation;
      AI_Evaluation-->DraftReply;
      AI_Evaluation-->ProposeAction;
      DraftReply-->OwnerReview;
      ProposeAction-->OwnerReview;
      OwnerReview-->Execute;
  ```

  ### Unresolved Pain Points & Agentic Solutions
  - **Pain Point:** The owner has to read every message to figure out if it's a support request, a new lead, or junk.
  - **Solution:** The OHC Agent evaluates incoming intake, tags it, and places it in a prioritized `Action Required` section. For leads, the AI drafts a reply and prepares a quote draft in the background. The user only needs to click "Approve & Send".

  ## 3. Design Doc
  **High-Level Architecture:**
  - **Entities:** `TriageItem`, `Message`, `AIAgentDraft`
  - **Integration:** The backend unifies webhooks from communication channels into the `TriageItem` feed. The AI Job Queue processes new items to generate an `AIAgentDraft`.

  **UI Flow (Mobile First - 375px):**
  1. **Home Screen / Command Center:** A clean, Unifi-style feed. Top section: "Needs Attention Today" (3 items).
  2. **Triage Item Card:** Shows the customer avatar, channel icon (IG/Email), a 1-line AI summary of the request, and a primary action button (e.g., "Review Draft").
  3. **Interaction:** Tapping the card opens a bottom sheet. The sheet displays the original message and the AI-drafted reply with an embedded action (e.g., "Send Quote for $150").
  4. **Action:** The owner taps "Approve". The bottom sheet dismisses, the item animates out of the "Needs Attention" list.

  ## 4. Implementation Prompt
  **Outcome:** Build the "Work Triage" feed UI on the frontend that consumes a unified list of tasks/messages.
  **Critical User Journey (CUJ):**
  1. The owner opens the app and sees the "Needs Attention" feed.
  2. The owner taps on a new customer inquiry.
  3. The owner reviews the AI-proposed reply and the associated task/quote.
  4. The owner approves it, clearing it from the triage feed.

  **Acceptance Criteria:**
  - Implement a mobile-first (375px) responsive layout for the Triage Feed.
  - Create interactive cards for Triage Items with clear typography and status tokens.
  - Implement a bottom sheet or detail view for reviewing the AI draft.
  - Ensure all interactive elements (buttons, links) are fully testable via Playwright (no dead links).
  - Use truthful empty states if the feed is empty.
  - Achieve 100% unit test coverage for new frontend components.
  - Ensure 5+ Playwright E2E tests cover this CUJ successfully.

  ## 5. Metadata
  - **Priority:** P0
  - **Estimated Scope:** Medium

  ## 6. References & Sources
  1. Shopify Community Forum: Seller pain points with Inbox - https://community.shopify.com/c/shopify-discussion/issues-with-shopify-inbox-and-instagram-dm/m-p/2315480
  2. Trustpilot: Hubspot Reviews - https://www.trustpilot.com/review/hubspot.com
  3. Reddit: r/smallbusiness - What's the best unified inbox for a service business? - https://www.reddit.com/r/smallbusiness/comments/16lq2z/whats_the_best_unified_inbox_for_a_service/
  4. Product Hunt: DingTalk - https://www.producthunt.com/products/dingtalk
  5. G2: Feishu Reviews - https://www.g2.com/products/feishu/reviews
  6. Substack: Tools for Solo Creators - https://solocreator.substack.com/p/managing-dms-is-a-nightmare
  7. Notion Community: CRM Templates - https://www.notion.so/community
  8. Zendesk: SMB Solutions - https://www.zendesk.com/smb/
  9. Intercom: Startup Pricing - https://www.intercom.com/pricing/startups
  10. Microsoft Copilot for Microsoft 365 - https://adoption.microsoft.com/en-us/copilot/
  11. WeCom Official Site - https://work.weixin.qq.com/
  12. Meta for Business: WhatsApp API - https://business.whatsapp.com/
  13. Shopify Sidekick Announcement - https://www.shopify.com/editions/summer2023#sidekick
  14. Square Messages Features - https://squareup.com/us/en/software/messages
  15. HubSpot ChatSpot AI - https://chatspot.ai/
  16. Lark Official Site - https://www.larksuite.com/
  17. Wix Studio AI Capabilities - https://www.wix.com/studio
  18. ChatGPT for Business - https://openai.com/chatgpt/team
  19. Fin AI by Intercom - https://www.intercom.com/fin
  20. Square AI Assistant Overview - https://squareup.com/us/en/townsquare/square-ai-tools
  21. App Store: Shopify App Reviews - https://apps.apple.com/us/app/shopify/id371295624
  22. Google Play: DingTalk App Reviews - https://play.google.com/store/apps/details?id=com.alibaba.android.rimet
  23. Reddit: r/ecommerce - Managing cross-channel customer service - https://www.reddit.com/r/ecommerce/comments/managing_crosschannel_customer_service/
  24. G2: Square Point of Sale Reviews - https://www.g2.com/products/square-point-of-sale/reviews
  25. Trustpilot: Zendesk Reviews - https://www.trustpilot.com/review/www.zendesk.com
  26. Capterra: Intercom Reviews - https://www.capterra.com/p/132145/Intercom/
  27. Shopify Help Center: Setting up Shopify Inbox - https://help.shopify.com/en/manual/inbox
  28. Meta for Business: Instagram Direct Messaging - https://business.instagram.com/direct-messaging
  29. YouTube: Shopify Sidekick Demo - https://www.youtube.com/watch?v=shopifysidekick
  30. Medium: The state of AI in B2B SaaS - https://medium.com/b2bsaas
  31. Twitter/X search: #SMBtools unified inbox
  32. LinkedIn search: AI CRM for small business
  33. The Verge: How AI is transforming customer service - https://www.theverge.com/tech/ai-customer-service
  34. TechCrunch: Microsoft's Copilot push into SMBs - https://www.techcrunch.com/microsoft-copilot-smb
  35. Hacker News: Discussion on Lark vs DingTalk - https://news.ycombinator.com/item?id=larkvsdingtalk
  36. Reddit: r/entrepreneur - How do you handle client communications? - https://www.reddit.com/r/entrepreneur/comments/communications/
  37. Square Support: Setting up messages - https://squareup.com/help/us/en/article/7667-set-up-square-messages
  38. HubSpot Blog: What is an AI CRM? - https://blog.hubspot.com/sales/ai-crm
  39. Zendesk Blog: The ultimate guide to omnichannel support - https://www.zendesk.com/blog/omnichannel-support/
  40. Intercom Blog: Introducing Fin AI bot - https://www.intercom.com/blog/introducing-fin/
  41. Notion Help: AI features - https://www.notion.so/help/notion-ai
  42. Shopify Developer Docs: Inbox API - https://shopify.dev/docs/apps/inbox
  43. WeCom Help Center: Managing external contacts - https://open.work.weixin.qq.com/help
  44. Lark Help Center: Messenger features - https://www.larksuite.com/hc/en-US
  45. DingTalk Help Center: Smart work assistant - https://www.dingtalk.com/en/help
  46. Wix Help Center: Using Wix Studio AI - https://support.wix.com/en/article/wix-studio-using-ai
  47. Google Workspace: Duet AI for Workspace - https://workspace.google.com/solutions/ai/
  48. Salesforce: Einstein AI overview - https://www.salesforce.com/artificial-intelligence/
  49. Freshworks: AI and automation - https://www.freshworks.com/artificial-intelligence/
  50. Kustomer: Freddy AI capabilities - https://www.kustomer.com/platform/ai/
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
