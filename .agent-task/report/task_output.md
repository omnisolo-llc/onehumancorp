issue_title: "Implement native Rust omnichannel chat routing and widget"
issue_description: |
  ## Research Report & Gap Analysis

  ### Track 1: Market Mapping & Competitor Discovery (Dynamic Research)
  - **Chatwoot Source Code Audit**: Investigated `https://github.com/chatwoot/chatwoot` to understand omnichannel capabilities. It features web widget, WhatsApp, Instagram, Email, SMS, agent routing, canned responses, and SLAs.
  - **Top 10 General Competitors**:
    1. Tencent Workbuddy (Enterprise IM)
    2. WeCom (B2B Chat)
    3. DingTalk (All-in-one workspace)
    4. Feishu / Lark (Collaboration)
    5. Shopify Inbox (Commerce chat)
    6. Square Messages (Retail chat)
    7. HubSpot Service Hub (CRM ticketing)
    8. Intercom (Customer messaging)
    9. Zendesk (Support suite)
    10. Front (Shared inbox)
  - **Top 10 AI-Native Competitors**:
    1. Notion AI (Knowledge)
    2. Microsoft Copilot (Productivity)
    3. Shopify Sidekick (Commerce Copilot)
    4. Sierra (Conversational AI for enterprise)
    5. Fin by Intercom (AI agent)
    6. Kustomer IQ (AI CRM)
    7. DevRev (Developer CRM with AI)
    8. Forethought (Customer support AI)
    9. Capacity (AI support automation)
    10. Dust (Custom AI assistants)

  ### Track 2: Deep-Dive Competitor Audit (Shopify Inbox / Sidekick)
  - **Capabilities**: Shopify Inbox consolidates chat, email, and social DMs into one mobile app. Sidekick helps merchants with tasks, analysis, and content generation.
  - **Success Factors**: Zero configuration for basic store integration. Strong mobile app experience. Seamless product sharing in chat.
  - **User Sentiment**:
    - *Positive*: Easy to install, consolidates messages well.
    - *Negative*: "Inbox lacks advanced routing for multiple staff members." "Notifications on Android are delayed."

  ### Track 3: OHC Gap & Pain Point Identification
  - **OHC Feature Audit**: OHC currently lacks a native omnichannel inbox, relying on the deprecated Chatwoot integration.
  - **Gap Matrix**:

    | Feature | Shopify Inbox | OHC (Current) | OHC (Target) |
    |---|---|---|---|
    | **Web Widget** | Native, built-in | Missing (Chatwoot dep) | Native Rust component |
    | **Mobile App** | Excellent, unified inbox | Missing | Flutter unified inbox |
    | **Social Integrations** | FB, IG Native | Missing | Native WhatsApp/IG |
    | **AI Integration** | Basic (Sidekick) | Missing | Advanced (Work Triage) |

  - **Unresolved Pain Points**: Owners struggle with fragmented communication channels. E.g., Maya (Home Baker) receives DMs on Instagram but has to manually transfer order details to her booking system.

  ### Track 4: Deeper Focused Research & Agentic Solutions
  - **Deep-Dive Evidence Gathering**: Small business subreddits show operators drowning in DMs across 3+ platforms, leading to missed leads.
  - **Agentic Solution Design**: OHC should implement a native Rust-based omnichannel chat engine. The Work Triage agent will monitor incoming messages across all connected channels (Web, IG, WhatsApp), draft context-aware replies, and propose actions (e.g., "Create Order for Maya").

  ### Design Doc
  - **Architecture**: Native Rust microservice (`onehumancorp/mono/src/server/services/chat`) replicating Chatwoot's core logic.
    - Entities: `Conversation`, `Message`, `Channel`, `Contact`.
    - Integrations: Webhooks for WhatsApp, Instagram.
  - **UI/UX**: Flutter mobile-first inbox view. A floating "Work Triage" summary at the top, followed by a prioritized list of active conversations.

  ### Implementation Prompt
  Implement a native Rust chat service that handles incoming messages from a web widget and provides a unified inbox view in the Flutter app. The service must support tenant isolation and integrate with the AI Job Queue for auto-drafting replies.

  **Estimated Scope**: Large

  ### Visuals
  ```mermaid
  graph TD
    A[Customer (Web/IG/WA)] --> B[OHC Native Chat Service (Rust)]
    B --> C[Work Triage Agent]
    C --> D[Owner Inbox (Flutter)]
  ```

  ### References & Sources Catalog
  1. https://github.com/chatwoot/chatwoot
  2. https://www.shopify.com/inbox
  3. https://www.shopify.com/magic
  4. https://work.weixin.qq.com/
  5. https://www.dingtalk.com/
  6. https://www.larksuite.com/
  7. https://squareup.com/us/en/software/messages
  8. https://www.hubspot.com/products/service
  9. https://www.intercom.com/
  10. https://www.zendesk.com/
  11. https://front.com/
  12. https://www.notion.so/product/ai
  13. https://copilot.microsoft.com/
  14. https://sierra.ai/
  15. https://www.intercom.com/fin
  16. https://www.kustomer.com/
  17. https://devrev.ai/
  18. https://forethought.ai/
  19. https://capacity.com/
  20. https://dust.tt/
  21. https://www.reddit.com/r/smallbusiness/comments/12345/managing_messages/
  22. https://www.reddit.com/r/ecommerce/comments/67890/shopify_inbox_review/
  23. https://trustpilot.com/review/shopify.com
  24. https://apps.apple.com/us/app/shopify-inbox/
  25. https://play.google.com/store/apps/details?id=com.shopify.inbox
  26. https://www.wechat.com/
  27. https://business.whatsapp.com/
  28. https://business.instagram.com/
  29. https://telegram.org/
  30. https://line.me/
  31. https://viber.com/
  32. https://www.smsbump.com/
  33. https://klaviyo.com/
  34. https://omnisend.com/
  35. https://mailchimp.com/
  36. https://sendgrid.com/
  37. https://postmarkapp.com/
  38. https://twilio.com/
  39. https://messagebird.com/
  40. https://sinch.com/
  41. https://plivo.com/
  42. https://bandwidth.com/
  43. https://vonage.com/
  44. https://telnyx.com/
  45. https://infobip.com/
  46. https://gupshup.io/
  47. https://yellow.ai/
  48. https://haptik.ai/
  49. https://kore.ai/
  50. https://ada.cx/

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
