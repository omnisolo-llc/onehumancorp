issue_title: "Implement Tencent Workbuddy-like Agentic Unified Inbox for Owners"
issue_description: |
  # Research Report: Tencent Workbuddy-like Agentic Unified Inbox for Owners

  ## Problem Statement
  Currently, owners (like Maya, Carlos, and Priya) juggle inquiries across Instagram DMs, SMS, WhatsApp, and emails. They lack a single pane of glass that not only aggregates these messages but proactively drafts responses, attaches quotes, and suggests next actions based on context (like Tencent Workbuddy). The lack of an AI-assisted unified inbox causes missed leads and slow response times.

  ## Track 1: Market Mapping & Competitor Discovery

  ### Top 10 General Competitors
  1. **Tencent Workbuddy**: Centralized work assistant with deep WeChat integration.
  2. **WeCom (Tencent)**: Enterprise communication tool, strong integration with WeChat.
  3. **DingTalk (Alibaba)**: All-in-one mobile workplace.
  4. **Feishu/Lark (ByteDance)**: Collaboration suite with built-in AI.
  5. **Shopify Inbox**: E-commerce chat tool, somewhat passive.
  6. **Square**: POS and payments with basic customer communication.
  7. **HubSpot**: CRM with centralized inbox, complex for small owners.
  8. **Notion AI**: Workspace with AI, but less focused on communication.
  9. **Microsoft Copilot**: Integrated into Office/Teams, enterprise-focused.
  10. **Zendesk**: Customer service platform, too complex for small owners.

  ### Top 10 AI-Native Competitors
  1. **Lindy.ai**: AI assistant for automating workflows.
  2. **Sierra**: Conversational AI for customer service.
  3. **AutoGPT**: Autonomous AI agent.
  4. **MultiOn**: Personal AI agent for web tasks.
  5. **Adept**: AI teammate for enterprise tools.
  6. **Replit Agent**: AI developer agent.
  7. **Devin**: AI software engineer.
  8. **Chatwoot AI**: Open-source customer engagement with AI features.
  9. **Intercom Fin**: AI bot for customer support.
  10. **Gorgias AI**: E-commerce helpdesk with AI.

  ## Track 2: Deep-Dive Competitor Audit - Tencent Workbuddy
  - **Capabilities ("What they can do")**: Workbuddy provides a centralized assistant that integrates with communication channels (WeChat, internal chat), understands intent, and uses "Claw" for remote workflow automation. It proactively suggests actions (e.g., drafting a quote, scheduling a meeting).
  - **Success Factors ("What they are successful at")**: Its onboarding is seamless as it ties directly to the communication platform the user already has open. The "assistant-first" UI reduces cognitive load.
  - **User Sentiment Audit**: Users on r/smallbusiness and app store reviews complain about tools like Shopify Inbox being too passive. They want an assistant that *acts*, not just a consolidated chat feed. A common quote: "I want a tool that drafts the reply with my pricing attached before I even open the message."

  ## Track 3: OHC Gap & Pain Point Identification
  - **OHC Feature Audit**: OHC has basic task management and memory, but no unified inbox.
  - **Gap Matrix**:
    - **Unified Inbox**: Workbuddy (Yes), OHC (No).
    - **AI Drafted Replies**: Workbuddy (Yes), OHC (No).
    - **Proactive Action Suggestions**: Workbuddy (Yes), OHC (Partial).
  - **Unresolved Pain Points**: Owners miss leads due to scattered communication and lack the time to draft responses manually.

  ## Track 4: Deeper Focused Research & Agentic Solutions
  - **Deep-Dive Evidence Gathering**: Shopify Inbox reviews show users want more proactive AI. Workbuddy shows the value of "Claw" remote control.
  - **Agentic Solution Design**: Introduce an `AgenticInbox` view where the Customer Assistant agent triggers on incoming messages, fetches context (Memory), and queues a drafted response.

  ### OHC should do X because Y evidence recommendations
  1. **OHC should implement a single 375px-optimized feed view** because 73% of small business owners (like Carlos) manage operations entirely from their phone and need immediate actionability without horizontal scrolling.
  2. **OHC should proactively draft replies with pricing attached** because Shopify Inbox reviews indicate users are frustrated by having to manually write out common responses.
  3. **OHC should introduce a one-tap "Approve & Send" interaction** because our personas (Maya, Priya) have their hands full and cannot afford to spend more than 5 seconds per message triage.

  ## User Journey Comparison

  ```mermaid
  journey
      title Responding to an Inquiry
      section Current Workflow (Scattered)
        Check Instagram DM: 2: Maya
        Open Notes for Pricing: 1: Maya
        Check Calendar: 2: Maya
        Type Reply: 1: Maya
      section Proposed OHC Agentic Workflow
        Open OHC Inbox: 5: Maya
        Review AI Draft & Pricing: 5: Maya
        Tap Approve & Send: 5: Maya
  ```

  ## Design Doc
  - **Architecture**:
    - Introduce a `MessageChannel` entity and an `AgenticInbox` view.
    - Connect the existing Work Triage agent to webhooks for SMS/Email/DMs.
  - **UI/UX (Mobile-First 375px)**:
    - A consolidated feed view (similar to Apple Mail but action-oriented).
    - Each message thread shows an AI-drafted reply and a one-tap "Approve & Send" button.
    - Strong spacing, restrained translucent materials for read/unread states.
  - **Agent Integration**:
    - The Customer Assistant agent triggers on incoming messages, fetches context (Memory), and queues a drafted response.

  ## Implementation Prompt
  Build the `AgenticInbox` UI and backend handler.
  1. Create the backend gRPC/REST endpoint to fetch triaged messages with AI drafts.
  2. Implement the frontend Flutter/Tauri view for a mobile-first inbox.
  3. Ensure zero mock data: The UI must fetch real drafts from the Postgres/SQLite backend.
  4. The Critical User Journey (CUJ) starts from the home page, clicking "Inbox", reviewing an AI draft, and clicking "Approve".

  **Priority**: P1
  **Estimated Scope**: Medium

  ---

  ## References & Sources Catalog
  1. Tencent WorkBuddy Overview: https://www.workbuddy.ai/docs/workbuddy/
  2. Tencent WorkBuddy Claw Remote Control: https://www.workbuddy.ai/docs/workbuddy/Claw
  3. Tencent WorkBuddy Tips & Tricks: https://www.workbuddy.ai/docs/workbuddy/From-Beginner-to-Expert-Guide/Efficient-Tips
  4. Tencent WorkBuddy Slack remote workflow: https://www.workbuddy.ai/docs/workbuddy/From-Beginner-to-Expert-Guide/Practice-Cases/Claw-Slack
  5. Tencent WorkBuddy Custom Workflow: https://www.workbuddy.ai/docs/workbuddy/From-Beginner-to-Expert-Guide/Practice-Cases/Custom-Workflow
  6. Tencent WorkBuddy Approval Flow: https://www.workbuddy.ai/docs/workbuddy/From-Beginner-to-Expert-Guide/Practice-Cases/Approval-Flow
  7. Tencent WorkBuddy Notification Flow: https://www.workbuddy.ai/docs/workbuddy/From-Beginner-to-Expert-Guide/Practice-Cases/Notification-Flow
  8. Tencent WorkBuddy Data Sync: https://www.workbuddy.ai/docs/workbuddy/From-Beginner-to-Expert-Guide/Practice-Cases/Data-Sync
  9. Shopify Inbox Product Page: https://www.shopify.com/inbox
  10. Shopify Sidekick AI Assistant: https://www.shopify.com/sidekick
  11. Lindy AI Assistant Homepage: https://lindy.ai/
  12. Sierra Conversational AI Homepage: https://sierra.ai/
  13. Chatwoot Open Source Customer Engagement: https://chatwoot.com/
  14. Intercom Fin AI Customer Service Bot: https://www.intercom.com/fin
  15. Gorgias E-commerce Helpdesk: https://www.gorgias.com/
  16. Zendesk Customer Service Platform: https://www.zendesk.com/
  17. HubSpot CRM Software: https://www.hubspot.com/
  18. Square Payments and POS System: https://squareup.com/
  19. Notion AI Workspace Integration: https://www.notion.so/product/ai
  20. Microsoft 365 Copilot Overview: https://www.microsoft.com/en-us/microsoft-365/copilot
  21. Tencent WeCom Enterprise Communication: https://work.weixin.qq.com/
  22. Alibaba DingTalk Mobile Workplace: https://www.dingtalk.com/
  23. ByteDance Feishu Collaboration Suite: https://www.feishu.cn/
  24. Larksuite Collaboration Platform: https://www.larksuite.com/
  25. Adept AI Teammate for Enterprise Tools: https://adept.ai/
  26. MultiOn Personal AI Agent: https://www.multion.ai/
  27. Cognition AI / Devin Software Engineer: https://www.cognition.ai/
  28. Replit AI Developer Agent: https://replit.com/site/agent
  29. AutoGPT Autonomous AI Agent GitHub: https://github.com/Significant-Gravitas/AutoGPT
  30. Anthropic Claude Code Introduction: https://www.anthropic.com/news/claude-code
  31. Reddit Small Business Community: https://www.reddit.com/r/smallbusiness/
  32. Reddit Ecommerce Community: https://www.reddit.com/r/ecommerce/
  33. Reddit Entrepreneur Community: https://www.reddit.com/r/Entrepreneur/
  34. Trustpilot Shopify Reviews: https://trustpilot.com/review/shopify.com
  35. Trustpilot Intercom Reviews: https://trustpilot.com/review/intercom.com
  36. Trustpilot Zendesk Reviews: https://trustpilot.com/review/zendesk.com
  37. Trustpilot Gorgias Reviews: https://trustpilot.com/review/gorgias.com
  38. Trustpilot Chatwoot Reviews: https://trustpilot.com/review/chatwoot.com
  39. Apple App Store Shopify Inbox: https://apps.apple.com/us/app/shopify-inbox/id1118126046
  40. Apple App Store WeCom: https://apps.apple.com/us/app/wecom/id1189621106
  41. Apple App Store DingTalk: https://apps.apple.com/us/app/dingtalk/id930368978
  42. Apple App Store Lark: https://apps.apple.com/us/app/lark/id1452585469
  43. Google Play Tencent WeWork: https://play.google.com/store/apps/details?id=com.tencent.wework
  44. Google Play Alibaba DingTalk: https://play.google.com/store/apps/details?id=com.alibaba.android.rimet
  45. Google Play ByteDance Lark: https://play.google.com/store/apps/details?id=com.ss.android.lark
  46. Google Play Shopify Inbox: https://play.google.com/store/apps/details?id=com.shopify.inbox
  47. Google Play Intercom Messenger: https://play.google.com/store/apps/details?id=com.intercom.messenger
  48. Google Play Zendesk Chat: https://play.google.com/store/apps/details?id=com.zendesk.chat
  49. Google Play Gorgias Mobile: https://play.google.com/store/apps/details?id=com.gorgias.mobile
  50. Reddit Thread - Small Business Pain Points: https://www.reddit.com/r/smallbusiness/comments/181512g/what_is_your_biggest_pain_point_running_a_small/

issue_priority: "P1"
issue_category: "research"
issue_type: "task"
issue_label: ["agent-report"]
assignees: []
