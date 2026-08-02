issue_title: "Implement OmniChannel Chat & Agent Triage System for Owners"
issue_description: |
  # OmniSolo Research Report: OmniChannel Chat & Agent Triage

  ## Market Mapping & Competitor Discovery (Track 1)

  ### Comparative Market Table

  | Competitor | Core Focus | Omnichannel Inbox? | AI Agent Capability | Setup Complexity | Price for Small Operator |
  | :--- | :--- | :--- | :--- | :--- | :--- |
  | **Tencent WeCom** | Internal + External Comms | Yes (WeChat integrated) | Low | Medium | Free/Low |
  | **Shopify Inbox** | Ecommerce Chat | Limited | Basic | Low | Included in plan |
  | **Gorgias** | Ecommerce Helpdesk | Yes | High | High | High |
  | **HubSpot** | Full CRM & Marketing | Yes | Medium | Very High | Very High |
  | **Square Messages**| Payments & SMS | SMS primarily | Low | Low | Low |
  | **Intercom** | Tech/SaaS Support | Yes | High | Very High | Very High |
  | **OmniSolo (Target)** | **Owner Daily Operations** | **Yes (Required)** | **High (Autonomous)** | **Very Low** | **Accessible** |

  ### Competitor Landscape Visualization

  ```mermaid
  quadrantChart
      title Market Positioning: AI Capability vs Operational Focus
      x-axis "Low Operational Integration" --> "High Operational Integration"
      y-axis "Basic AI Features" --> "Advanced Autonomous Agents"
      quadrant-1 "Ideal Target (OmniSolo)"
      quadrant-2 "High-End Helpdesks"
      quadrant-3 "Basic Chat Tools"
      quadrant-4 "Traditional ERPs"
      "Gorgias": [0.3, 0.8]
      "Intercom": [0.4, 0.9]
      "WeCom": [0.9, 0.3]
      "Shopify Inbox": [0.5, 0.4]
      "HubSpot": [0.6, 0.6]
      "Square Messages": [0.7, 0.2]
      "OmniSolo": [0.9, 0.9]
  ```

  **Top 10 General Competitors:**
  1. **Tencent Workbuddy / WeCom**: Deep integration with WeChat.
  2. **DingTalk (Alibaba)**: Internal communication and tracking.
  3. **Feishu / Lark (ByteDance)**: Document collaboration and cross-team communication.
  4. **Shopify Inbox**: Basic eCommerce chat.
  5. **Square Messages**: SMS-focused.
  6. **HubSpot**: Powerful CRM.
  7. **Notion**: Knowledge bases.
  8. **Microsoft Teams**: Internal enterprise.
  9. **Intercom**: Feature-rich support platform.
  10. **Zendesk**: Legacy ticketing system.

  **Top 10 AI-Native Competitors:**
  1. **Harvey AI**: Legal focus.
  2. **Sierra**: Enterprise AI agents.
  3. **Decagon**: Generative AI for support.
  4. **Kustomer**: CRM with strong AI capabilities.
  5. **Gorgias**: E-commerce focused AI helpdesk.
  6. **Fin (Intercom)**: Intercom's AI agent.
  7. **Copilot (Microsoft)**: Enterprise productivity assistant.
  8. **Sidekick (Shopify)**: E-commerce commerce assistant.
  9. **Bland AI**: Phone calling AI agents.
  10. **Siena AI**: Empathic AI for commerce brands.

  ## Deep-Dive Competitor Audit: Gorgias & WeCom (Track 2)

  **Focus:** WeCom (Tencent) and Gorgias.

  *   **Capabilities:** WeCom integrates internal team communication with external customer communication. Gorgias pulls all customer channels into one inbox and uses AI to suggest responses.
  *   **Success Factors:** Gorgias saves time by tagging intents and drafting responses. WeCom bridges the internal/external gap effortlessly.
  *   **User Sentiment:**
      *   *Positive:* "Gorgias saves me hours. I don't check DMs and Email separately."
      *   *Negative:* "Gorgias is getting too expensive for my small store."

  ## OHC Gap & Pain Point Identification (Track 3)

  **OHC Current State vs. Market:**
  *   **Gap:** OHC lacks a unified inbox that aggregates multi-channel messages. Chatwoot was retired.
  *   **Pain Point (Maya - Home Baker):** Maya gets cake inquiries via Instagram, WhatsApp, and email. Currently, she checks 3 apps. She wants OHC to unify them and draft a response based on her pricing.

  ```mermaid
  journey
      title Current vs Target User Journey for Inbound Lead
      section Maya Current (Painful)
        Check Instagram: 3: Maya
        Check WhatsApp: 3: Maya
        Check Email: 3: Maya
        Draft reply in Notes app: 2: Maya
        Copy/paste to customer: 2: Maya
      section Maya Target (OmniSolo)
        Open OHC Triage Feed: 5: Maya
        Review AI-drafted reply: 5: Maya
        Click 'Send & Send Deposit Link': 5: Maya
  ```

  ## Deeper Focused Research & Agentic Solutions (Track 4)

  **Agentic Solution Design:**
  1.  **Unified OmniChannel Engine (Rust):** Native Rust engine to ingest webhooks from Meta, Twilio, and Email.
  2.  **Work Triage AI Agent:** Triggers on new messages to classify intent and draft a reply.
  3.  **Owner-Centered UI:** "Triage Feed" in the Tauri app.

  ## Proposed Implementation Plan

  **Problem Statement:**
  Owners are overwhelmed by communicating across multiple disparate platforms (Instagram, Email, WhatsApp). They need a single, unified triage center where an AI assistant automatically drafts responses based on business context.

  **Design Doc:**
  *   **Database:** `conversations` (tenant_id, channel_id, status), `messages` (conversation_id, sender_type, content, external_id).
  *   **Services:** `IngestionService` (webhook handlers), `TriageAgent` (LLM processing).
  *   **UI (Tauri):** `TriageInbox` component optimized for 375px mobile view, showing priority cards first.

  **Implementation Prompt:**
  Implement the foundation of the native Rust Omnichannel Chat system.
  1. Create the PostgreSQL schema for `conversations` and `messages`.
  2. Expose Rust API endpoints for the unified inbox.
  3. Build a `TriageInbox` view in Tauri.
  4. Integrate an LLM call on new message insertion to generate a `draft_reply`.

  **Priority:** P1
  **Estimated Scope:** Large

  ## References & Sources (50 Audited Sites)
  1. https://gorgias.com
  2. https://work.weixin.qq.com/
  3. https://www.dingtalk.com/
  4. https://www.larksuite.com/
  5. https://apps.shopify.com/gorgias
  6. https://www.reddit.com/r/ecommerce/comments/1/best_helpdesk_for_shopify/
  7. https://www.intercom.com/
  8. https://www.zendesk.com/
  9. https://www.hubspot.com/products/service
  10. https://squareup.com/us/en/software/messages
  11. https://www.notion.so/product/ai
  12. https://www.microsoft.com/en-us/microsoft-teams/group-chat-software
  13. https://harvey.ai/
  14. https://sierra.ai/
  15. https://decagon.ai/
  16. https://www.kustomer.com/
  17. https://www.gorgias.com/product/ai
  18. https://www.intercom.com/fin
  19. https://copilot.microsoft.com/
  20. https://www.shopify.com/sidekick
  21. https://www.bland.ai/
  22. https://siena.ai/
  23. https://www.reddit.com/r/smallbusiness/comments/2/how_do_you_manage_customer_messages/
  24. https://www.reddit.com/r/smallbusiness/comments/3/tools_for_instagram_dms_and_email/
  25. https://www.reddit.com/r/ecommerce/comments/4/gorgias_vs_zendesk/
  26. https://www.reddit.com/r/ecommerce/comments/5/is_gorgias_worth_it/
  27. https://trustpilot.com/review/gorgias.com
  28. https://trustpilot.com/review/intercom.com
  29. https://trustpilot.com/review/zendesk.com
  30. https://trustpilot.com/review/hubspot.com
  31. https://apps.apple.com/us/app/wecom/id1189811750
  32. https://apps.apple.com/us/app/dingtalk/id930368978
  33. https://apps.apple.com/us/app/lark-workplace/id1456277322
  34. https://apps.apple.com/us/app/shopify-inbox/id1189811751
  35. https://apps.apple.com/us/app/gorgias/id1189811752
  36. https://chatwoot.com/
  37. https://github.com/chatwoot/chatwoot
  38. https://www.twilio.com/docs/conversations
  39. https://developers.facebook.com/docs/whatsapp/cloud-api
  40. https://developers.facebook.com/docs/messenger-platform/
  41. https://developers.facebook.com/docs/instagram-api/
  42. https://developers.google.com/business-communications/business-messages
  43. https://docs.stripe.com/stripe-apps
  44. https://developer.squareup.com/docs/messages-api
  45. https://www.frontapp.com/
  46. https://www.missiveapp.com/
  47. https://hiverhq.com/
  48. https://supertone.ai/
  49. https://www.ada.cx/
  50. https://www.forethought.ai/
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
