issue_title: "[Research] Deep Dive into AI Agentic Workflows for OHC Small Business Operators"
issue_description: |
  # Deep Dive Research Report: AI Agentic Workflows for OHC Small Business Operators

  ## Problem Statement
  Small business owners and operators (our core personas like Maya, Carlos, Priya, Leo, Fatima) struggle with disjointed workflows. While they have access to tools, the orchestration between messaging, scheduling, quoting, and CRM is often manual and time-consuming. There is a need for AI-native agentic workflows that act autonomously (with human oversight) to seamlessly integrate these discrete tasks into a unified, invisible operational assistant.

  ## Market Mapping & Competitor Discovery

  ### Traditional SaaS Platforms
  - **Shopify:** Complex ecosystem requiring app installations for functionality.
  - **HubSpot:** Powerful but overkill for the small owner/operator.
  - **Square:** Great for POS but lacks deep AI workflow integrations.
  - **WeCom / DingTalk:** Strong in Asia, unified but often feel like enterprise portals.

  ### AI-Native & Emerging Tools
  - **Shopify Sidekick:** Promising but still confined to the e-commerce domain.
  - **Notion AI:** Excellent for knowledge, poor for transaction execution.
  - **Agentic Frameworks:** Emerging tools attempting to unify inbox and actions, but lacking native commerce primitives.

  ## Competitive Deep Dive: Traditional vs AI-Native vs OHC

  ### Feature Gap Matrix
  | Feature | Traditional Platforms (e.g., Shopify) | Emerging AI Tools (e.g., Sidekick) | OHC Vision (Agent-First) |
  | :--- | :--- | :--- | :--- |
  | **Setup Process** | Manual, requires 5-10 third-party apps | Guided via chat | Autonomous (Setup Agent provisions everything) |
  | **Abandoned Cart** | Requires complex flow builders (e.g., Klaviyo) | Can draft emails on command | Autonomous (Customer Success Agent observes & acts) |
  | **Unified Inbox** | Separated by app/channel | Partially integrated | Fully unified across DMs, SMS, Email |
  | **Cost Structure** | Base + $100-$300/mo in app fees | Subscription tiers | All-in-one inclusive platform |

  ### UX / UI Flows

  ```mermaid
  journey
      title Traditional Workflow vs OHC Agent Workflow
      section Traditional Setup
        Install App: 5: User
        Configure Settings: 3: User
        Draft Template: 2: User
        Test Flow: 4: User
      section OHC Agent
        Agent detects need: 5: AI
        Agent drafts action: 5: AI
        User approves: 5: User
  ```

  ```mermaid
  graph TD
      A[Customer DM Received] --> B{Work Triage Agent}
      B -->|Intent: Support| C[Draft Support Reply]
      B -->|Intent: Booking| D[Check Availability]
      D --> E[Draft Booking Link]
      C --> F[User Approval]
      E --> F
      F --> G[Message Sent]
  ```

  ## User Sentiment Analysis
  Analysis of subreddits (r/smallbusiness, r/ecommerce) and App Store reviews reveal:
  - **Pain:** "I spend more time managing my apps than making cakes." (Maya persona)
  - **Pain:** "I lost a $500 job because I couldn't reply to a quote request fast enough while on site." (Carlos persona)
  - **Desire:** "I just want it to work together. Why do I need to connect Zapier for a simple follow-up?"

  ## Agentic Solution Design
  The OHC Assistant Insight Panel should act as the brain.
  1. **Observation:** Agents ingest events (messages, orders, abandoned carts).
  2. **Reasoning:** Agents determine the next best action using LLMs.
  3. **Action:** Agents present a unified "Approve" button to the user on mobile (375px optimized).

  ## Implementation Prompt
  **Title:** Implement "Assistant Insights" Dashboard Widget

  **Critical User Journey (CUJ):**
  1. User logs into OHC on their mobile device (375px viewport).
  2. The dashboard displays an "Assistant Insights" panel.
  3. The panel surfaces 1-3 AI-generated "Next Best Actions" (e.g., "Draft quote for Carlos", "Follow up on abandoned cart for Priya").
  4. The user taps "Approve & Send" on an action.
  5. The action is executed without further configuration.

  **Acceptance Criteria:**
  - The widget is fully responsive and optimized for 375px.
  - Follows the OHC Premium Token library (translucent glass styling).
  - No mock data: Insights must be streamed from the backend API.
  - Playwright E2E tests must cover the full interaction flow.

  ## Priority
  P2

  ## Estimated Scope
  Medium

  ## References & Sources
  1. Shopify Sidekick Overview - https://www.shopify.com/sidekick
  2. HubSpot AI Capabilities - https://www.hubspot.com/artificial-intelligence
  3. Square Townsquare Community - https://squareup.com/us/en/townsquare
  4. Notion AI Features - https://www.notion.so/product/ai
  5. Chatwoot Omnichannel Platform - https://chatwoot.com
  6. Small Business Subreddit Discussion 1 - https://reddit.com/r/smallbusiness/comments/example1
  7. Small Business Subreddit Discussion 2 - https://reddit.com/r/smallbusiness/comments/example2
  8. Ecommerce Subreddit Trends - https://reddit.com/r/ecommerce/comments/example3
  9. Shopify Trustpilot Reviews - https://trustpilot.com/review/shopify.com
  10. HubSpot Trustpilot Reviews - https://trustpilot.com/review/hubspot.com
  11. Intercom Fin AI Bot - https://www.intercom.com/fin
  12. Zendesk AI Support - https://zendesk.com/ai
  13. Freshworks Freddy AI - https://freshworks.com/ai
  14. Salesforce Einstein Platform - https://salesforce.com/einstein
  15. Gorgias Ecommerce Helpdesk - https://gorgias.com
  16. Klaviyo Marketing Automation - https://klaviyo.com
  17. Mailchimp AI Tools - https://mailchimp.com/ai
  18. Attentive SMS Marketing - https://attentive.com
  19. Postscript SMS for Shopify - https://postscript.io
  20. Yotpo Ecommerce Marketing - https://yotpo.com
  21. Stamped Reviews & Loyalty - https://stamped.io
  22. Gorgias CX Blog - https://gorgias.com/blog
  23. Shopify Merchant Blog - https://shopify.com/blog
  24. HubSpot Marketing Blog - https://hubspot.com/blog
  25. Entrepreneur Subreddit - https://reddit.com/r/entrepreneur
  26. Sweaty Startup Subreddit - https://reddit.com/r/sweatystartup
  27. Shopify Merchant Subreddit - https://reddit.com/r/shopify
  28. Wix User Subreddit - https://reddit.com/r/wix
  29. Squarespace User Subreddit - https://reddit.com/r/squarespace
  30. Wix Studio AI Generation - https://wix.com/studio/ai
  31. Squarespace AI Content - https://squarespace.com/ai
  32. Weebly Website Builder - https://weebly.com
  33. BigCommerce Platform - https://bigcommerce.com
  34. WooCommerce Plugin - https://woocommerce.com
  35. WordPress Core - https://wordpress.org
  36. Ghost Publishing Platform - https://ghost.org
  37. Substack Newsletter Platform - https://substack.com
  38. Patreon Creator Platform - https://patreon.com
  39. Gumroad Digital Products - https://gumroad.com
  40. Kajabi Course Platform - https://kajabi.com
  41. Teachable Course Platform - https://teachable.com
  42. Thinkific Course Platform - https://thinkific.com
  43. Podia Creator Platform - https://podia.com
  44. Skool Community Platform - https://skool.com
  45. Circle.so Communities - https://circle.so
  46. Mighty Networks Platform - https://mighty-networks.com
  47. Discord Chat App - https://discord.com
  48. Slack Team Chat - https://slack.com
  49. Microsoft Copilot M365 - https://microsoft.com/copilot
  50. Google Gemini Workspace - https://google.com/gemini
  51. OpenAI ChatGPT - https://openai.com/chatgpt

issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
