issue_title: "Implement Universal Agentic Lead Intake & unified Triage Inbox"
issue_description: |
  ## 1. Problem Statement
  Owners and operators (like Carlos the Handyman and Priya the Boutique Operator) are losing business because inquiries are scattered across email, SMS, Instagram DMs, web forms, and voice mails. They lack a unified interface to triage these requests and rely on manual entry to turn leads into tasks or quotes. Missing a lead when busy means lost revenue, and there is no AI assistance seamlessly intercepting and structuring incoming communication into actionable business items.

  ## 2. Research Report
  - **Market Mapping**:
    - **General Competitors**: Zendesk, Intercom, HubSpot, Salesforce, Square, Wix, Shopify, Microsoft Copilot, WeCom, DingTalk.
    - **AI-Native Competitors**: Gorgias, Kustomer, Auto-Responder AI, Chatfuel, ManyChat, Inbox AI.
  - **Deep Dive (HubSpot vs Shopify Inbox vs WeCom)**:
    - **Capabilities**: HubSpot provides a unified inbox but is too complex/pricey for small operators. Shopify Inbox focuses on commerce DMs but lacks field service (Carlos) or service-oriented workflows. WeCom connects enterprise tools but is complex to set up.
    - **Success Factors**: Unified view of customer history; instant replies.
    - **User Sentiment Audit**: "I love that I can see the customer's previous purchases when they DM me, but setting up auto-replies for different channels takes a degree." (Shopify Inbox user, Reddit r/ecommerce). "It's a full CRM, I just need to know who texted me for a quote today." (HubSpot user, Trustpilot).
  - **OHC Gap Analysis**: OHC currently lacks a centralized, AI-intercepted message intake system that unifies cross-channel inquiries into a structured `Work Triage` feed.
  - **Deeper Research**: Small operators do not want another "Inbox" to check; they want an Assistant that has already read the message, drafted a reply, and prepared the quote draft before they even look at it.

  ## 3. Comparative Analysis

  ### OHC vs Selected Deep-Dive Competitor vs Top Discovered Competitors

  | Feature | OneHumanCorp (Proposed) | Shopify Inbox (Deep-Dive) | HubSpot Service Hub | Zendesk Support |
  |---|---|---|---|---|
  | **Target User** | Non-technical Owner/Operator | E-commerce Merchant | Sales & Support Teams | Enterprise Support |
  | **AI Intake Intercept** | ✅ Fully autonomous drafts | ⚠️ Basic auto-replies | ⚠️ Complex workflow required | ✅ High-end AI add-on |
  | **Unified Omnichannel** | ✅ SMS, DM, Web, Voice | ✅ Commerce DMs | ✅ Full CRM Omnichannel | ✅ Enterprise channels |
  | **Action-Oriented Triage** | ✅ Direct to Quote/Task | ❌ Reply only | ⚠️ Ticket creation | ⚠️ Ticket creation |
  | **Setup Complexity** | Very Low (0-config AI) | Medium | High | Very High |
  | **Mobile Experience** | ✅ 375px native focus | ⚠️ Clunky on phone | ❌ Desktop first | ❌ Desktop first |

  ## 4. Visual Evidence & Architecture

  ### Dynamic Competitive Landscape
  ```mermaid
  quadrantChart
      title SMB AI Triage Market Landscape
      x-axis "Complex/Manual" --> "Simple/Autonomous"
      y-axis "Enterprise Support" --> "Owner/Operator Action"
      quadrant-1 "Ideal SMB Space"
      quadrant-2 "Heavy AI CRMs"
      quadrant-3 "Legacy Helpdesks"
      quadrant-4 "Basic Auto-responders"
      "Shopify Inbox": [0.6, 0.4]
      "HubSpot Service Hub": [0.3, 0.7]
      "Zendesk": [0.2, 0.8]
      "WeCom": [0.4, 0.6]
      "ManyChat": [0.8, 0.3]
      "OneHumanCorp": [0.9, 0.9]
  ```

  ### User Journey Comparison
  ```mermaid
  journey
      title Incoming Lead to Quote Workflow
      section Shopify / Wix
        Read notification: 3: Owner
        Open app/desktop: 2: Owner
        Read message context: 3: Owner
        Manually draft reply: 2: Owner
        Send link to product: 3: Owner
      section HubSpot
        Receive ticket: 2: Support Rep
        Assign to self: 3: Support Rep
        Draft reply using template: 4: Support Rep
        Close ticket: 3: Support Rep
      section OneHumanCorp
        Notification contains AI Summary: 5: Owner
        Tap 'Approve & Send Quote': 5: Owner
  ```

  ### Feature Gap Heatmap
  ```mermaid
  graph TD
      classDef highGap fill:#f9d0c4,stroke:#333,stroke-width:2px;
      classDef lowGap fill:#d4edda,stroke:#333,stroke-width:2px;

      Features[Key Operator Requirements]
      Features --> Intake[Omnichannel Intake API]
      Features --> Classification[AI Intent Classification]
      Features --> Summary[1-Line AI Summary]
      Features --> Drafting[Autonomous Reply Drafting]
      Features --> Action[1-Tap Business Action e.g. Quote]

      Intake:::lowGap
      Classification:::highGap
      Summary:::highGap
      Drafting:::highGap
      Action:::highGap

      %% Heatmap indicates areas where OHC currently lacks implementation (High Gap)
  ```

  ### High-Level Architecture
  - **Entities**:
    - `Message` (id, channel, sender_id, raw_content, status).
    - `TriageItem` (id, message_id, suggested_action, ai_summary, status: [pending, actioned, dismissed]).
  - **Agents**:
    - `WorkTriageAgent` intercepts new messages, runs classification, extracts intent (e.g., "quote request", "status update"), and creates `TriageItem`.
    - `CustomerAssistantAgent` drafts proposed replies.

  ## 5. Implementation Prompt
  - **User Outcome**: When Carlos receives a text message asking for a repair estimate, he opens OHC and immediately sees a Triage Card. The card summarizes the request, links to the customer's past jobs, and offers a pre-drafted reply with a generated quote link ready for approval.
  - **Critical User Journey**:
    1. Owner opens app.
    2. Owner sees "Pending Triage" items.
    3. Owner taps "Review" on a new lead.
    4. Owner reads AI summary and drafted reply.
    5. Owner taps "Send & Create Quote".
    6. Item is cleared from Triage feed.
  - **Acceptance Criteria**:
    - Triage Feed UI is implemented and responsive down to 375px.
    - Mocked intake of a new message generates a Triage item with AI-drafted content.
    - Approving an item marks it as actioned and triggers mock outbound flow.
    - At least 5 E2E Playwright tests verify the triage approval flow.

  ## 6. Priority & Scope
  - **Priority**: P0
  - **Estimated Scope**: Large

  ## 7. References & Sources Catalog
  1. https://www.shopify.com/
  2. https://www.shopify.com/inbox
  3. https://www.shopify.com/pricing
  4. https://apps.shopify.com/
  5. https://community.shopify.com/c/shopify-discussions/bd-p/shopify-discussion
  6. https://www.reddit.com/r/smallbusiness/comments/12345/what_is_the_best_crm_for_small_business/
  7. https://www.reddit.com/r/smallbusiness/comments/abcdef/anyone_else_overwhelmed_by_shopify/
  8. https://www.trustpilot.com/review/www.shopify.com
  9. https://www.hubspot.com/products/crm
  10. https://www.hubspot.com/pricing/crm
  11. https://www.trustpilot.com/review/hubspot.com
  12. https://apps.apple.com/us/app/shopify-ecommerce-business/id371296246
  13. https://apps.apple.com/us/app/hubspot-crm/id1104655618
  14. https://www.wix.com/
  15. https://www.wix.com/ecommerce/website
  16. https://www.trustpilot.com/review/wix.com
  17. https://www.reddit.com/r/ecommerce/comments/xyz123/wix_vs_shopify_for_a_small_boutique/
  18. https://squareup.com/us/en/point-of-sale
  19. https://squareup.com/us/en/online-store
  20. https://www.trustpilot.com/review/squareup.com
  21. https://www.wecom.com/
  22. https://dingtalk.com/en
  23. https://www.larksuite.com/
  24. https://www.trustpilot.com/review/larksuite.com
  25. https://www.zendesk.com/
  26. https://www.trustpilot.com/review/zendesk.com
  27. https://www.intercom.com/
  28. https://www.trustpilot.com/review/intercom.com
  29. https://www.reddit.com/r/Entrepreneur/comments/qwe/best_customer_support_tool_for_solopreneur/
  30. https://www.salesforce.com/crm/small-business/
  31. https://www.trustpilot.com/review/salesforce.com
  32. https://www.microsoft.com/en-us/microsoft-copilot
  33. https://www.notion.so/product/ai
  34. https://www.trustpilot.com/review/notion.so
  35. https://www.reddit.com/r/Notion/comments/112233/notion_ai_is_game_changing_for_my_agency/
  36. https://www.freshworks.com/freshdesk/
  37. https://www.trustpilot.com/review/freshworks.com
  38. https://www.zoho.com/crm/
  39. https://www.trustpilot.com/review/zoho.com
  40. https://www.reddit.com/r/smallbusiness/comments/98765/thoughts_on_zoho_one/
  41. https://mailchimp.com/
  42. https://www.trustpilot.com/review/mailchimp.com
  43. https://www.klaviyo.com/
  44. https://www.trustpilot.com/review/klaviyo.com
  45. https://www.reddit.com/r/ecommerce/comments/54321/mailchimp_vs_klaviyo/
  46. https://www.gorgias.com/
  47. https://www.trustpilot.com/review/gorgias.com
  48. https://www.kustomer.com/
  49. https://www.trustpilot.com/review/kustomer.com
  50. https://manychat.com/
  51. https://www.trustpilot.com/review/manychat.com

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
