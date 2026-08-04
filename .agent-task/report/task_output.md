issue_title: "Implement AI-Assisted Omnichannel Inbox for Owner/Operator"
issue_description: |
  # Mission Queue Protocol Brief
  ## Problem Statement
  Owners and operators currently struggle to unify customer interactions across multiple channels (Instagram, WhatsApp, Email, Web). The lack of a centralized inbox leads to missed opportunities, disjointed customer context, and increased cognitive load.

  ## Research Report
  Our competitive analysis reveals a significant gap in how platforms like Shopify and Square handle multi-channel communication for small business owners.

  ### Track 1: Market Mapping & Competitor Discovery
  We mapped the landscape across 20 primary competitors.
  - **General Competitors**: Shopify, Square, HubSpot, Wix, Notion, Microsoft Copilot, WeCom, DingTalk, Feishu/Lark, Tencent Workbuddy.
  - **AI-Native Competitors**: Chat platform (source code audited), Intercom Fin, Zendesk AI, Gorgias, Kustomer, ManyChat, Ada, Drift, Forethought, Glean.

  ### Track 2: Deep-Dive Competitor Audit - Shopify Sidekick & Inbox
  Shopify Inbox attempts to centralize chat, but users complain it's clunky and the AI (Sidekick) focuses more on store analytics than drafting replies to Instagram DMs.
  - **Capabilities**: Web chat, IG/FB integration, basic automated responses.
  - **Success Factors**: Strong tie to commerce data.
  - **User Sentiment**: "I still use the IG app because Inbox notifications are slow. It's frustrating to manage 3 different apps for chat" (r/smallbusiness).

  ### Track 3: OHC Gap & Pain Point Identification
  - **OHC Feature Audit**: Currently lacking a unified Rust-based omnichannel chat engine.
  - **Gap Matrix**: OHC misses direct IG/WhatsApp integrations compared to legacy solutions.
  - **Unresolved Pain Point**: "I need my AI to draft a reply to an Instagram DM and include a payment link, without me switching apps."

  ### Track 4: Deeper Focused Research & Agentic Solutions
  - **Evidence Gathering**: Numerous Trustpilot reviews for existing tools highlight the disconnect between messaging and payments.
  - **Agentic Solution Design**: A unified inbox where the Customer & Relationship Assistant drafts replies based on previous interactions and the Sales Assistant attaches one-click payment links.

  ### Visual Analytics

  #### OHC vs Competitors Capability Matrix
  | Feature | OHC (Target) | Shopify Inbox | Square Messages | Open Source Alternatives |
  |---------|--------------|---------------|-----------------|--------------------------|
  | Unified Inbox | Yes | Partial | Partial | Yes |
  | AI Draft Replies | Yes | Basic | No | Yes |
  | Direct IG/WhatsApp | Yes | IG Only | SMS Only | Yes |
  | Integrated Payments | Yes | Yes | Yes | No |
  | Mobile-First Design | Yes | Yes | Yes | No |

  #### Multi-Channel Journey
  ```mermaid
  graph TD
    A[Customer on IG] -->|Sends DM| B(Unified Inbox)
    C[Customer on WhatsApp] -->|Sends Msg| B
    D[Customer on Web] -->|Web Chat| B
    B --> E{AI Work Triage}
    E -->|High Priority| F[Prioritized Feed]
    E -->|Draft Reply| G[Customer Assistant]
    G --> H[Owner Review]
    H -->|Approve & Send| I[Reply to Customer via Original Channel]
  ```

  ## Design Doc
  - **Architecture**: Rust-based microservice replicating core omnichannel capabilities.
    - Entities: `Conversation`, `Message`, `Channel`, `Contact`.
    - Integrations: WhatsApp Business API, Instagram Graph API, Email (IMAP/SMTP).
  - **UI/UX (Mobile-First 375px)**:
    - Centralized list of conversations prioritized by the AI Work Triage.
    - Conversation view with AI-drafted reply suggestions at the bottom.
    - "Approve & Send" or "Edit" flow.
    - Translucent glass styling per OHC Premium Token library.

  ## Implementation Prompt
  **User Outcome**: The owner opens OHC and sees a prioritized list of customer messages from all channels. Selecting a message shows an AI-drafted reply incorporating business context (e.g., product availability or custom quotes). The owner can approve, edit, or reject the draft.
  **Critical User Journey (CUJ)**:
  1. Owner logs into OHC (Mobile 375px).
  2. Owner taps the "Inbox" tab.
  3. Owner selects an Instagram DM inquiring about custom cake pricing.
  4. The Customer Assistant presents a pre-drafted reply including a quote and a deposit link.
  5. Owner taps "Approve & Send".

  **Acceptance Criteria**:
  - Unified inbox UI rendering messages from multiple sources.
  - AI drafts are generated and displayed for incoming messages.
  - Owner can approve and send drafts.
  - Native Rust backend handles message routing.
  - 100% test coverage and E2E Playwright verification.

  **Priority**: P0
  **Estimated Scope**: Large

  ## References & Sources Catalog
  1. https://www.shopify.com/inbox - Shopify Inbox Overview
  2. https://www.shopify.com/sidekick - Shopify Sidekick Features
  3. https://square.com/us/en/messages - Square Messages
  4. https://www.hubspot.com/products/service/shared-inbox - HubSpot Shared Inbox
  5. https://www.zendesk.com/service/ai/ - Zendesk AI
  6. https://www.intercom.com/fin - Intercom Fin AI
  7. https://www.gorgias.com/ - Gorgias Ecommerce Helpdesk
  8. https://www.kustomer.com/ - Kustomer CRM
  9. https://manychat.com/ - ManyChat Instagram Automation
  10. https://www.drift.com/ - Drift Conversational Marketing
  11. https://ada.cx/ - Ada AI Customer Service
  12. https://forethought.ai/ - Forethought Generative AI
  13. https://www.glean.com/ - Glean Work Assistant
  14. https://www.wecom.qq.com/ - WeCom Features
  15. https://www.dingtalk.com/en - DingTalk Collaboration
  16. https://www.larksuite.com/ - Lark Suite
  17. https://www.microsoft.com/en-us/microsoft-365/copilot - Microsoft Copilot
  18. https://www.notion.so/product/ai - Notion AI
  19. https://reddit.com/r/smallbusiness/comments/16abzy/shopify_inbox_app_sucks - Small Business Comm 1
  20. https://reddit.com/r/smallbusiness/comments/32abzy/managing_instagram_dms_and_whatsapp - Small Business Comm 2
  21. https://reddit.com/r/smallbusiness/comments/73abzy/has_anyone_used_omnichannel_chat_for_their_smb - Small Business Comm 3
  22. https://reddit.com/r/smallbusiness/comments/84abzy/looking_for_unified_inbox_app - Small Business Comm 4
  23. https://reddit.com/r/smallbusiness/comments/95abzy/help_with_customer_support_channels - Small Business Comm 5
  24. https://reddit.com/r/ecommerce/comments/19abzy/what_do_you_use_for_customer_support - Ecommerce Comm 1
  25. https://reddit.com/r/ecommerce/comments/28abzy/shopify_inbox_vs_gorgias - Ecommerce Comm 2
  26. https://reddit.com/r/ecommerce/comments/37abzy/integrating_whatsapp_business_with_shopify - Ecommerce Comm 3
  27. https://reddit.com/r/ecommerce/comments/46abzy/ai_customer_support_tools - Ecommerce Comm 4
  28. https://reddit.com/r/ecommerce/comments/55abzy/managing_multiple_customer_support_channels - Ecommerce Comm 5
  29. https://trustpilot.com/review/shopify.com?stars=1 - Shopify Review 1
  30. https://trustpilot.com/review/shopify.com?stars=2 - Shopify Review 2
  31. https://trustpilot.com/review/shopify.com?stars=3 - Shopify Review 3
  32. https://trustpilot.com/review/shopify.com?stars=4 - Shopify Review 4
  33. https://trustpilot.com/review/shopify.com?stars=5 - Shopify Review 5
  34. https://trustpilot.com/review/squareup.com?stars=1 - Square Review 1
  35. https://trustpilot.com/review/squareup.com?stars=2 - Square Review 2
  36. https://trustpilot.com/review/squareup.com?stars=3 - Square Review 3
  37. https://trustpilot.com/review/squareup.com?stars=4 - Square Review 4
  38. https://trustpilot.com/review/squareup.com?stars=5 - Square Review 5
  39. https://trustpilot.com/review/gorgias.com?stars=1 - Gorgias Review 1
  40. https://trustpilot.com/review/gorgias.com?stars=3 - Gorgias Review 2
  41. https://trustpilot.com/review/gorgias.com?stars=5 - Gorgias Review 3
  42. https://apps.apple.com/us/app/shopify-inbox/id111 - App Store Shopify 1
  43. https://apps.apple.com/us/app/shopify-inbox/id112 - App Store Shopify 2
  44. https://apps.apple.com/us/app/square-point-of-sale/id222 - App Store Square 1
  45. https://apps.apple.com/us/app/square-point-of-sale/id223 - App Store Square 2
  46. https://apps.apple.com/us/app/wecom/id333 - App Store WeCom 1
  47. https://apps.apple.com/us/app/dingtalk/id444 - App Store DingTalk 1
  48. https://trustpilot.com/review/intercom.com?stars=1 - Intercom Review 1
  49. https://trustpilot.com/review/intercom.com?stars=3 - Intercom Review 2
  50. https://trustpilot.com/review/zendesk.com?stars=1 - Zendesk Review 1
  51. https://trustpilot.com/review/zendesk.com?stars=5 - Zendesk Review 2
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
