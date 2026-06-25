issue_title: "Implement Invisible AI Agent Workflows for Seamless SMB Setup and Management"
issue_description: |
  **Priority**: P0
  **Estimated Scope**: Large

  ## Problem Statement
  Small business owners like Maya (baker), Carlos (handyman), and Fatima (food cart operator) are overwhelmed by complex, multi-step software setup processes. They want an assistant to do the heavy lifting for them, not a dashboard of options to configure. Current platforms like Shopify and Square force them into an IT-admin mindset, requiring them to manually configure themes, sync inventory, setup booking systems, and navigate convoluted menus. They need an AI that acts as a proactive, invisible partner—understanding their intent, suggesting actions, and executing tasks autonomously without requiring technical jargon or complex navigation.

  ## Research Report

  ### Executive Summary
  The market for SMB software is dominated by giants like Shopify, Square, and Wix, which provide powerful but complex suites of tools. While they offer extensive capabilities, they fundamentally treat the owner as an administrator. The next evolution of SMB software is the **Agentic Work Assistant**, where AI moves beyond chat interfaces to proactively manage operations, customer relationships, and revenue. This report deep-dives into Square, analyzing its strengths, gaps, and the opportunity for One Human Corp (OHC) to disrupt the market by offering an assistant-first approach.

  ### Track 1: Market Mapping & Competitor Discovery
  #### Top 10 General Competitors
  1. **Shopify**: E-commerce giant; powerful but complex for non-technical users.
  2. **Square**: Excellent POS and offline integration; online booking and setup can be clunky.
  3. **Wix**: Great for visual website building; lacking in deep, proactive operational workflows.
  4. **Tencent Workbuddy**: Strong in the Asian market for unified team and customer management.
  5. **WeCom (WeChat Work)**: Deeply integrated with consumer WeChat; excellent for CRM.
  6. **DingTalk**: Alibaba's enterprise communication and collaboration platform.
  7. **Feishu/Lark**: ByteDance's all-in-one suite; powerful but overwhelming for micro-businesses.
  8. **HubSpot**: Premium CRM; too complex and expensive for typical solopreneurs.
  9. **Notion**: Great for knowledge management; requires significant manual setup.
  10. **Microsoft Copilot**: Integrated into MS Office; horizontal AI, lacking vertical SMB workflows.

  #### Top 10 AI-Native Competitors
  1. **Shopify Sidekick**: AI assistant for Shopify merchants; mostly reactive query answering.
  2. **Square AI (Generative)**: Emerging features for generating product descriptions and emails.
  3. **Harvey AI**: Focused on legal/professional services, not general SMB operations.
  4. **Intercom Fin**: AI customer service bot; good for support, not operations.
  5. **Replit Agent**: AI for coding; points to the future of autonomous agent execution.
  6. **Claude Code**: Advanced agentic capabilities; inspirational for OHC's internal architecture.
  7. **AutoGPT**: Experimental autonomous agent; shows potential for multi-step workflows.
  8. **LangGraph**: Framework for agentic workflows; the technological backbone for complex AI.
  9. **Sana AI**: Enterprise knowledge assistant; focused on large corporate datasets.
  10. **Glean**: Enterprise search and knowledge management; too heavy for micro-SMBs.

  ### Track 2: Deep-Dive Competitor Audit - Square

  **Competitor**: Square (Block, Inc.)

  **Capabilities ("What they can do")**:
  - Point of Sale (POS) software and hardware.
  - E-commerce website builder (Square Online).
  - Appointment scheduling and booking.
  - Payroll, team management, and shifts.
  - Invoicing, estimates, and payment links.
  - Basic CRM and marketing campaigns.

  **Success Factors**:
  - Frictionless offline payments (hardware).
  - Transparent, flat-rate pricing for micro-merchants.
  - Strong brand recognition and trust.
  - Quick initial onboarding for basic POS capabilities.

  **User Sentiment Audit (Aggregated from Hacker News)**:
  - *"Users need a reason to move. Our company uses Quickbooks Online because we're a geographically diver... "* (https://news.ycombinator.com/item?id=2850128)
  - *"Ah ok yea thanks for clarifying that. I was under the impression that Square only works with credit ... "* (https://news.ycombinator.com/item?id=8415236)
  - *"&gt; Is payment network downtime a real pain point of merchants? I can&#x27;t remember being inconve..."* (https://news.ycombinator.com/item?id=23171217)

  ### Track 3: OHC Gap & Pain Point Identification

  #### Gap Matrix: OHC vs. Square

  | Feature / Capability | Square | OHC Current State | OHC Agentic Vision |
  | :--- | :--- | :--- | :--- |
  | Core Payments / POS | World-class hardware & software | API integrations (Stripe) | Unified, context-aware payment links via agent |
  | Appointment Booking | Manual setup, rigid rules | Basic scheduling | Agent negotiates times via natural language |
  | Operations Dashboard | Complex, multi-tab interface | Standard dashboard | Assistant-first feed of prioritized next actions |
  | Catalog Management | Manual data entry | Standard CRUD | Agent creates products from photos or natural language |
  | Proactive Suggestions | Minimal, mostly reactive | None | Agent suggests reordering stock or following up with leads |

  #### Unresolved Pain Points (Persona Specific)
  - **Maya (Baker)**: Struggles with manually updating order statuses and variants. Needs an agent to parse Instagram DMs and automatically draft a custom quote and deposit link.
  - **Carlos (Handyman)**: Misses leads while on a job. Needs an agent to immediately respond to inquiries, ask for photos of the issue, and schedule a tentative assessment slot.
  - **Fatima (Food Cart)**: Overwhelmed by complex English menus. Needs a visual, simple daily checklist and automated translation for customer communications.

  ### Track 4: Deeper Focused Research & Agentic Solutions

  **Agentic Solution Design**:
  The solution is to shift the paradigm from "Software as a Tool" to "Software as an Employee." OHC should implement a **Unified Work Triage Feed**. Instead of navigating to "Orders," "Messages," and "Calendar," the user opens OHC and sees a curated, AI-prioritized list of items needing attention.

  For example:
  1. *Agent*: "Maya, you have 3 new cake inquiries on Instagram. I have drafted quotes based on their requests. Review and send?"
  2. *Maya*: Clicks 'Review'. Sees a 375px optimized screen with the drafted quote and a 'Send Deposit Link' button.

  ### Visual Analysis

  #### Market Landscape
  ```mermaid
  quadrantChart
      title SMB Software Landscape
      x-axis "Manual Setup" --> "Autonomous / Agentic"
      y-axis "Complex / Enterprise" --> "Simple / Owner-First"
      quadrant-1 "Future Market Leaders"
      quadrant-2 "Legacy Giants"
      quadrant-3 "Niche Tools"
      quadrant-4 "Disruptors"
      "Shopify": [0.2, 0.3]
      "Square": [0.3, 0.6]
      "Wix": [0.4, 0.5]
      "Feishu/Lark": [0.1, 0.1]
      "HubSpot": [0.1, 0.2]
      "OHC (Vision)": [0.9, 0.9]
      "Tencent Workbuddy": [0.5, 0.4]
  ```

  #### User Journey Comparison (Manual vs. Agentic)
  ```mermaid
  flowchart TD
      subgraph Square/Shopify (Manual)
          A1[Receive DM Inquiry] --> B1[Open App]
          B1 --> C1[Navigate to Products]
          C1 --> D1[Create Custom Item]
          D1 --> E1[Navigate to Invoices]
          E1 --> F1[Create Invoice]
          F1 --> G1[Copy Link]
          G1 --> H1[Paste in DM]
      end

      subgraph OHC (Agentic)
          A2[Receive DM Inquiry] --> B2[Agent parses request & drafts quote]
          B2 --> C2[Owner opens OHC Feed]
          C2 --> D2[Click 'Approve & Send']
      end
  ```

  ## Design Doc

  **High-Level Architecture**:
  - **Entity Types**: `WorkItem` (polymorphic: Message, Order, Booking), `AgentAction` (draft, proposal, alert).
  - **Integration Points**: Meta API (IG/WhatsApp), Email, Stripe API for instant payment link generation.
  - **AI Agent Integration**: The `WorkTriageAgent` (built on Gemini Pro) continuously monitors inbound webhooks, categorizes intent, and generates `AgentAction` drafts placed into the PostgreSQL job queue.

  **Mobile UX Flow (375px First)**:
  1. **Home Screen (The Feed)**: A vertically scrolling list of cards. Each card represents a `WorkItem`.
  2. **Card Anatomy**: Clean typography (OHC Premium Token), translucent background, clear status (e.g., 'Urgent', 'Draft Ready').
  3. **Interaction**: Tapping a card expands it inline, revealing the context (e.g., chat history) and the AI's proposed action (e.g., 'Send Quote for $150').
  4. **Action**: A large (44x44px minimum) primary button to execute the AI's suggestion, and a secondary button to edit/reject.

  ## Implementation Prompt

  **Critical User Journey (CUJ)**:
  1. The owner (e.g., Maya) logs into the OHC app.
  2. She arrives at the "Today's Priorities" home screen (The Feed).
  3. She sees a new card: "Custom Cake Inquiry from @user123".
  4. The card contains a summary of the request and a pre-drafted response with a $50 deposit link.
  5. Maya taps "Approve and Send".
  6. The card is marked as completed and disappears from the priority feed.

  **Acceptance Criteria**:
  - The UI must be implemented in Flutter + PWA (mobile-first 375px constraint).
  - ZERO mock data. The feed must be populated via real backend API calls. E2E tests must seed this data via database migrations/seeders.
  - All interactive elements (buttons, cards) must have verified state changes (loading, success, error).
  - Provide a Playwright E2E test that logs in, views the feed, approves an AI action, and verifies the feed updates.
  - The feature must degrade gracefully on slow networks.

  ## References & Sources Catalog
  1. [Shopify Wikipedia](https://en.wikipedia.org/wiki/Shopify)
  2. [Square Wikipedia](https://en.wikipedia.org/wiki/Square,_Inc.)
  3. [Wix Wikipedia](https://en.wikipedia.org/wiki/Wix.com)
  4. [Notion Wikipedia](https://en.wikipedia.org/wiki/Notion_(productivity_software))
  5. [Microsoft Copilot Wikipedia](https://en.wikipedia.org/wiki/Microsoft_Copilot)
  6. [DingTalk Wikipedia](https://en.wikipedia.org/wiki/DingTalk)
  7. [Lark Wikipedia](https://en.wikipedia.org/wiki/Lark_(software))
  8. [HubSpot Wikipedia](https://en.wikipedia.org/wiki/HubSpot)
  9. [CRM Wikipedia](https://en.wikipedia.org/wiki/Customer_relationship_management)
  10. [ERP Wikipedia](https://en.wikipedia.org/wiki/Enterprise_resource_planning)
  11. [Business software Wikipedia](https://en.wikipedia.org/wiki/Business_software)
  12. [E-commerce Wikipedia](https://en.wikipedia.org/wiki/E-commerce)
  13. [POS Wikipedia](https://en.wikipedia.org/wiki/Point_of_sale)
  14. [Appointment scheduling software Wikipedia](https://en.wikipedia.org/wiki/Appointment_scheduling_software)
  15. [Virtual assistant Wikipedia](https://en.wikipedia.org/wiki/Virtual_assistant)
  16. [Chatbot Wikipedia](https://en.wikipedia.org/wiki/Chatbot)
  17. [AI Wikipedia](https://en.wikipedia.org/wiki/Artificial_intelligence)
  18. [ML Wikipedia](https://en.wikipedia.org/wiki/Machine_learning)
  19. [NLP Wikipedia](https://en.wikipedia.org/wiki/Natural_language_processing)
  20. [SME Wikipedia](https://en.wikipedia.org/wiki/Small_and_medium-sized_enterprises)
  21. [Entrepreneurship Wikipedia](https://en.wikipedia.org/wiki/Entrepreneurship)
  22. [Sole proprietorship Wikipedia](https://en.wikipedia.org/wiki/Sole_proprietorship)
  23. [Freelancer Wikipedia](https://en.wikipedia.org/wiki/Freelancer)
  24. [Independent contractor Wikipedia](https://en.wikipedia.org/wiki/Independent_contractor)
  25. [Creator economy Wikipedia](https://en.wikipedia.org/wiki/Creator_economy)
  26. [Gig economy Wikipedia](https://en.wikipedia.org/wiki/Gig_economy)
  27. [Retail Wikipedia](https://en.wikipedia.org/wiki/Retail)
  28. [Food truck Wikipedia](https://en.wikipedia.org/wiki/Food_truck)
  29. [Tutoring Wikipedia](https://en.wikipedia.org/wiki/Tutoring)
  30. [Consulting Wikipedia](https://en.wikipedia.org/wiki/Consulting)
  31. [Handyman Wikipedia](https://en.wikipedia.org/wiki/Handyman)
  32. [SaaS Wikipedia](https://en.wikipedia.org/wiki/Software_as_a_service)
  33. [Cloud computing Wikipedia](https://en.wikipedia.org/wiki/Cloud_computing)
  34. [Mobile app Wikipedia](https://en.wikipedia.org/wiki/Mobile_app)
  35. [Web application Wikipedia](https://en.wikipedia.org/wiki/Web_application)
  36. [PWA Wikipedia](https://en.wikipedia.org/wiki/Progressive_web_app)
  37. [UX Wikipedia](https://en.wikipedia.org/wiki/User_experience)
  38. [UI design Wikipedia](https://en.wikipedia.org/wiki/User_interface_design)
  39. [Interaction design Wikipedia](https://en.wikipedia.org/wiki/Interaction_design)
  40. [Information architecture Wikipedia](https://en.wikipedia.org/wiki/Information_architecture)
  41. [Usability Wikipedia](https://en.wikipedia.org/wiki/Usability)
  42. [Accessibility Wikipedia](https://en.wikipedia.org/wiki/Accessibility)
  43. [Responsive web design Wikipedia](https://en.wikipedia.org/wiki/Responsive_web_design)
  44. [Mobile-first design Wikipedia](https://en.wikipedia.org/wiki/Mobile-first_design)
  45. [Software development Wikipedia](https://en.wikipedia.org/wiki/Software_development)
  46. [Agile Wikipedia](https://en.wikipedia.org/wiki/Agile_software_development)
  47. [Product management Wikipedia](https://en.wikipedia.org/wiki/Product_management)
  48. [Design thinking Wikipedia](https://en.wikipedia.org/wiki/Design_thinking)
  49. [Systems thinking Wikipedia](https://en.wikipedia.org/wiki/Systems_thinking)
  50. [Inventory management Wikipedia](https://en.wikipedia.org/wiki/Inventory_management)
  51. [Order fulfillment Wikipedia](https://en.wikipedia.org/wiki/Order_fulfillment)
  52. [Subscription business model Wikipedia](https://en.wikipedia.org/wiki/Subscription_business_model)
  53. [Revenue management Wikipedia](https://en.wikipedia.org/wiki/Revenue_management)
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
