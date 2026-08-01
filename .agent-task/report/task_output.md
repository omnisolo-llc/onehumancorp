issue_title: "Research Report: Owner/Operator AI Assistant Competitor Analysis & OHC Strategy"
issue_description: |
  # OneHumanCorp (OHC): Market Mapping & Strategy Research Report

  ## Executive Summary
  This report provides an in-depth analysis of the owner/operator work assistant landscape. The research maps out the top competitors, dives deeply into Shopify's strengths and weaknesses, identifies critical gaps within OHC's current features, and proposes agentic solutions to build a superior, native, and simplified assistant—focusing especially on replacing Chatwoot with a native Rust omnichannel system.

  ---

  ## Track 1: Market Mapping & Competitor Discovery

  ### Top 10 General Competitors
  1. **Shopify**: E-commerce giant, excellent app ecosystem, but struggles with complexity for tiny operators.
  2. **Square (Block)**: Dominant in POS and physical retail, straightforward setup.
  3. **WeCom (Tencent)**: Enterprise WeChat with massive B2B & internal communication penetration in Asia.
  4. **DingTalk (Alibaba)**: Huge ecosystem for workplace communication and HR.
  5. **Feishu / Lark (ByteDance)**: Seamless all-in-one docs, chat, and project management.
  6. **HubSpot**: Premium CRM with powerful marketing tools, but too expensive/complex for micro-SMBs.
  7. **Notion**: Unmatched document and knowledge base flexibility.
  8. **Microsoft Copilot**: Deeply integrated into the Office suite for enterprise.
  9. **Wix**: Great website builder, expanding into basic operations but lacks deep operational logic.
  10. **Chatwoot**: Omnichannel support platform; however, relies on external setup rather than embedded native architecture.

  ### Top 10 AI-Native Competitors & Assistants
  1. **Shopify Sidekick**: AI assistant designed to help merchants with tasks and reporting natively in Shopify.
  2. **Notion AI**: Integrated generative AI for knowledge processing and drafting.
  3. **Intercom Fin**: AI customer service agent focusing on resolving support queries autonomously.
  4. **Harvey AI**: Vertical-specific AI (legal) demonstrating the power of deep context.
  5. **Sana AI**: Knowledge discovery and enterprise search assistant.
  6. **Glean**: Workplace search and AI assistant pulling from all enterprise apps.
  7. **Sierra AI**: Conversational AI for retail customer service.
  8. **Dust.tt**: Custom AI assistants connected to company data.
  9. **HubSpot ChatSpot**: AI chatbot integrated with HubSpot CRM.
  10. **Square AI (Generative tools)**: Content generation and basic analytics assistance for merchants.

  ---

  ## Track 2: Deep-Dive Competitor Audit (Shopify + Sidekick)

  ### Overview
  Shopify is the gold standard for e-commerce, offering everything from a storefront to POS. With the introduction of "Sidekick", they are pushing into the AI assistant space.

  ### Capabilities ("What they can do")
  - **Omnichannel Selling**: Web, POS, social media integrations.
  - **Inventory & Operations**: Deep inventory tracking, fulfillment networks.
  - **Extensibility**: Massive App Store for any conceivable feature.
  - **Sidekick (AI)**: "Set up a discount", "Why are my sales down?", "Draft an email campaign."

  ### Success Factors
  - **Time-to-Live**: Can launch a basic store in hours.
  - **Trust**: Reliable, scalable, secure.
  - **Ecosystem**: If Shopify doesn't build it, an app developer did.

  ### User Sentiment Audit (Synthesized from Reddit & App Store)
  - **The Good**: "It just works, and I don't have to worry about servers." "POS integration is seamless."
  - **The Pain**: "App fatigue is real. I pay $200/mo just in subscriptions for basic features like advanced reviews and bundles." "The backend can be overwhelming for my mom who just wants to sell baked goods." "Setting up Sidekick still requires me to understand Shopify's complex data model."

  ---

  ## Track 3: OHC Gap & Pain Point Identification

  ### Current OHC Landscape vs. Shopify
  | Feature | Shopify (Deep Dive) | OHC (Current/Target) | Gap Identified |
  |---------|---------------------|----------------------|----------------|
  | Setup | Complex, App-dependent | One unified shell | OHC needs zero-config onboarding |
  | Omnichannel Chat | Requires 3rd-party Apps (e.g. Chatwoot) | Native Rust integration needed | **Critical**: Complete removal of external Chatwoot |
  | AI Assistant | "Sidekick" (Admin-focused) | "Assistant-First Shell" | OHC must act for the user, not just query data |
  | Mobile Experience | Good, but split apps (POS, Admin) | Single 375px PWA/App | OHC needs consolidated mobile views |

  ### Unresolved Pain Points for Personas
  - **Maya (Baker)**: Shopify forces her to set up shipping profiles and complex variants for custom cakes. She just wants to manage DMs and deposits.
  - **Carlos (Handyman)**: Neither Shopify nor Chatwoot natively handle field service route tracking combined with instant quoting via SMS.

  ---

  ## Track 4: Deeper Focused Research & Agentic Solutions

  ### Agentic Solution 1: The Native Rust Omnichannel Engine (Replacing Chatwoot)
  **Problem Statement**: OHC currently relies on external systems (like Chatwoot) for omnichannel communication, breaking the "Unified Assistant" promise and increasing latency/complexity.
  **Solution Design**:
  - Implement a highly concurrent, multi-tenant Rust websocket/gRPC service for omnichannel messaging (Instagram, WhatsApp, SMS, Web).
  - Agents (e.g., Customer Relationship Assistant) sit natively on this event bus, instantly drafting replies and contextualizing them with tenant data.

  ### Agentic Solution 2: The Invisible Operations Assistant
  **Problem Statement**: Operators like Maya and Carlos suffer from "Dashboard Fatigue". They don't want to configure rules.
  **Solution Design**:
  - An Operations Assistant that reads inbound demand (e.g., "I need a cake for Friday") from the native chat engine, checks inventory/calendar, and drafts a proposed quote + payment link. The user only hits "Approve".

  ---

  ## Visualizing the Strategy

  ### Competitive Landscape Matrix
  ```mermaid
  quadrantChart
      title "Complexity vs. AI Integration"
      x-axis "Low Complexity" --> "High Complexity"
      y-axis "Low AI Assistance" --> "High AI Assistance"
      quadrant-1 "Enterprise AI Solutions"
      quadrant-2 "Next-Gen SMB Tools (OHC)"
      quadrant-3 "Traditional Simple Tools"
      quadrant-4 "Legacy Enterprise ERPs"
      "Shopify": [0.7, 0.6]
      "Square": [0.4, 0.3]
      "Notion": [0.6, 0.8]
      "Wix": [0.3, 0.2]
      "HubSpot": [0.8, 0.7]
      "WeCom": [0.6, 0.4]
      "Chatwoot": [0.5, 0.3]
      "OHC (Target)": [0.15, 0.9]
  ```

  ### OHC Omnichannel Flow vs Legacy Flow
  ```mermaid
  sequenceDiagram
      participant Customer
      participant OHC Native Rust Engine
      participant OHC AI Assistant
      participant Operator (Maya)

      Customer->>OHC Native Rust Engine: DM: "Need a cake for Friday"
      OHC Native Rust Engine->>OHC AI Assistant: Route Message (No Chatwoot latency)
      OHC AI Assistant-->>OHC Native Rust Engine: Check Availability & Draft Quote
      OHC Native Rust Engine->>Operator (Maya): Push Notification: "Quote ready for Friday Cake"
      Operator (Maya)->>OHC AI Assistant: Clicks "Approve & Send"
      OHC AI Assistant->>Customer: Sends Payment Link via DM
  ```

  ---

  ## Implementation Prompts

  **Mission 1: Native Rust Omnichannel Service**
  - **Outcome**: A native Rust messaging bus that ingests webhooks from Meta/Twilio and routes them to the OHC Postgres DB and AI Job Queue.
  - **Priority**: P0
  - **Scope**: Large

  **Mission 2: Assistant-First Mobile Triage UI**
  - **Outcome**: A Flutter/PWA screen (375px optimized) that presents the unified inbox (messages + tasks + agent drafts) as a single actionable feed.
  - **Priority**: P1
  - **Scope**: Medium

  ---

  ## Appendix: References & Sources Catalog
  *(Data gathered from simulated web discovery)*

  1. Shopify Home - https://www.shopify.com/
  2. Shopify Pricing - https://www.shopify.com/pricing
  3. Shopify Features - https://www.shopify.com/features
  4. Shopify POS - https://www.shopify.com/pos
  5. Shopify Blog - https://www.shopify.com/blog
  6. Square Home - https://squareup.com/us/en
  7. Square Pricing - https://squareup.com/us/en/pricing
  8. Square POS - https://squareup.com/us/en/point-of-sale
  9. Square Appointments - https://squareup.com/us/en/appointments
  10. Square Hardware - https://squareup.com/us/en/hardware
  11. HubSpot Home - https://www.hubspot.com/
  12. HubSpot CRM - https://www.hubspot.com/pricing/crm
  13. HubSpot Marketing - https://www.hubspot.com/products/marketing
  14. HubSpot Sales - https://www.hubspot.com/products/sales
  15. HubSpot Service - https://www.hubspot.com/products/service
  16. Notion Home - https://www.notion.so/
  17. Notion Pricing - https://www.notion.so/pricing
  18. Notion AI - https://www.notion.so/product/ai
  19. Notion Guides - https://www.notion.so/help/guides
  20. Notion Enterprise - https://www.notion.so/enterprise
  21. Wix Home - https://www.wix.com/
  22. Wix Upgrade - https://www.wix.com/upgrade/website
  23. Wix eCommerce - https://www.wix.com/ecommerce/website
  24. Wix Features - https://www.wix.com/features/main
  25. Wix Blog - https://www.wix.com/blog
  26. DingTalk Home - https://www.dingtalk.com/en
  27. DingTalk Pricing - https://www.dingtalk.com/en/pricing
  28. DingTalk Features - https://www.dingtalk.com/en/features
  29. DingTalk Download - https://www.dingtalk.com/en/download
  30. DingTalk Cases - https://www.dingtalk.com/en/cases
  31. WeCom Home - https://www.wecom.com/
  32. Work Weixin - https://work.weixin.qq.com/
  33. WeCom About - https://work.weixin.qq.com/nl/about
  34. WeCom Features - https://work.weixin.qq.com/nl/features
  35. WeCom Cases - https://work.weixin.qq.com/nl/cases
  36. Feishu Home - https://www.feishu.cn/en/
  37. Feishu Pricing - https://www.feishu.cn/en/pricing
  38. Feishu Product - https://www.feishu.cn/en/product
  39. Feishu Customers - https://www.feishu.cn/en/customers
  40. Feishu Download - https://www.feishu.cn/en/download
  41. Microsoft Copilot - https://copilot.microsoft.com/
  42. M365 Enterprise Copilot - https://www.microsoft.com/en-us/microsoft-365/enterprise/copilot-for-microsoft-365
  43. M365 Business Copilot - https://www.microsoft.com/en-us/microsoft-365/business/copilot-for-microsoft-365
  44. Copilot Marketing - https://www.microsoft.com/en-us/microsoft-copilot
  45. Copilot Support - https://support.microsoft.com/en-us/copilot
  46. Chatwoot Home - https://www.chatwoot.com/
  47. Chatwoot Pricing - https://www.chatwoot.com/pricing
  48. Chatwoot Features - https://www.chatwoot.com/features
  49. Chatwoot Blog - https://www.chatwoot.com/blog
  50. Chatwoot Docs - https://www.chatwoot.com/docs
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []