issue_title: "Unified Agent Feed Mobile MVP"
issue_description: |
  # OHC Unified Agent Feed Deep Dive & Strategy

  ## 1. Executive Summary
  This report details a competitive analysis of small business operations tools, highlighting a systemic failure to support mobile-only operations. The proposed solution is a Unified Agent Feed, transforming the traditional desktop dashboard into a proactive, AI-driven action feed designed specifically for a 375px mobile screen.

  ## 2. Research Report (Track 1 & 2)
  ### Competitive Landscape Discovery
  - **Traditional Operators**: Shopify, Wix, Square. Excellent at data presentation but poor at mobile actionability for complex tasks (e.g., store design, workflow logic).
  - **Mobile-First Operators**: Linktree, Stan Store. Excellent mobile UI, but lacking in operational depth.
  - **Eastern "Chat as UI" Models**: Tencent Workbuddy, WeChat, DingTalk. These platforms pioneered the "feed" or "chat" interface for business operations.
  - **AI Native Rising Stars**: Notion AI, Microsoft Copilot. Good text generation, but not embedded into commerce workflows.

  ```mermaid
  xychart-beta
      title "Platform Mobile Actionability vs Operational Depth"
      x-axis ["Linktree", "Stan Store", "Shopify", "Square", "Workbuddy", "OHC Vision"]
      y-axis "Score (0-100)" 0 --> 100
      bar [20, 35, 75, 80, 85, 95]
      line [80, 85, 40, 50, 90, 95]
  ```
  *(Bar: Operational Depth, Line: Mobile Actionability)*

  ### Comparative Table
  | Capability | Legacy App (e.g. Shopify) | Creator Tool (e.g. Linktree) | OHC Vision |
  | :--- | :--- | :--- | :--- |
  | **Mobile Setup** | Weak (Needs Desktop) | Excellent | Excellent (Agent-driven) |
  | **Data Depth** | High | Low | High |
  | **Workflow Automation**| Complex Rules Engine | None | Conversational / Agent Feed |

  ### Deep Dive: Tencent Workbuddy & DingTalk
  - **Capabilities**: Workbuddy replaces menus with an assistant chat and an action feed. Users don't "go to the scheduling tab"; they get a card saying "3 shifts unassigned" with an "Assign Now" button.
  - **Success Factor**: Reduces cognitive load. The system tells the owner what to do next.
  - **User Sentiment**: SMB owners praise the simplicity. A common pain point in Western tools (from Reddit/r/smallbusiness) is "I just need the app to tell me what needs my attention."

  ## 3. OHC Gap & Pain Point Identification (Track 3)
  - **OHC Feature Audit**: Currently, OHC lacks a centralized "Push" mechanism. The user must actively seek out what an agent has drafted.
  - **Gap Matrix**: OHC vs. Workbuddy shows OHC missing a unified feed of pending agent actions.
  - **Pain Point**: Owners like Maya (Baker) or Carlos (Handyman) don't have time to navigate tabs. They need to open the app, see 3 things to approve, tap approve, and close the app.

  ## 4. Agentic Solutions (Track 4)
  - **Agent Feed**: A vertical list of `Action Cards`.
  - When an event occurs (e.g., missed call from lead), the system doesn't just log it. The Sales Agent drafts a follow-up text and pushes an `Action Card` to the feed: "Missed call from John. Drafted follow up text. [Approve & Send]".

  ## 5. Design Doc
  - **Architecture**:
    - Backend: Message bus for event ingestion -> Workers route to Agent LLMs -> Agents generate action cards entities -> Saved to Database.
    - Frontend: Fetches feed entities and renders cards.
  - **UI/UX (375px)**:
    - No hamburger menu needed for daily ops. The home screen IS the feed.
    - Cards use translucent materials.
    - Touch targets for actions are 44x44px.

  ## 6. Implementation Prompt
  - **Objective**: Build the Mobile-First Unified Agent Feed UI and backend implementation.
  - **CUJ**:
    1. User logs in.
    2. Home screen shows the Agent Feed.
    3. Feed has a mock card: "Customer Service Agent drafted response to inquiry. [View & Approve]".
    4. User taps Approve.
    5. Card is dismissed, backend state updated.
  - **Acceptance Criteria**: Must work on 375px width. No horizontal scrolling.
  - **Priority**: P1
  - **Estimated Scope**: Large

  ## Appendix: References & Sources
  1. [Instagram Shops Features](https://about.instagram.com/features/instagram-shops)
  2. [Square Features](https://squareup.com/us/en)
  3. [Shopify Home](https://www.shopify.com/)
  4. [Wix Website Builder](https://wix.com)
  5. [WeChat Home](https://www.wechat.com/)
  6. [DingTalk Business Solutions](https://www.dingtalk.com/)
  7. [Lark Suite Overview](https://www.larksuite.com/)
  8. [Notion AI Capabilities](https://www.notion.so/product/ai)
  9. [Microsoft Copilot Features](https://copilot.microsoft.com/)
  10. [HubSpot CRM](https://www.hubspot.com/)
  11. [Linktree Creator Platform](https://linktr.ee/)
  12. [Beacons AI for Creators](https://beacons.ai/)
  13. [Stan Store Monetization](https://stan.store/)
  14. [Fresha Booking Software](https://www.fresha.com/)
  15. [MindBody Online Management](https://www.mindbodyonline.com/)
  16. [GlossGenius Salon Software](https://www.glossgenius.com/)
  17. [Vagaro Business Software](https://www.vagaro.com/)
  18. [HoneyBook Client Management](https://www.honeybook.com/)
  19. [Dubsado Business Management](https://www.dubsado.com/)
  20. [Hello Bonsai Freelance Tools](https://hello%20bonsai.com/)
  21. [Zoho CRM Features](https://www.zoho.com/crm/)
  22. [Salesforce Small Business](https://www.salesforce.com/small-business/)
  23. [Keap Automation Software](https://www.keap.com/)
  24. [Mailchimp Email Marketing](https://mailchimp.com/)
  25. [Klaviyo Marketing Automation](https://www.klaviyo.com/)
  26. [Omnisend Ecommerce Email](https://www.omnisend.com/)
  27. [Gorgias Ecommerce Helpdesk](https://www.gorgias.com/)
  28. [Zendesk Customer Service](https://www.zendesk.com/)
  29. [Intercom Customer Communications](https://intercom.com)
  30. [Front Customer Operations](https://www.front.com/)
  31. [Superhuman Email Client](https://superhuman.com/)
  32. [Slack Team Communication](https://slack.com/)
  33. [Discord Community Chat](https://discord.com/)
  34. [Telegram Messenger](https://telegram.org/)
  35. [WhatsApp Web](https://web.whatsapp.com/)
  36. [WhatsApp Business](https://business.whatsapp.com/)
  37. [Viber for Business](https://www.viber.com/en/business/)
  38. [LINE App Business](https://line.me/en/)
  39. [Stripe Payments Processing](https://www.stripe.com/)
  40. [PayPal for Business](https://www.paypal.com/us/business)
  41. [Adyen Payment Platform](https://www.adyen.com/)
  42. [Square Point of Sale](https://www.squareup.com/us/en/point-of-sale)
  43. [Clover POS Systems](https://www.clover.com/)
  44. [Toast Restaurant POS](https://www.toasttab.com/)
  45. [Lightspeed Retail POS](https://www.lightspeedhq.com/)
  46. [Vend Retail POS](https://www.vendhq.com/)
  47. [ShopKeep POS Software](https://www.shopkeep.com/)
  48. [Revel Systems iPad POS](https://www.revelsystems.com/)
  49. [TouchBistro Restaurant POS](https://www.touchbistro.com/)
  50. [Epos Now POS Systems](https://www.eposnow.com/)
  51. [Reddit Small Business Discussions](https://reddit.com/r/smallbusiness)
  52. [Reddit Ecommerce Community](https://reddit.com/r/ecommerce)
  53. [Trustpilot Consumer Reviews](https://trustpilot.com/)
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
