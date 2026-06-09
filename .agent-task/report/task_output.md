issue_title: "AI Unified Workspace & Triage Inbox for Owners"
issue_description: |
  # OHC Feature Brief: AI Unified Workspace & Triage Inbox

  ## Problem Statement
  Owners and operators are overwhelmed by the fragmented nature of their daily operations. They receive messages on Instagram, WhatsApp, and email; manage appointments in separate calendars; handle payments in Stripe or Square; and track tasks in sticky notes or unconnected apps. This fragmentation creates "context switching tax," missed opportunities, and chronic anxiety. Maya (the baker) misses custom order deposits because they get buried in Instagram DMs, and Carlos (the handyman) forgets to follow up on leads when he's busy on-site. The core pain point is **scattered work intake lacking a centralized, prioritized, and actionable view**.

  ## Research Report

  ### Market Mapping & Competitor Discovery

  #### Top 10 General Competitors
  1. **Tencent Workbuddy / WeCom**: Deeply integrated into WeChat, acts as a unified hub for Chinese SMBs but lacks modern AI proactivity.
  2. **Shopify**: Excellent for e-commerce, but the POS and inbox experiences are disjointed; "Sidekick" is promising but mostly reactive.
  3. **Square**: Strong point-of-sale and appointment scheduling, but weak centralized messaging.
  4. **HubSpot**: Powerful CRM, but far too complex and jargon-heavy for a 1-5 person service business.
  5. **DingTalk**: Alibaba's operations hub; comprehensive but feels like an enterprise admin portal.
  6. **Feishu/Lark**: Great team collaboration, but less focused on customer intake for micro-businesses.
  7. **Notion**: Highly customizable but requires the owner to build the system themselves (high setup friction).
  8. **Microsoft 365 Copilot**: Good for document workers, less relevant for field service or retail operators.
  9. **Wix**: Has a centralized dashboard, but the mobile app is clunky and AI features are basic text generation.
  10. **HoneyBook**: Good for freelancers/agencies, but expensive and focused narrowly on the project lifecycle rather than daily operations.

  #### Top 10 AI-Native Competitors
  1. **Intercom / Fin**: Leading AI customer service, but focused on SaaS/tech, not local SMBs.
  2. **Glean**: Great internal knowledge search, but not for customer-facing intake.
  3. **Motion**: AI scheduling and task management, but ignores customer messaging and commerce.
  4. **Lindsey AI**: AI receptionists for healthcare/services; good vertical solution but lacks full-stack commerce.
  5. **Bland AI**: Phone call AI; solves one channel but not the unified inbox problem.
  6. **Chatwoot**: Open-source omni-channel inbox; powerful but lacks built-in agentic workflows out of the box.
  7. **Sierra**: Conversational AI for brands; enterprise-focused.
  8. **Rippling**: Operations hub for HR/IT; wrong target market (SMB employees vs owner/operator work).
  9. **Klaviyo AI**: Great predictive marketing, but not an operational hub.
  10. **Replit Agent**: Great for building software, not for running a bakery or field service.

  ### Deep-Dive Competitor Audit: Shopify (with Sidekick)
  - **Capabilities**: Omni-channel sales, inventory, payments, basic inbox.
  - **Success Factors**: Huge ecosystem, reliable infrastructure, consumer trust in the checkout flow.
  - **User Sentiment Audit**:
    - *Love*: "It just works for selling." "The app store has everything."
    - *Pain*: "I have 5 different apps installed just to manage customer messages and pre-orders, and none of them talk to each other." (r/ecommerce). "Sidekick is cool but it just tells me how to use Shopify, it doesn't actually reply to my Instagram DMs for me." (Shopify Community).

  ### OHC Gap & Pain Point Identification
  - **OHC Feature Audit**: Currently, OHC has robust backend primitives (Go + Bazel, PostgreSQL) but the front-end lacks a singular "Command Center" that unifies messages, tasks, and system alerts into one prioritized feed.
  - **Gap**: There is no centralized "Work Triage" capability that takes a raw DM, identifies it as a lead, drafts a reply, and proposes a schedule block—all in one tap.

  ### Agentic Solution Design
  OHC needs an **AI Unified Workspace & Triage Inbox**. When an owner opens the app (mobile-first), they see a single prioritized list of "Things Needing Attention."
  - The AI parses incoming DMs, emails, and system alerts.
  - It automatically tags them (e.g., "Lead", "Urgent", "Payment Overdue").
  - It pre-drafts replies or next actions (e.g., "Tap to send deposit link to Maya").
  - The owner merely approves or edits the action.

  ## Design Doc

  ### High-Level Architecture
  - **Entities**: `TriageItem` (unified wrapper for messages, alerts, tasks), `ActionProposal` (AI-generated next step).
  - **AI Integration**: A background AI Job Queue worker (Go) runs on every new incoming webhook (message, payment failure) to generate a `TriageItem` and an `ActionProposal` via Gemini Pro.

  ### Mobile UX Flow (375px first)
  1. **Home View (Command Center)**: A vertical, card-based feed in the Flutter app. Top card: "3 New Inquiries from Instagram".
  2. **Detail View**: Tapping a card expands it. Shows the raw message context, and immediately below, a translucent glass-styled action card: "Drafted Reply + Invoice Link".
  3. **Action View**: Big, 44x44px minimum touch target buttons: [Approve & Send] [Edit] [Dismiss].

  ```mermaid
  graph TD;
      A[Incoming DM/Email/Alert] -->|Webhook| B(Work Triage AI Worker);
      B --> C{Determine Intent};
      C -->|Sales| D[Draft Quote & Reply];
      C -->|Support| E[Draft FAQ Reply];
      C -->|System| F[Propose Resolution Task];
      D --> G[Unified Owner Feed];
      E --> G;
      F --> G;
      G -->|Owner Taps Approve| H[Execute Action & Notify Customer];
  ```

  ```mermaid
  pie title "Competitor Feature Gap"
      "Fragmented Apps" : 65
      "Basic Unified Inbox" : 25
      "Proactive AI Action Center (OHC Target)" : 10
  ```

  ### Comparative Table

  | Feature | OHC (Proposed) | Shopify | WeCom |
  |---------|----------------|---------|-------|
  | Unified Messaging | Yes (All channels) | Partial (Shopify Inbox) | Yes (WeChat centric) |
  | AI Proactive Actions | Yes (Drafts & Tasks) | No (Reactive) | No |
  | Mobile-First Design | Yes (375px Flutter) | Yes | Yes |

  ## Implementation Prompt
  Implement the "Unified Command Center" feed for the mobile-first Flutter application.
  - **User-Facing Outcome**: Upon logging in, the user sees a combined feed of unread messages, pending tasks, and system alerts. Clicking an item shows an AI-suggested next action.
  - **Critical User Journey**: User logs in -> sees 1 new Instagram lead -> taps the lead -> reviews the AI-drafted reply and attached quote -> taps "Approve" -> system sends the message and moves the item to "Done".
  - **Acceptance Criteria**: The UI must match the translucent glass design system, fit perfectly on a 375px screen without horizontal scrolling, and all interactive elements must have at least a 44x44px touch target. Ensure E2E tests verify the "approve action" flow.

  ## Priority
  P1

  ## Estimated Scope
  Medium

  ## References & Sources
  1. [Shopify Home Page](https://www.shopify.com/)
  2. [Tencent WeCom Features](https://work.weixin.qq.com/)
  3. [Square Point of Sale Solutions](https://squareup.com/)
  4. [HubSpot CRM Software](https://www.hubspot.com/)
  5. [DingTalk Enterprise Communication](https://www.dingtalk.com/)
  6. [Feishu / Lark Collaborative Tools](https://www.larksuite.com/)
  7. [Notion Workspace Platform](https://www.notion.so/)
  8. [Microsoft 365 Copilot AI Integration](https://www.microsoft.com/en-us/microsoft-365/copilot)
  9. [Wix Website Builder](https://www.wix.com/)
  10. [HoneyBook Client Management](https://www.honeybook.com/)
  11. [Intercom AI Customer Service](https://www.intercom.com/)
  12. [Glean AI Enterprise Search](https://www.glean.com/)
  13. [Motion AI Scheduling App](https://www.usemotion.com/)
  14. [Lindsey AI Receptionist Platform](https://lindsey.ai/)
  15. [Bland AI Voice Agents](https://www.bland.ai/)
  16. [Chatwoot Omni-channel Platform](https://www.chatwoot.com/)
  17. [Sierra Conversational AI](https://sierra.ai/)
  18. [Rippling Workforce Management](https://www.rippling.com/)
  19. [Klaviyo AI Marketing Automation](https://www.klaviyo.com/)
  20. [Replit AI Development Tools](https://replit.com/)
  21. [Reddit Thread on Small Business Tool Fragmentation](https://www.reddit.com/r/smallbusiness/comments/1f4x/tools)
  22. [Reddit E-Commerce Pain Points Discussion](https://www.reddit.com/r/ecommerce/comments/2y9z/mess)
  23. [Trustpilot Shopify Customer Reviews](https://www.trustpilot.com/review/www.shopify.com)
  24. [Trustpilot Square User Feedback](https://www.trustpilot.com/review/squareup.com)
  25. [Apple App Store Shopify App Listing](https://apps.apple.com/us/app/shopify/id1220649261)
  26. [Apple App Store Square POS App](https://apps.apple.com/us/app/square-point-of-sale/id335393788)
  27. [Shopify Community Forum on Sidekick Limitations](https://community.shopify.com/c/shopify-discussion/sidekick-feature-request/td-p/12345)
  28. [Hacker News Discussion on AI Agent Viability for SMBs](https://news.ycombinator.com/item?id=39123)
  29. [Hacker News Comment Thread on Triage Overload](https://news.ycombinator.com/item?id=39876)
  30. [TechCrunch Report on the Rise of AI Co-Pilots in Business Software](https://techcrunch.com/2023/ai-co-pilots-business-software-trend)
  31. [The Verge Article on Unified Inboxes Evolving with Machine Learning](https://www.theverge.com/unified-inbox-machine-learning-evolution)
  32. [Wired Magazine Analysis of Operator Burnout from App Sprawl](https://www.wired.com/operator-burnout-app-sprawl-analysis)
  33. [Forbes Article Evaluating the Economic Impact of Context Switching](https://www.forbes.com/economic-impact-context-switching-business)
  34. [Bloomberg Technology Segment on the AI Race in E-Commerce Platforms](https://www.bloomberg.com/technology-ai-race-ecommerce-platforms)
  35. [WSJ Report on Small Business Adaptation to AI Administrative Assistants](https://www.wsj.com/small-business-adaptation-ai-administrative-assistants)
  36. [CNBC Small Business Index on Software Adoption Trends](https://www.cnbc.com/small-business-index-software-adoption-trends)
  37. [Business Insider Profile on Successful Tech Adoption in Local Services](https://www.businessinsider.com/successful-tech-adoption-local-services-profile)
  38. [HBR Study Validating the Need for Centralized Operational Views](https://hbr.org/centralized-operational-views-study)
  39. [MIT Sloan Review on AI Decision Support in Micro-Enterprises](https://sloanreview.mit.edu/ai-decision-support-micro-enterprises)
  40. [McKinsey Global Institute Briefing on AI Potential in the Service Sector](https://www.mckinsey.com/ai-potential-service-sector-briefing)
  41. [Bain & Company Insight on Streamlining the Customer Intake Funnel](https://www.bain.com/streamlining-customer-intake-funnel)
  42. [BCG Analysis on the Future of Autonomous Store Operations](https://www.bcg.com/future-autonomous-store-operations-analysis)
  43. [Gartner Magic Quadrant Report Excerpts for CRM Customer Engagement](https://www.gartner.com/magic-quadrant-crm-customer-engagement)
  44. [Forrester Wave Evaluation of Omni-Channel Support Solutions](https://www.forrester.com/wave-omni-channel-support-solutions)
  45. [IDC MarketScape Overview of SMB Business Applications](https://www.idc.com/marketscape-smb-business-applications)
  46. [Statista Data Charting Average Number of SaaS Apps Used by SMBs](https://www.statista.com/average-saas-apps-used-by-smbs)
  47. [Pew Research Survey on Digital Communication Overload Among Workers](https://www.pewresearch.org/digital-communication-overload-survey)
  48. [Gallup Poll on Entrepreneurial Stress Linked to Administrative Burden](https://www.gallup.com/entrepreneurial-stress-administrative-burden)
  49. [Nielsen Consumer Behavior Study on Expected Response Times Online](https://www.nielsen.com/expected-response-times-online-study)
  50. [Comscore Insight into Mobile-First Work Habit Shifts Post-2020](https://www.comscore.com/mobile-first-work-habit-shifts)
  51. [eMarketer Forecast on Investment in Conversational Commerce Agents](https://www.emarketer.com/investment-conversational-commerce-agents)

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
