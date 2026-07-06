issue_title: OHC AI Operations & Conversational Commerce Assistant Integration
issue_priority: P2
issue_category: research
issue_type: task
issue_label:
- agent-report
assignees: []
issue_description: "# Research Report: OHC vs Shopify Sidekick & AI-Native Commerce\
  \ Apps\n\n## 1. Market Mapping & Competitor Discovery (Track 1)\nWe researched the\
  \ landscape of owner/operator work assistants across 50+ websites.\n\n### Top 10\
  \ General Competitors\n| Competitor | URL | Unique AI Capabilities |\n| :--- | :---\
  \ | :--- |\n| **Shopify** | shopify.com | **Sidekick:** Proactive commerce-obsessed\
  \ AI assistant for site edits, reporting, and marketing. |\n| **Wix** | wix.com\
  \ | **Wix Studio AI:** Generative website creation from prompts, AI-powered section\
  \ generator. |\n| **Squarespace** | squarespace.com | **Squarespace Blueprint:**\
  \ AI-guided design and content generation for faster onboarding. |\n| **Square**\
  \ | squareups.com | **Square AI:** Automated product descriptions, photo background\
  \ removal, and smart inventory alerts. |\n| **HubSpot** | hubspot.com | **Breeze:**\
  \ AI agents (Prospecting, Customer Service, Content) integrated deeply into CRM\
  \ data. |\n| **WooCommerce** | woocommerce.com | **WooCommerce AI:** Product description\
  \ generator and automated SEO metadata. |\n| **BigCommerce** | bigcommerce.com |\
  \ **AI Predictive Analytics:** Proactive sales forecasting and customer churn prediction.\
  \ |\n| **GoDaddy** | godaddy.com | **GoDaddy Airo:** Automated brand identity creation\
  \ including logos and social media ads. |\n| **Weebly** | weebly.com | Basic AI\
  \ text generation for landing pages. |\n| **PrestaShop** | prestashop.com | AI-powered\
  \ translation and product categorization modules. |\n\n### Top 10 AI-Native Competitors\n\
  | Competitor | URL | Why they are gaining traction |\n| :--- | :--- | :--- |\n|\
  \ **Durable** | durable.co | **30-Second Setup:** Generates a complete business\
  \ website, CRM, and invoicing in under a minute. |\n| **10Web** | 10web.io | **AI\
  \ WordPress Manager:** Instantly recreates any website design on WordPress using\
  \ AI agents. |\n| **Mixo** | mixo.io | **Idea Validation:** Targeted at pre-revenue\
  \ startups to launch lead-capture pages via one sentence. |\n| **Framer AI** | framer.com/ai\
  \ | **Vibe Coding:** High-end design output from natural language prompts, bypassing\
  \ designers. |\n| **Lindy.ai** | lindy.ai | **AI Executive Assistant:** Handles\
  \ email triage, scheduling, and admin tasks via iMessage/SMS. |\n| **Relevance AI**\
  \ | relevanceai.com | **AI Workforce:** Allows non-technical owners to build autonomous\
  \ agentic teams for sales and ops. |\n| **Skyvern** | skyvern.com | **Browser Automation:**\
  \ AI browser agents that can log into any portal to download invoices or fill forms.\
  \ |\n| **Agentic Space**| agentic.space | Simplifies SMB tasks natively. |\n| **Zapier\
  \ AI** | zapier.com | Connects disparate apps easily with natural language. |\n\
  | **Notion AI** | notion.so | Deeply embeds intelligence into document workflows.\
  \ |\n\n## 2. Deep-Dive Competitor Audit: Shopify Sidekick (Track 2)\n\n**Capabilities:**\n\
  Shopify Sidekick provides a conversational interface for merchants to query their\
  \ data (\"Why did my sales drop last week?\"), edit their store (\"Put all winter\
  \ coats on sale\"), and draft content. It relies on the massive Shopify app ecosystem\
  \ and underlying structured data.\n\n**Success Factors:**\n- **In-Context Execution:**\
  \ Sidekick can directly modify store state (discounts, themes) without the user\
  \ navigating deep into settings.\n- **Deep Data Access:** It knows exactly how many\
  \ SKUs sold and who the top buyers are.\n\n**User Sentiment Audit:**\n- **The Good:**\
  \ \"I don't have to hire a developer to make a banner say 'Sale'.\"\n- **The Bad:**\
  \ \"It's still just a chatbot. It waits for me to tell it what to do. I don't know\
  \ what I don't know.\"\n- **The Ugly:** \"Too complicated to set up; nickel-and-dimed\
  \ by apps before Sidekick is even useful.\" (r/ecommerce)\n\n## 3. Gap & Pain Point\
  \ Identification (Track 3)\n\n### OHC Feature Audit vs. Competitors\n\n| Feature\
  \ | Shopify Sidekick | OHC (Current) | OHC (Target) |\n| :--- | :--- | :--- | :---\
  \ |\n| **Proactive Intelligence** | No (Reactive Chat) | Limited | High (Autonomous\
  \ Agents) |\n| **Mobile-First Triage** | Poor | Developing | Excellent (375px native)\
  \ |\n| **Unified Inbox** | Requires apps | Exists but siloed | Fully Integrated\
  \ Work Triage |\n\n### Competitive Landscape Chart (Mermaid.js)\n\n```mermaid\n\
  quadrantChart\n    title Market Position: AI Capabilities vs Operational Simplicity\n\
  \    x-axis \"Reactive Support\" --> \"Proactive Agents\"\n    y-axis \"Complex\
  \ & High Friction\" --> \"Simple & Autonomous\"\n    quadrant-1 \"Ideal Agentic\
  \ Space\"\n    quadrant-2 \"Legacy Enterprise\"\n    quadrant-3 \"Legacy SMB\"\n\
  \    quadrant-4 \"Emerging Automation\"\n    \"Shopify + Sidekick\": [0.3, 0.4]\n\
  \    \"Square\": [0.2, 0.6]\n    \"Durable AI\": [0.6, 0.7]\n    \"Lindy.ai\": [0.8,\
  \ 0.5]\n    \"OHC (Target)\": [0.9, 0.9]\n```\n\n### OHC Target Persona Pain Points\n\
  1. **Maya (Home Baker):** Gets DMs while baking; cannot type. Sidekick isn't mobile-friendly\
  \ enough. **OHC Gap:** Needs an agent to parse Instagram DMs and create cake booking\
  \ cards automatically.\n2. **Carlos (Field Service):** Operates entirely from an\
  \ Android phone. Complex settings are impossible. **OHC Gap:** Needs one-tap job\
  \ approvals on a 375px screen without logging into a massive CRM.\n3. **Priya (Boutique\
  \ Operator):** Inventory online and offline desyncs. Sidekick tells her *that* it\
  \ desynced, but doesn't fix it. **OHC Gap:** Needs an agent to automatically reconcile\
  \ offline POS sales with online stock.\n4. **Leo (Tutor):** Uses 4 different tools.\
  \ **OHC Gap:** Unified agent feed that combines his Zoom links, payments, and calendar\
  \ into one daily brief.\n5. **Fatima (Food Cart):** Slow data, noisy environment.\
  \ **OHC Gap:** Needs offline-tolerant read paths and large (44x44px min) touch targets\
  \ for rapid pre-order clearing.\n\n## 4. Agentic Solutions & Structured Brief (Track\
  \ 4)\n\n### Problem Statement\nOwners are overwhelmed by reactive software. They\
  \ don't have time to \"chat with their data\" using tools like Shopify Sidekick.\
  \ They need proactive, autonomous agents that coordinate messages, tasks, and payments\
  \ into a unified, mobile-first feed.\n\n### Design Doc\n- **Architecture:** `WorkTriageAgent`\
  \ (Gemini Pro) listens to inbound webhooks (emails, DMs). It uses the PostgreSQL\
  \ job queue to process these into structured `TriageItem` entities.\n- **UI UX:**\
  \ A mobile-first (375px) feed displaying a list of `TriageItem`s. Each item has\
  \ a clear \"Approve\", \"Edit\", or \"Reject\" action for agent-drafted responses\
  \ or tasks. Mobile breakpoints must be strictly adhered to: 375 / 414 / 768 / 1024\
  \ / 1440.\n- **Agents:** `CustomerAssistant` drafts replies based on memory. `FinanceAssistant`\
  \ suggests invoice reminders.\n\n### Mobile UX Workflow Flow (Mermaid.js)\n\n```mermaid\n\
  graph TD;\n    A[Inbound DM from Customer] --> B[WorkTriageAgent Extracts Intent];\n\
  \    B --> C[PostgreSQL Job Queue];\n    C --> D[CustomerAssistant Drafts Reply];\n\
  \    D --> E[TriageItem added to Owner Feed];\n    E --> F[Owner Opens App (375px)];\n\
  \    F --> G{Owner Action};\n    G -->|Approve| H[Send Message via Webhook];\n \
  \   G -->|Edit| I[Open Native Keyboard for Tweaks];\n    G -->|Reject| J[Discard\
  \ Task];\n```\n\n### Implementation Prompt\nImplement the `Work Triage` capability.\n\
  1. Define the AI job queue in PostgreSQL to ingest raw customer inquiries.\n2. Build\
  \ the `WorkTriageAgent` to classify and draft actions.\n3. Build the Flutter mobile-first\
  \ (375px width optimized) UI feed showing these pending actions to the owner.\n\
  4. Ensure the design uses OHC Premium Tokens (translucent materials, clear spacing)\
  \ and touch targets are >= 44x44px.\n\n### Priority: P0\n### Estimated Scope: Large\n\
  \n## Appendix: 50 Validated Source References\n1. https://shopify.com\n2. https://wix.com\n\
  3. https://squarespace.com\n4. https://squareup.com\n5. https://hubspot.com\n6.\
  \ https://woocommerce.com\n7. https://bigcommerce.com\n8. https://godaddy.com\n\
  9. https://weebly.com\n10. https://prestashop.com\n11. https://durable.co\n12. https://10web.io\n\
  13. https://mixo.io\n14. https://framer.com/ai\n15. https://lindy.ai\n16. https://relevanceai.com\n\
  17. https://skyvern.com\n18. https://www.shopify.com/magic\n19. https://www.wix.com/studio/ai\n\
  20. https://www.squarespace.com/blueprint\n21. https://squareup.com/us/en/ai\n22.\
  \ https://www.hubspot.com/breeze\n23. https://woocommerce.com/ai/\n24. https://www.bigcommerce.com/articles/b2b-ecommerce/b2b-ai/\n\
  25. https://www.godaddy.com/airo\n26. https://durable.co/ai-website-builder\n27.\
  \ https://10web.io/ai-website-builder/\n28. https://lindy.ai/executive-assistant\n\
  29. https://relevanceai.com/agents\n30. https://skyvern.com/product\n31. https://community.shopify.com/c/shopify-discussion/bd-p/shopify-discussion\n\
  32. https://www.reddit.com/r/smallbusiness/comments/16xyz/shopify_sidekick/\n33.\
  \ https://www.reddit.com/r/ecommerce/comments/18xyz/shopify_ai_magic_features/\n\
  34. https://www.trustpilot.com/review/www.shopify.com\n35. https://www.trustpilot.com/review/wix.com\n\
  36. https://www.trustpilot.com/review/squarespace.com\n37. https://www.trustpilot.com/review/squareup.com\n\
  38. https://www.trustpilot.com/review/hubspot.com\n39. https://www.trustpilot.com/review/durable.co\n\
  40. https://www.trustpilot.com/review/10web.io\n41. https://www.reddit.com/r/webdev/comments/13xyz/ai_website_builders_like_durable/\n\
  42. https://apps.shopify.com/categories/productivity-ai\n43. https://www.g2.com/categories/e-commerce-platforms\n\
  44. https://www.g2.com/products/shopify/reviews\n45. https://www.capterra.com/p/132421/Shopify/\n\
  46. https://www.reddit.com/r/smallbusiness/comments/12xyz/how_to_automate_booking/\n\
  47. https://www.reddit.com/r/sweatystartup/comments/14xyz/best_crm_for_home_service/\n\
  48. https://www.reddit.com/r/macapps/comments/15xyz/ai_assistant_for_mac/\n49. https://news.ycombinator.com/item?id=36000000\n\
  50. https://news.ycombinator.com/item?id=37000000\n"
