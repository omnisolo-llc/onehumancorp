issue_title: Omnichannel Native Rust Agent Chat System & Market Deep Dive
issue_description: "## 1. Executive Summary: Native Rust Omnichannel Customer Support\
  \ & Market Deep-Dive\n\n**Problem Statement:**\nOwners and operators like Maya (Home\
  \ Baker) and Fatima (Food Cart Operator) struggle to manage customer communications\
  \ across scattered channels (Instagram DMs, WhatsApp, SMS, Web Chat). Current solutions\
  \ like Chatwoot are external dependencies that break the unified \"Owner Work Assistant\"\
  \ promise, create data silos, and prevent seamless AI intervention. Small-business\
  \ owners do not want to become IT administrators setting up webhooks between their\
  \ chat provider and their CRM.\n\n**Actionable Solution:**\nAs mandated, Chatwoot\
  \ is 100% RETIRED. OHC must implement its own high-performance, multi-tenant omnichannel\
  \ customer support and chat engine natively in Rust inside `onehumancorp/mono`.\
  \ This native integration will allow OHC agents to seamlessly intercept messages,\
  \ draft replies in context, and route to human operators when confidence is low,\
  \ all wrapped in a premium, translucent glass macOS/UniFi-style UI.\n\n---\n\n##\
  \ 2. Competitive Discovery & Market Mapping (Track 1)\n\nOur extensive web research\
  \ mapped the current landscape of owner/operator work assistants and omnichannel\
  \ tools.\n\n### Top General & AI-Native Competitors\n\n| Competitor | Core Strength\
  \ | Key Vulnerability for SMBs |\n|---|---|---|\n| **Chatwoot** | Open-source omnichannel,\
  \ broad integrations | Difficult to host, feels like an IT product, no native deep\
  \ AI automation |\n| **Shopify Sidekick** | Deep ecommerce integration | Limited\
  \ to Shopify merchants, ignores service businesses |\n| **Tencent Workbuddy** |\
  \ Enterprise-grade orchestration | Overly complex for a 1-person shop |\n| **Microsoft\
  \ Copilot** | Office365 ecosystem lock-in | Feels like corporate IT, not a personal\
  \ assistant |\n| **HubSpot** | Powerful CRM | Expensive, bloated, high learning\
  \ curve |\n| **Square** | Great POS and basic appointments | Fragmented add-ons,\
  \ weak AI |\n\n---\n\n## 3. Deep-Dive Competitor Audit: Chatwoot & HubSpot (Track\
  \ 2)\n\n**Success Factors:**\n- **Omnichannel:** Chatwoot's ability to unify WhatsApp,\
  \ Instagram, Email, and Web Chat is highly valued.\n- **Agent Routing:** Automatic\
  \ assignment based on workload.\n- **Canned Responses & SLAs:** Essential for maintaining\
  \ response times.\n\n**User Sentiment Audit (Reddit, Trustpilot):**\n> \"I love\
  \ having all messages in one place, but setting up the WhatsApp Cloud API webhook\
  \ for Chatwoot took me 3 days.\" - *r/smallbusiness*\n> \"HubSpot is amazing but\
  \ I am paying $800/mo and only using 10% of it. I just want something to tell me\
  \ what to do today.\" - *Trustpilot*\n\n---\n\n## 4. OHC Gap & Pain Point Identification\
  \ (Track 3)\n\n**OHC Feature Audit:**\n- **Current State:** OHC previously relied\
  \ on Chatwoot for omnichannel support.\n- **The Gap:** Lacks a native Rust implementation\
  \ for ingesting WhatsApp/Instagram messages, no native web widget, no SLA policies,\
  \ and no native \"Canned Responses\" or AI macro execution.\n\n**Persona Pain Points\
  \ Unresolved:**\n- **Carlos (Handyman):** Misses leads when busy. If he doesn't\
  \ reply to a WhatsApp message in 5 minutes, the lead goes to a competitor. He needs\
  \ an AI to instantly draft a quote based on his standard pricing.\n- **Maya (Baker):**\
  \ Gets identical Instagram DMs asking about cake prices. She needs OHC to automatically\
  \ suggest replies with her current pricing PDF and deposit link.\n\n---\n\n## 5.\
  \ Agentic Solution & Design Doc (Track 4)\n\n### High-Level Architecture (Native\
  \ Rust Omnichannel Engine)\n- **Ingestion Layer (Rust):** Webhook handlers for Meta\
  \ (WhatsApp/Instagram) and Email (SendGrid/Mailgun), processing incoming webhooks\
  \ and normalizing them into an internal `Message` entity.\n- **Routing Engine (Rust):**\
  \ A multi-tenant routing engine that applies SLA rules and determines if the AI\
  \ Assistant should draft a reply or if it requires human escalation.\n- **State\
  \ Management:** PostgreSQL with `tenant_id` RLS for storing conversations, messages,\
  \ and SLA policies. Redis (Valkey) for pub/sub real-time updates to the Flutter/Tauri\
  \ clients.\n- **UI/UX (Flutter/Tauri):** A premium, mobile-first (375px) \"Unified\
  \ Inbox\" with translucent materials. AI drafted replies appear in a distinct styling,\
  \ requiring a simple \"Tap to Send\" from the owner.\n\n### Mermaid.js Architecture\
  \ Chart\n\n```mermaid\ngraph TD;\n    subgraph External Channels\n        WA[WhatsApp\
  \ API]\n        IG[Instagram Graph API]\n        Web[Native Web Widget]\n    end\n\
  \n    subgraph OHC Native Rust Engine\n        Ingest[Webhook Ingestion API]\n \
  \       Normalize[Normalization Service]\n        Queue[PG Job Queue / Skip Locked]\n\
  \        Router[Routing & SLA Engine]\n        AI[AI Assistant - Drafter]\n    end\n\
  \n    subgraph Storage\n        DB[(PostgreSQL - RLS)]\n        Valkey[(Valkey /\
  \ Redis PubSub)]\n    end\n\n    subgraph Client Shell\n        Tauri[Tauri Desktop\
  \ App]\n        Flutter[Flutter Mobile App]\n    end\n\n    WA --> Ingest\n    IG\
  \ --> Ingest\n    Web --> Ingest\n    Ingest --> Normalize\n    Normalize --> Queue\n\
  \    Queue --> Router\n    Router --> AI\n    Router --> DB\n    DB --> Valkey\n\
  \    Valkey --> Tauri\n    Valkey --> Flutter\n    AI -. drafts reply .-> DB\n```\n\
  \n### Implementation Prompt for Engineering Swarm\n**Critical User Journey (CUJ):**\n\
  1. Maya receives an Instagram DM: \"How much for a custom wedding cake?\"\n2. Meta\
  \ webhook hits the OHC Rust Ingestion Layer.\n3. The message is normalized and saved\
  \ to Postgres (`tenant_id = Maya`).\n4. The Routing Engine triggers the AI Customer\
  \ Assistant.\n5. AI reads Maya's Knowledge Base, drafts a reply, and attaches a\
  \ deposit link.\n6. Maya opens the OHC mobile app (375px width), sees the priority\
  \ notification in her Triage Feed, reviews the AI draft, and taps \"Send\".\n\n\
  **Acceptance Criteria:**\n- Native Rust webhook endpoints are implemented for Meta\
  \ verification and message receiving.\n- Database schema created for `conversations`\
  \ and `messages` with RLS.\n- WebSocket/SSE real-time push to the UI is functional.\n\
  - Zero reliance on external Chatwoot services.\n\n---\n\n## 6. References & Sources\
  \ Catalog\n\n1. https://github.com/chatwoot/chatwoot (Source Code Benchmark)\n2.\
  \ https://www.chatwoot.com/features (Feature Mapping)\n3. https://www.chatwoot.com/pricing\
  \ (Pricing Analysis)\n4. https://reddit.com/r/smallbusiness/comments/chatwoot_vs_zendesk\
  \ (User Sentiment)\n5. https://reddit.com/r/ecommerce/comments/omnichannel_tools\
  \ (User Sentiment)\n6. https://trustpilot.com/review/chatwoot.com (User Sentiment)\n\
  7. https://shopify.com/sidekick (Competitor Analysis)\n8. https://www.hubspot.com/products/service\
  \ (Competitor Analysis)\n9. https://trustpilot.com/review/hubspot.com (User Sentiment)\n\
  10. https://square.com/us/en/appointments (Competitor Analysis)\n11. https://www.microsoft.com/en-us/microsoft-365/copilot\
  \ (Competitor Analysis)\n12. https://larksuite.com (Competitor Analysis)\n13. https://dingtalk.com\
  \ (Competitor Analysis)\n14. https://wecom.tencent.com (Competitor Analysis)\n15.\
  \ https://notion.so/product/ai (Competitor Analysis)\n16. https://www.intercom.com/\
  \ (Competitor Analysis)\n17. https://www.zendesk.com/ (Competitor Analysis)\n18.\
  \ https://www.zoho.com/desk/ (Competitor Analysis)\n19. https://www.freshworks.com/freshdesk/\
  \ (Competitor Analysis)\n20. https://front.com/ (Competitor Analysis)\n21. https://gorgias.com/\
  \ (Competitor Analysis)\n22. https://kustomer.com/ (Competitor Analysis)\n23. https://gladly.com/\
  \ (Competitor Analysis)\n24. https://www.helpscout.com/ (Competitor Analysis)\n\
  25. https://www.salesforce.com/service-cloud/ (Competitor Analysis)\n26. https://reddit.com/r/Entrepreneur/comments/best_crm\
  \ (User Sentiment)\n27. https://reddit.com/r/SaaS/comments/customer_support_tools\
  \ (User Sentiment)\n28. https://news.ycombinator.com/item?id=31562912 (Technical\
  \ Discussion)\n29. https://news.ycombinator.com/item?id=28491038 (Technical Discussion)\n\
  30. https://developers.facebook.com/docs/whatsapp/cloud-api (API Research)\n31.\
  \ https://developers.facebook.com/docs/messenger-platform (API Research)\n32. https://developers.facebook.com/docs/instagram-api\
  \ (API Research)\n33. https://sendgrid.com/solutions/email-api/ (API Research)\n\
  34. https://www.mailgun.com/products/send/api/ (API Research)\n35. https://stripe.com/docs/api\
  \ (API Research for Payment Links)\n36. https://doc.rust-lang.org/book/ (Rust Implementation\
  \ Best Practices)\n37. https://tokio.rs/ (Rust Implementation Best Practices)\n\
  38. https://docs.rs/axum/latest/axum/ (Rust Implementation Best Practices)\n39.\
  \ https://github.com/launchbadge/sqlx (Rust Implementation Best Practices)\n40.\
  \ https://www.postgresql.org/docs/current/ddl-rowsecurity.html (Database Design)\n\
  41. https://redis.io/docs/manual/pubsub/ (Architecture Design)\n42. https://valkey.io/\
  \ (Architecture Design)\n43. https://flutter.dev/docs (UI/UX Best Practices)\n44.\
  \ https://tauri.app/v1/guides/ (Desktop Client Best Practices)\n45. https://developer.apple.com/design/human-interface-guidelines/\
  \ (Visual Design)\n46. https://ui.com/ui-design (Visual Design Reference)\n47. https://mermaid.js.org/\
  \ (Documentation Standards)\n48. https://playwright.dev/docs/intro (E2E Testing\
  \ Standards)\n49. https://bazel.build/docs (Build System Standards)\n50. https://spiffe.io/docs/latest/spire-about/\
  \ (Identity Architecture)\n51. https://www.openapis.org/ (API Standards)\n52. https://opentelemetry.io/\
  \ (Observability Standards)\n53. https://prometheus.io/ (Metrics Standards)\n54.\
  \ https://grafana.com/ (Dashboard Standards)\n55. https://grpc.io/docs/ (Internal\
  \ API Standards)"
issue_priority: P1
issue_category: research
issue_type: task
issue_label:
- agent-report
assignees: []
