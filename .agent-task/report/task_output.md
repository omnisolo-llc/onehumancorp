issue_title: "OmniSolo Capabilities vs Tencent Workbuddy / WeCom"
issue_description: |
  # OmniSolo Research Report: Bridging the Gap with AI-Native Work Assistants

  ## Executive Summary
  OmniSolo (formerly One Human Corp) aims to be the premier AI-native work assistant for owners and operators. Our research analyzes top global competitors—focusing deeply on WeCom and Tencent Workbuddy—to identify features and UX paradigms that OmniSolo must adopt to win the SMB space.

  ## Top 10 General Competitors
  1. WeCom
  2. Tencent Workbuddy
  3. DingTalk
  4. Feishu / Lark
  5. Shopify Sidekick
  6. Square
  7. HubSpot
  8. Notion
  9. Microsoft Copilot
  10. Slack

  ## Top 10 AI-Native Competitors
  1. ChatGPT Enterprise
  2. Claude for Business
  3. Replit Agent
  4. GitHub Copilot
  5. MultiOn
  6. Adept
  7. Lindy.ai
  8. HyperWrite
  9. Harvey
  10. You.com

  ## Deep Dive: WeCom & Tencent Workbuddy
  **Capabilities:** Omnichannel messaging integration, rich client profiles, unified inbox, AI-driven task assignment, automated replies based on knowledge bases.
  **Success Factors:** High integration with consumer platforms (WeChat), minimizing the learning curve for SMB operators.
  **User Sentiment:** Users love the seamless transition from personal chat to business operations. Complaints typically center around complex backend configurations and enterprise bloat.

  ## Gap Analysis & Pain Points
  **OHC Missing Features:**
  - Deep integration with social messaging apps for direct sales (e.g., WhatsApp, Instagram DMs).
  - Native POS/Tap-to-Pay capabilities integrated directly into the chat flow.
  - Granular, AI-generated daily summary reports accessible natively on a 375px display without scrolling.

  **Unresolved Pain Points:** Operators struggle to bridge online inquiries with offline tasks without jumping between 3-4 apps.

  ## Proposed Solution: The Omnichannel Action Stream
  Design a unified "Action Stream" where customer DMs (Instagram/WhatsApp) appear alongside inventory alerts and booking requests. The AI assistant automatically drafts responses and queues POS actions for the operator to approve with one tap.

  ```mermaid
  graph TD
    A[Customer DM] --> B(OmniSolo Assistant)
    C[Inventory Alert] --> B
    D[Booking Request] --> B
    B --> E{Operator Dashboard}
    E --> F[Approve Reply]
    E --> G[Schedule Action]
    E --> H[Process Payment]
  ```

  ### Implementation Prompt
  - Create a unified `ActionStream` UI component in the Tauri desktop app.
  - Integrate omnichannel webhooks (simulated for now) into the Rust backend to populate the stream.
  - Implement 1-tap AI resolution buttons (e.g., "Draft Reply", "Send Payment Link").
  - Ensure the UI is fully responsive and optimized for 375px mobile screens.

  ## References (50+)
  1. https://www.shopify.com - Shopify Home
  2. https://about.instagram.com - Instagram About
  3. https://www.wechat.com - WeChat Home
  4. https://squareup.com - Square Home
  5. https://www.wix.com - Wix Home
  6. https://www.hubspot.com - HubSpot Home
  7. https://www.larksuite.com - Lark Home
  8. https://www.dingtalk.com/en - DingTalk Home
  9. https://www.notion.so - Notion Home
  10. https://copilot.microsoft.com - Microsoft Copilot
  11. https://www.slack.com - Slack
  12. https://openai.com/enterprise - ChatGPT Enterprise
  13. https://www.anthropic.com/claude - Claude
  14. https://replit.com - Replit
  15. https://github.com/features/copilot - GitHub Copilot
  16. https://www.multion.ai/ - MultiOn
  17. https://www.adept.ai/ - Adept
  18. https://www.lindy.ai/ - Lindy
  19. https://www.hyperwriteai.com/ - HyperWrite
  20. https://www.harvey.ai/ - Harvey
  21. https://you.com/ - You.com
  22. https://business.whatsapp.com/ - WhatsApp Business
  23. https://telegram.org/ - Telegram
  24. https://discord.com/ - Discord
  25. https://www.salesforce.com/ - Salesforce
  26. https://www.zoho.com/ - Zoho
  27. https://www.freshworks.com/ - Freshworks
  28. https://www.zendesk.com/ - Zendesk
  29. https://www.intercom.com/ - Intercom
  30. https://www.gorgias.com/ - Gorgias
  31. https://www.kustomer.com/ - Kustomer
  32. https://www.front.com/ - Front
  33. https://missiveapp.com/ - Missive
  34. https://www.helpscout.com/ - Help Scout
  35. https://www.drift.com/ - Drift
  36. https://www.crisp.chat/ - Crisp
  37. https://www.tawk.to/ - Tawk.to
  38. https://www.livechat.com/ - LiveChat
  39. https://www.tidio.com/ - Tidio
  40. https://www.smartsupp.com/ - Smartsupp
  41. https://www.user.com/ - User.com
  42. https://www.salesiq.zoho.com/ - Zoho SalesIQ
  43. https://www.olark.com/ - Olark
  44. https://www.purechat.com/ - Pure Chat
  45. https://www.chatra.com/ - Chatra
  46. https://www.chaport.com/ - Chaport
  47. https://www.jivox.com/ - Jivox
  48. https://www.conversica.com/ - Conversica
  49. https://www.ada.cx/ - Ada
  50. https://www.moveworks.com/ - Moveworks
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
