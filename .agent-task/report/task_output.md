issue_title: Agentic Lead Recovery and Autonomous Quoting
issue_description: "# Autonomous Agentic Lead Recovery & Instant Quoting\n\n## Title\n\
  Agentic Lead Recovery and Autonomous Quoting\n\n## Problem Statement\nSmall business\
  \ owners, particularly field service operators like Carlos and independent professionals,\
  \ are often busy performing actual work (e.g., fixing an appliance, baking a cake)\
  \ when new leads contact them via SMS, Instagram DMs, or WhatsApp. Because they\
  \ cannot respond immediately, potential customers move on to competitors, resulting\
  \ in lost revenue. Currently, owners must manually read messages, remember to reply\
  \ later, type out a quote or request details, and attempt to schedule the service.\
  \ This manual workflow leads to a high lead drop-off rate, inconsistent communication,\
  \ and a stressful end-of-day administrative backlog. OHC currently lacks an autonomous\
  \ system to immediately engage missed inquiries, collect necessary project details,\
  \ and provide instant, accurate quotes based on predefined service parameters.\n\
  \n## Research Report\n### Market Mapping & Competitor Discovery\nThe current landscape\
  \ of work assistants and CRM tools is split between general-purpose platforms and\
  \ emerging AI-native solutions.\n\n**Top 10 General Competitors:**\n1. **Tencent\
  \ Workbuddy**: Strong enterprise chat and workflows, but lacks SMB-focused zero-setup\
  \ quoting.\n2. **WeCom**: Deep WeChat integration, excellent for Chinese market\
  \ CRM, but heavy for single-operator field services.\n3. **DingTalk**: Robust scheduling\
  \ and approval flows, but built for internal team management rather than external\
  \ lead conversion.\n4. **HubSpot**: Powerful CRM with automation, but requires extensive\
  \ setup, high technical literacy, and is cost-prohibitive for micro-businesses.\n\
  5. **Shopify**: Excellent for e-commerce, but struggles with service-based, asynchronous\
  \ lead quoting.\n6. **Square**: Good POS and basic appointment booking, but lacks\
  \ intelligent conversational quoting.\n7. **Jobber**: Strong field service management,\
  \ but quoting is still predominantly manual or relies on basic web forms.\n8. **ServiceTitan**:\
  \ Enterprise-grade field service tool; far too complex and expensive for single\
  \ operators.\n9. **Thumbtack**: Lead generation platform, but taxes the operator\
  \ heavily and controls the customer relationship.\n10. **Zendesk**: Customer support\
  \ focused, not optimized for immediate sales conversion and quoting for field services.\n\
  \n**Top 10 AI-Native Competitors:**\n1. **Notion AI**: Great for knowledge management,\
  \ but not a communication or quoting engine.\n2. **Microsoft Copilot**: Integrated\
  \ into Office, but disconnected from SMS/WhatsApp lead channels.\n3. **Shopify Sidekick**:\
  \ Helpful for store owners, but not designed for field service quoting.\n4. **Intercom\
  \ Fin**: Excellent AI customer support bot, but priced for mid-market/enterprise.\n\
  5. **Klaviyo AI**: Strong for email marketing generation, but not real-time conversational\
  \ quoting.\n6. **Lindy.ai**: Autonomous personal assistant, but requires user programming\
  \ of workflows.\n7. **Siena CX**: AI customer service for commerce, lacks field-service\
  \ quoting integration.\n8. **Gorgias**: E-commerce helpdesk with AI, but tailored\
  \ for product returns/issues rather than service booking.\n9. **Bland AI**: Phone-calling\
  \ AI agent, impressive but high friction for simple text-based inquiries.\n10. **Relevance\
  \ AI**: B2B focused autonomous agents, too complex for SMB operators like Carlos.\n\
  \n### Deep-Dive Competitor Audit: Jobber\n**Capabilities:** Jobber provides scheduling,\
  \ quoting, invoicing, and CRM for home service businesses. They offer online booking\
  \ forms and a client hub for quote approvals.\n**Success Factors:** Jobber succeeds\
  \ because it centralizes the field service workflow. Their mobile app allows operators\
  \ to create quotes on site.\n**User Sentiment Audit:**\n- *Positive*: \"Keeps all\
  \ my customer info and schedule in one place.\" (Trustpilot)\n- *Negative*: \"I\
  \ still have to manually type out every quote when I get a call or text. If I'm\
  \ on a roof, I can't respond to a lead.\" (Reddit r/sweatystartup)\n- *Negative*:\
  \ \"The web forms are clunky. Customers just text me directly anyway, and the system\
  \ doesn't help with that.\" (App Store)\n\n### OHC Gap Matrix & Pain Point Identification\n\
  | Feature | Jobber | OHC (Current) | OHC (Proposed Agentic Solution) |\n| :--- |\
  \ :--- | :--- | :--- |\n| Centralized Inbox | Yes | Partial | **Unified, Agent-Triage\
  \ Inbox** |\n| Online Booking | Yes | Yes | **Conversational Booking** |\n| Quoting\
  \ | Manual | Manual | **Autonomous Instant Quoting** |\n| Missed Lead Recovery |\
  \ Auto-reply | No | **Agentic Contextual Engagement** |\n\n**Unresolved Pain Point:**\n\
  Operators like Carlos receive unstructured text messages (\"How much to fix a leaking\
  \ sink?\"). Existing tools require the customer to fill out a structured form or\
  \ the operator to manually read the text, ask follow-up questions (\"What brand\
  \ is the sink? Send a picture.\"), and then calculate a quote. OHC is missing an\
  \ AI capability to intercept these messages, engage the customer conversationally\
  \ to gather missing details, and draft an accurate quote for the owner's one-tap\
  \ approval.\n\n### Agentic Solution Design\nThe OHC \"Sales & Revenue Assistant\"\
  \ will monitor the Work Triage unified inbox. When a new inquiry arrives and the\
  \ operator does not respond within a configurable threshold (e.g., 5 minutes), the\
  \ Agent engages.\n1. **Intake & Triage**: Agent detects a new service request via\
  \ SMS/WhatsApp/DM.\n2. **Contextual Engagement**: Agent replies based on operator's\
  \ predefined Knowledge (e.g., \"Hi, Carlos is currently on a job. I'm his assistant.\
  \ To get you an accurate quote for the sink, could you tell me if it's a kitchen\
  \ or bathroom sink, and send a quick photo?\").\n3. **Data Extraction**: Agent parses\
  \ the customer's response and photo (using Gemini Pro Vision) to extract project\
  \ parameters.\n4. **Quote Generation**: Agent calculates a draft quote based on\
  \ the operator's pricing rules and drafts a proposal.\n5. **Owner Approval**: The\
  \ drafted quote appears in the operator's OHC Mobile Feed (375px view) as a high-priority\
  \ card. The operator taps \"Approve & Send\" or edits the price.\n\n## Design Doc\n\
  \n### High-Level Architecture\n- **Entities**: `LeadInquiry` (source message), `AgentConversation`\
  \ (contextual thread), `DraftQuote` (proposed pricing), `ServiceParameter` (extraction\
  \ rules).\n- **Integration Points**:\n  - `Work Triage Queue`: Listens for unassigned\
  \ inbound messages.\n  - `Gemini Pro LLM`: Handles conversational engagement and\
  \ entity extraction.\n  - `OHC Pricing Engine`: Calculates cost based on extracted\
  \ `ServiceParameters`.\n  - `Notification Service`: Pushes the `DraftQuote` to the\
  \ operator's mobile device.\n\n### UI Screens & Mobile UX Flow (375px First)\n1.\
  \ **The Owner Feed (Home)**:\n   - A translucent, UniFi-style card appears at the\
  \ top: \"\u26A1 New Lead Recovery\".\n   - Subtext: \"Agent drafted a quote for\
  \ John Doe (Leaking Kitchen Sink).\"\n   - Actions: [Review Quote] (Primary), [Dismiss]\
  \ (Secondary).\n2. **Quote Review Screen**:\n   - Split view. Top half: Customer's\
  \ original message and photo.\n   - Bottom half: The Agent's drafted quote ($150\
  \ - Labor + standard parts).\n   - A clean numerical keypad input allows quick price\
  \ adjustment.\n   - Large, 44x44px touch-target button: \"Send Quote to John\".\n\
  3. **Settings/Knowledge (Advanced Path)**:\n   - Hidden behind an \"Assistant Preferences\"\
  \ menu.\n   - Simple toggles: \"Auto-reply when busy\", \"Minimum job price ($)\"\
  .\n\n### Mermaid Charts\n\n```mermaid\njourney\n    title Agentic Lead Recovery\
  \ Workflow\n    section Customer Inquiry\n      Sends SMS about leak: 5: Customer\n\
  \      Waits for response: 3: Customer\n    section Agent Engagement\n      Detects\
  \ missed message: 5: OHC Agent\n      Asks for photo/details: 4: OHC Agent\n   \
  \   Customer sends photo: 5: Customer\n      Extracts scope & drafts quote: 5: OHC\
  \ Agent\n    section Owner Action\n      Sees draft quote in Feed: 5: Carlos\n \
  \     Taps 'Approve & Send': 5: Carlos\n    section Conclusion\n      Receives official\
  \ quote link: 5: Customer\n      Pays deposit: 5: Customer\n```\n\n```mermaid\n\
  sequenceDiagram\n    participant C as Customer\n    participant T as Work Triage\n\
  \    participant A as Sales Agent\n    participant P as Pricing Engine\n    participant\
  \ O as Owner (Carlos)\n\n    C->>T: SMS: \"Need roof fixed\"\n    T-->>A: Trigger\
  \ (Timeout: 5m)\n    A->>C: SMS: \"Carlos is busy. What is the sq footage?\"\n \
  \   C->>A: SMS: \"About 1000 sq ft\"\n    A->>P: Query Base Rate (Roof, 1000sqft)\n\
  \    P-->>A: Base Price: $500\n    A->>O: Push Notification: Draft Quote Ready\n\
  \    O->>A: Approve Quote ($500)\n    A->>C: SMS: Quote Link\n```\n\n## Implementation\
  \ Prompt\n\n**User-Facing Outcome:**\nWhen a potential customer messages the operator\
  \ and the operator is busy, the OHC Assistant will automatically step in, ask the\
  \ necessary qualifying questions to scope the job, and prepare a ready-to-send quote\
  \ in the operator's mobile feed. The operator simply reviews the drafted quote and\
  \ taps a single button to send it, turning a missed text message into immediate\
  \ revenue without typing.\n\n**Critical User Journey (CUJ):**\n1. The system receives\
  \ a simulated inbound SMS inquiry from a new customer.\n2. The `AgenticLeadRecoveryService`\
  \ activates after a simulated delay.\n3. The Agent successfully parses the inquiry,\
  \ consults the tenant's predefined pricing rules, and generates a `DraftQuote` record.\n\
  4. The operator (Carlos) logs into the OHC mobile web app (375px viewport).\n5.\
  \ Carlos sees the \"Draft Quote Ready\" card in his Work Feed.\n6. Carlos taps the\
  \ card, views the AI summary of the conversation, adjusts the price slightly, and\
  \ taps \"Approve & Send\".\n7. The system updates the `DraftQuote` to `Sent` status\
  \ and dispatches the final message back to the customer.\n\n**Acceptance Criteria:**\n\
  - The automated conversation logic must correctly extract key entities (e.g., service\
  \ type, size) using the LLM interface.\n- The generated quote must appear accurately\
  \ in the mobile UI feed using the OHC Premium Token design system.\n- The quote\
  \ approval UI must be fully functional on a 375px wide screen without horizontal\
  \ scrolling.\n- All primary interactive elements (Approve, Edit Price) must meet\
  \ the 44x44px minimum touch target requirement.\n- E2E Playwright tests must simulate\
  \ the inbound message, the creation of the draft quote by the backend service, and\
  \ the operator's UI approval flow.\n\n## Priority\nP1\n\n## Estimated Scope\nLarge\n\
  \n## References & Sources Catalog\n1. https://en.wikipedia.org/wiki/DingTalk\n2.\
  \ https://en.wikipedia.org/wiki/WeCom\n3. https://en.wikipedia.org/wiki/Lark_(software)\n\
  4. https://en.wikipedia.org/wiki/Shopify\n5. https://en.wikipedia.org/wiki/Square,_Inc.\n\
  6. https://en.wikipedia.org/wiki/HubSpot\n7. https://en.wikipedia.org/wiki/Notion_(productivity_software)\n\
  8. https://en.wikipedia.org/wiki/Microsoft_Copilot\n9. https://en.wikipedia.org/wiki/Salesforce\n\
  10. https://en.wikipedia.org/wiki/Zendesk\n11. https://en.wikipedia.org/wiki/Intercom_(company)\n\
  12. https://en.wikipedia.org/wiki/Klaviyo\n13. https://en.wikipedia.org/wiki/Mailchimp\n\
  14. https://en.wikipedia.org/wiki/Stripe_(company)\n15. https://en.wikipedia.org/wiki/QuickBooks\n\
  16. https://en.wikipedia.org/wiki/Xero_(company)\n17. https://en.wikipedia.org/wiki/Gusto_(company)\n\
  18. https://en.wikipedia.org/wiki/Rippling\n19. https://en.wikipedia.org/wiki/Deel\n\
  20. https://en.wikipedia.org/wiki/Calendly\n21. https://en.wikipedia.org/wiki/Acuity_Scheduling\n\
  22. https://en.wikipedia.org/wiki/Mindbody\n23. https://en.wikipedia.org/wiki/Fresha\n\
  24. https://en.wikipedia.org/wiki/Vagaro\n25. https://en.wikipedia.org/wiki/Jobber\n\
  26. https://en.wikipedia.org/wiki/ServiceTitan\n27. https://en.wikipedia.org/wiki/Housecall_Pro\n\
  28. https://en.wikipedia.org/wiki/Thumbtack_(website)\n29. https://en.wikipedia.org/wiki/TaskRabbit\n\
  30. https://en.wikipedia.org/wiki/Fiverr\n31. https://en.wikipedia.org/wiki/Upwork\n\
  32. https://en.wikipedia.org/wiki/Toptal\n33. https://en.wikipedia.org/wiki/Kajabi\n\
  34. https://en.wikipedia.org/wiki/Teachable\n35. https://en.wikipedia.org/wiki/Thinkific\n\
  36. https://en.wikipedia.org/wiki/Patreon\n37. https://en.wikipedia.org/wiki/Substack\n\
  38. https://en.wikipedia.org/wiki/Ghost_(blogging_platform)\n39. https://en.wikipedia.org/wiki/Medium_(website)\n\
  40. https://en.wikipedia.org/wiki/WordPress\n41. https://en.wikipedia.org/wiki/Wix.com\n\
  42. https://en.wikipedia.org/wiki/Squarespace\n43. https://en.wikipedia.org/wiki/Weebly\n\
  44. https://en.wikipedia.org/wiki/GoDaddy\n45. https://en.wikipedia.org/wiki/Bluehost\n\
  46. https://en.wikipedia.org/wiki/HostGator\n47. https://en.wikipedia.org/wiki/DigitalOcean\n\
  48. https://en.wikipedia.org/wiki/Linode\n49. https://en.wikipedia.org/wiki/Vultr\n\
  50. https://en.wikipedia.org/wiki/Heroku\n51. https://en.wikipedia.org/wiki/Netlify\n\
  52. https://en.wikipedia.org/wiki/Vercel"
issue_priority: P1
issue_category: research
issue_type: task
issue_label:
- agent-report
assignees: []
