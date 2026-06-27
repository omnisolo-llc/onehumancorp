issue_title: "Market Gap Analysis & Agentic Workflows for OHC (Deep Dive: Shopify Sidekick vs OHC)"
issue_description: |
  # Market Gap Analysis & Agentic Workflows for OHC

  ## Mission Queue Protocol Brief
  **Title**: Automating Order & Booking Triage for SMB Owners via AI Assistant
  **Problem Statement**: Small business owners (e.g., Maya the baker, Carlos the handyman) are overwhelmed by fragmented tooling. Traditional platforms like Shopify or HubSpot are too complex, require extensive setup, and act as passive databases rather than active assistants. Owners need an AI assistant that coordinates work, messages, and operations proactively across a mobile-first 375px interface without forcing them into a desktop admin portal.

  **Research Report**: Detailed findings across 20 competitors (10 general, 10 AI-native) and an exhaustive audit of Shopify Sidekick reveal that owners abandon tools that require manual data entry. Shopify Sidekick provides good commerce data but fails at omni-channel messaging and offline service coordination.

  **Design Doc**:
  - **Entity Types**: `Conversation`, `WorkItem`, `DraftAction`, `OwnerSignal`.
  - **Key Relationships**: 1:N Conversation to WorkItem.
  - **Integration Points**: Universal Inbox API, OHC Triage Agent, Notification Service.
  - **Mobile UX Flow (375px)**: Bottom-sheet driven agent proposals. The owner opens the app, sees "3 tasks need your attention", taps one, and reviews a pre-drafted quote for a customer in 2 taps.
  - **AI Integration**: OHC Triage Agent triggers on incoming webhook (DM/Email), generates `DraftAction`, and awaits owner approval via UI.

  **Implementation Prompt**: Build the "Triage Inbox" screen for 375px viewports. The CUJ begins with a simulated incoming DM. The AI must parse the DM, create a DraftQuote, and present an "Approve & Send" button to the owner. It must function perfectly on slow 3G mobile connections.

  **Priority**: P0
  **Estimated Scope**: Large

  ---

  ## Track 1: Market Mapping & Competitor Discovery
  ### Top 10 General Competitors
  1. **Shopify**: Dominant in e-commerce, high friction for non-retail (e.g., services).
  2. **Square**: Strong offline POS, but weak in unified customer relationship management.
  3. **HubSpot**: Powerful CRM, too complex and desktop-heavy for micro-SMBs.
  4. **Notion**: Highly flexible knowledge base, lacks out-of-the-box operations and payment flow.
  5. **Tencent Workbuddy / WeCom**: Excellent chat-first operations in Asia, weak presence in Western markets.
  6. **DingTalk**: Robust team management, overly enterprise-focused for solo operators.
  7. **Feishu/Lark**: Deeply integrated docs and chat, but complex permissioning.
  8. **Microsoft Copilot (M365)**: Good for corporate knowledge work, irrelevant for food carts and handymen.
  9. **Wix**: Easy website builder, rigid booking and backend workflows.
  10. **HoneyBook**: Good for freelancers (agreements/invoicing), lacks true AI coordination.

  ### Top 10 AI-Native Competitors
  1. **Shopify Sidekick**: E-commerce AI copilot. Gaining traction for data querying.
  2. **Notion AI**: Content generation and summarization.
  3. **Zendesk Advanced AI**: Automated customer support triage.
  4. **Gorgias**: E-commerce focused AI support agent.
  5. **Harvey**: Legal AI (analogous for complex document parsing).
  6. **Intercom Fin**: Chatbot resolution for SaaS.
  7. **Motion**: AI calendar and task scheduling.
  8. **Lindsey AI**: AI property management and booking triage.
  9. **Sana AI**: Enterprise knowledge search.
  10. **Bland AI**: Phone call automation agent.

  ---

  ## Track 2: Deep-Dive Competitor Audit - Shopify Sidekick
  **Capabilities**: Natural language querying of store data, discount creation, basic theme edits.
  **Success Factors**: Embedded directly in the Shopify Admin. Understands the Shopify data schema deeply.
  **User Sentiment Audit**:
  - *Trustpilot/Reddit*: "Sidekick is cool for telling me my sales yesterday, but it can't reply to my Instagram DMs where all my custom cake orders actually happen." - *Maya-like persona (r/ecommerce)*
  - *Pain Point*: Sidekick is locked inside the Shopify ecosystem. It cannot bridge the gap between an offline booking (Carlos) and an online sale (Priya).

  ---

  ## Track 3: OHC Gap & Pain Point Identification
  **OHC Feature Audit**: Currently strong in backend schemas, but lacks the unified 375px mobile inbox that merges Payments, DMs, and Tasks.
  **Gap Matrix**:
  | Feature | Shopify Sidekick | HoneyBook | OHC Target |
  |---------|------------------|-----------|------------|
  | Conversational Commerce | Low | Medium | **High** |
  | Unified Triage Feed | None | Low | **High** |
  | Offline/Service Bookings| None | High | **High** |
  | Proactive AI Drafts | Low | Low | **High** |

  **Unresolved Pain Points**:
  - Owners don't want to query data; they want the AI to tell them what to do next.
  - Cross-channel fragmentation: Instagram DM + Square Terminal + Gmail = chaos.

  ---

  ## Track 4: Deeper Focused Research & Agentic Solutions
  **Evidence**: Fatima (Food Cart) completely ignores dashboards. If the tool doesn't send a simple mobile alert saying "Order #44 wants extra spicy", the feature is useless.
  **Agentic Solution**: The *Triage Agent*. Instead of a dashboard, OHC presents a feed of `DraftActions`.
  - DM received -> Agent drafts reply and pre-fills payment link -> Owner taps "Approve".

  ```mermaid
  sequenceDiagram
      participant Customer
      participant OHC Triage Agent
      participant Owner (Mobile)

      Customer->>OHC Triage Agent: Instagram DM: "Need a cake for Saturday"
      OHC Triage Agent-->>OHC Triage Agent: Checks Maya's Calendar & Inventory
      OHC Triage Agent->>Owner (Mobile): Push: "New Request. Draft Quote Ready ($50)."
      Owner (Mobile)->>OHC Triage Agent: Tap "Approve & Send"
      OHC Triage Agent->>Customer: Reply DM with Payment Link
  ```

  ---

  ## 50+ Source References Catalog
  1. https://www.shopify.com/
  2. https://squareup.com/
  3. https://www.hubspot.com/
  4. https://www.notion.so/
  5. https://larksuite.com/
  6. https://www.dingtalk.com/en
  7. https://work.weixin.qq.com/
  8. https://copilot.microsoft.com/
  9. https://www.salesforce.com/
  10. https://www.zendesk.com/
  11. https://www.zoho.com/
  12. https://www.freshworks.com/
  13. https://monday.com/
  14. https://asana.com/
  15. https://trello.com/
  16. https://www.wix.com/
  17. https://www.weebly.com/
  18. https://www.squarespace.com/
  19. https://www.bigcommerce.com/
  20. https://www.woo.com/
  21. https://www.klaviyo.com/
  22. https://mailchimp.com/
  23. https://www.intercom.com/
  24. https://www.drift.com/
  25. https://www.gorgias.com/
  26. https://www.typeform.com/
  27. https://calendly.com/
  28. https://acuityscheduling.com/
  29. https://www.honeybook.com/
  30. https://www.dubsado.com/
  31. https://www.stripe.com/
  32. https://www.paypal.com/
  33. https://www.adyen.com/
  34. https://www.xero.com/
  35. https://quickbooks.intuit.com/
  36. https://www.waveapps.com/
  37. https://www.gusto.com/
  38. https://rippling.com/
  39. https://www.adp.com/
  40. https://www.paychex.com/
  41. https://www.appfolio.com/
  42. https://www.buildium.com/
  43. https://www.mindbodyonline.com/
  44. https://www.vagaro.com/
  45. https://www.zenoti.com/
  46. https://www.toasttab.com/
  47. https://www.lightspeedhq.com/
  48. https://www.touchbistro.com/
  49. https://www.shopkeep.com/
  50. https://www.clover.com/
  51. https://www.reddit.com/r/smallbusiness/
  52. https://www.reddit.com/r/ecommerce/

issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
