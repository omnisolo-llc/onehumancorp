issue_title: Implement Native Rust Omnichannel Chat & Agentic Triage (Chatwoot Replacement)
issue_description: "# OHC Owner Work Assistant: Competitive Deep-Dive & Omnichannel\
  \ Chat Replacement\n\n## Problem Statement\nSmall business owners\u2014from bakers\
  \ like Maya to field operators like Carlos\u2014are managing fragmented communication,\
  \ orders, and context across Instagram, WhatsApp, email, and legacy platforms like\
  \ Shopify or Zendesk. Traditional tools act as \"dumb inboxes\" that force owners\
  \ to act as manual routers. Meanwhile, Chatwoot, which previously handled omnichannel,\
  \ has been retired as an external dependency, leaving a critical need for a high-performance,\
  \ native Rust omnichannel chat system within OHC. The gap isn't just about speed;\
  \ it's about turning passive communication into agentic, proactive resolutions directly\
  \ on a mobile device without overwhelming the owner with tabs or complex interfaces.\n\
  \n## Research Report\n### Methodology\nConducted an extensive review of 51 industry\
  \ leading urls (including Shopify, Square, Stripe, Zendesk, Salesforce, Intercom,\
  \ Apple Business Chat, WeChat/WeCom, and more) focusing on how AI assistants are\
  \ augmenting or replacing traditional CRM and support inboxes. I also performed\
  \ a source code audit of Chatwoot (`https://github.com/chatwoot/chatwoot`) to map\
  \ its database schema (`conversations`, `messages`, `inboxes`) for our Rust replication\
  \ effort.\n\n### Competitive Deep Dive: Intercom Fin & Shopify Sidekick vs OHC\n\
  - **Intercom Fin / Zendesk AI:** \n  - *Success Factors:* Highly accurate RAG-based\
  \ bot resolutions for enterprise, strong API ecosystem.\n  - *Gaps for SMBs:* Requires\
  \ extensive setup, help center writing, and high cost. It operates as a customer-facing\
  \ bot rather than an internal \"co-pilot/ambassador\" for the owner. It fails the\
  \ \"Carlos\" test (a mobile-only handyman won't configure Intercom flows).\n- **Shopify\
  \ Sidekick / Wix AI:** \n  - *Success Factors:* Deeply integrated into the commerce\
  \ catalog. Great at answering \"how do I change my shipping policy.\"\n  - *Gaps\
  \ for SMBs:* Operates in a silo. Can't text a customer back via WhatsApp about a\
  \ custom order deposit in context of their past appointments.\n\n\n### Comparative\
  \ Table: OHC vs Competitors\n\n| Feature / Platform | OHC (Native Rust) | Intercom\
  \ Fin | Shopify Sidekick | Legacy Platforms (Chatwoot/Zendesk) |\n|---|---|---|---|---|\n\
  | **Primary User** | Solo/SMB Owner on Mobile | Enterprise Support Teams | E-commerce\
  \ Operators | Support Agents |\n| **Omnichannel Intake** | Yes (Insta, WA, Email)\
  \ | Yes | Limited | Yes |\n| **Agentic Triage** | Proactive Drafts for 1-Tap Approval\
  \ | Conversational Bot (Customer Facing) | Conversational Bot (Internal) | Manual\
  \ Rules/Macros |\n| **Commerce Native** | Yes (Direct Payment Links & Booking) |\
  \ Via complex API integrations | Yes (Product Catalog only) | Via API integrations\
  \ |\n| **Mobile First UX** | Yes (375px native triage feed) | No (Complex dashboards)\
  \ | No (Web centric) | No (Complex ticketing views) |\n| **Architecture** | Native\
  \ Rust + SQLite/PgHQ | SaaS Heavyweight | SaaS Embedded | Ruby/Rails (Legacy Chatwoot)\
  \ |\n\n### OHC Gap Analysis\n- **Current State:** OHC lacks the native omnichannel\
  \ connectivity previously provided by Chatwoot.\n- **Unresolved Pain Point:** Owners\
  \ must manually switch context between payment links, CRM notes, and DM apps. They\
  \ need an AI (\"The Ambassador\") that proactively reads incoming messages, checks\
  \ the CRM and inventory, and drafts a complete reply (with payment links or booking\
  \ availability) for 1-tap approval in a unified 375px mobile feed.\n\n## Design\
  \ Doc: Native Rust Omnichannel & Agentic Triage\n### High-Level Architecture\nWe\
  \ will replace Chatwoot with a lightweight, multi-tenant Rust engine (`onehumancorp/mono`)\
  \ tightly coupled with the OHC LLM Agent framework.\n\n```mermaid\ngraph TD\n  \
  \  A[Instagram DM/WhatsApp] -->|Webhook| B(Rust Omnichannel Ingress)\n    C[Email]\
  \ -->|Webhook| B\n    B --> D[(PostgreSQL: Conversations & Messages)]\n    B -->\
  \ E[Event Bus / Queue]\n    E --> F[The Ambassador Agent LLM]\n    F -->|Query Context|\
  \ G[(Tenant Memory & Commerce)]\n    F -->|Drafts Response| H[Action Required Feed]\n\
  \    H --> I[Mobile App 375px]\n    I -->|1-Tap Approve| J(Rust Egress Dispatcher)\n\
  \    J --> A\n```\n\n### Mobile UX Flow (375px First)\n1. **Push Notification:**\
  \ \"Carlos, a new lead asked about roof repair.\"\n2. **Home Screen Triage:** The\
  \ top card shows the incoming text. Below it, a translucent glass-styled section\
  \ labeled \"Drafted by The Ambassador\".\n3. **The Draft:** \"Hi Sarah, I can inspect\
  \ your roof tomorrow at 2 PM. My estimate fee is $50. Here is a link to book and\
  \ pay the deposit: [Link].\"\n4. **Action Buttons:** Large (44x44px minimum) touch\
  \ targets for `[ Approve & Send ]`, `[ Edit ]`, `[ Dismiss ]`.\n\n## Implementation\
  \ Prompt\n**Mission:** Build the Core Native Rust Omnichannel Ingestion & Triage\
  \ UI.\n**Critical User Journey (CUJ):** \n1. As Maya, I receive an Instagram DM\
  \ inquiring about a custom cake. \n2. The Rust backend ingests this, creates a unified\
  \ `Conversation` and `Message` record.\n3. The Ambassador agent detects the intent,\
  \ checks past orders, and creates a \"Drafted Reply\" record.\n4. Maya opens the\
  \ Flutter PWA on her phone, sees the \"Drafted Reply\" in her unified feed, and\
  \ taps \"Approve\" to dispatch the message.\n**Acceptance Criteria:**\n- Implement\
  \ the `Conversation` and `Message` tables in PostgreSQL with Row Level Security\
  \ (RLS) by `tenant_id`.\n- Build the Rust gRPC/REST endpoints for message ingestion.\n\
  - Build the 375px optimized Flutter UI card displaying the drafted reply with \"\
  Approve\" and \"Edit\" buttons (NO mock data; driven by real backend state).\n-\
  \ E2E Playwright test must prove a simulated webhook ingestion results in a clickable\
  \ \"Approve\" button in the UI.\n\n## References & Sources Catalog\n1. https://www.shopify.com/inbox\n\
  2. https://www.shopify.com/magic\n3. https://help.shopify.com/en/manual/shopify-magic/sidekick\n\
  4. https://www.wecom.qq.com/\n5. https://squareup.com/us/en/software/messages\n\
  6. https://squareup.com/us/en/appointments\n7. https://www.hubspot.com/products/sales/sales-assistant\n\
  8. https://www.hubspot.com/artificial-intelligence\n9. https://www.notion.so/product/ai\n\
  10. https://copilot.microsoft.com/\n11. https://www.larksuite.com/\n12. https://dingtalk.com/en\n\
  13. https://chatwoot.com/\n14. https://github.com/chatwoot/chatwoot\n15. https://www.zendesk.com/service/messaging/\n\
  16. https://intercom.com/fin-ai-copilot\n17. https://intercom.com/ai-bot\n18. https://www.gorgias.com/\n\
  19. https://www.gorgias.com/features/ai-agent\n20. https://www.klaviyo.com/features/ai\n\
  21. https://www.salesforce.com/agentforce/\n22. https://www.salesforce.com/products/service-cloud/overview/\n\
  23. https://www.zoho.com/zia/\n24. https://www.zoho.com/desk/\n25. https://www.freshworks.com/freshdesk/\n\
  26. https://www.freshworks.com/freddy-ai/\n27. https://support.apple.com/business-chat\n\
  28. https://business.whatsapp.com/\n29. https://about.meta.com/technologies/ai/\n\
  30. https://www.intuit.com/intuit-assist/\n31. https://quickbooks.intuit.com/global/\n\
  32. https://www.xero.com/us/accounting-software/ai/\n33. https://stripe.com/docs/stripe-apps\n\
  34. https://stripe.com/use-cases/saas\n35. https://www.paypal.com/us/business\n\
  36. https://squareup.com/us/en/hardware/terminal\n37. https://www.clover.com/\n\
  38. https://www.lightspeedhq.com/\n39. https://www.toasttab.com/\n40. https://www.mindbodyonline.com/\n\
  41. https://www.vagaro.com/\n42. https://www.honeybook.com/\n43. https://www.dubsado.com/\n\
  44. https://www.jobber.com/\n45. https://www.housecallpro.com/\n46. https://www.servicetitan.com/\n\
  47. https://www.buildertrend.com/\n48. https://www.procore.com/\n49. https://monday.com/\n\
  50. https://asana.com/\n51. https://trello.com/\n52. https://clickup.com/\n\n##\
  \ Estimated Scope\nMedium\n\n## Priority\nP1\n"
issue_priority: P1
issue_category: research
issue_type: task
issue_label:
- agent-report
assignees: []
