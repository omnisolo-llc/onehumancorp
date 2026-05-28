issue_title: '[Architecture] AI-Driven Predictive Inventory & Zero-Touch Restocking
  Engine'
issue_description: "# [Architecture] AI-Driven Predictive Inventory & Zero-Touch Restocking\
  \ Engine\n\n## Problem Statement\nFor small business owners who sell physical products\u2014\
  like Priya (boutique owner) or Fatima (food cart operator)\u2014inventory management\
  \ is a constant source of anxiety. Priya has to manually count stock and toggle\
  \ \"sold out\" on her website when an item sells out in-store, often forgetting\
  \ and overselling. Fatima needs to know exactly how much meat and vegetables to\
  \ prep based on upcoming weather, local events, and past sales, but she relies entirely\
  \ on guesswork. When inventory is managed manually, it leads to stockouts (lost\
  \ revenue) or overstocking (wasted capital). These solopreneurs need an invisible,\
  \ AI-driven inventory engine that acts like a seasoned logistics manager\u2014predicting\
  \ demand, syncing online and in-person sales instantly without an active internet\
  \ connection, and automatically generating purchase orders or restocking alerts\
  \ on their 375px mobile device.\n\n## Research Report\n**Competitive Analysis:**\n\
  - **Shopify:** Excellent multi-channel inventory syncing, but predictive restocking\
  \ relies heavily on expensive third-party apps (e.g., Inventory Planner). The native\
  \ tools are reactive, not proactive.\n- **Square:** Strong offline capability for\
  \ POS inventory reduction, but advanced predictive ordering is a paid add-on or\
  \ requires manual setup of reorder points. \n- **Wix:** Basic inventory tracking.\
  \ Lacks any meaningful AI forecasting or offline-first POS integration for physical\
  \ stock management.\n\n**Market Needs:**\nThe \"grandmother test\" reveals that\
  \ terms like \"SKU,\" \"Reorder Point,\" and \"Safety Stock\" alienate non-technical\
  \ owners. They simply want the app to say: *\"Priya, you're going to run out of\
  \ Red Summer Dresses by Thursday. Tap here to approve an order from your supplier.\"\
  * By integrating our AI Swarm (specifically an Operations Agent) with an offline-first\
  \ CRDT inventory ledger, OHC can provide predictive logistics out-of-the-box, eliminating\
  \ the cognitive load of supply chain management.\n\n## Design Doc\n\n### Architecture\
  \ Diagram\n```mermaid\ngraph TD;\n    subgraph Mobile Device\n        App[OHC Mobile\
  \ App 375px] --> InvUI[Inventory Insights Card];\n        App --> POS[Native POS\
  \ / Tap-to-Pay];\n        InvUI --> LocalCRDT[(Local SQLite CRDT Ledger)];\n   \
  \     POS --> LocalCRDT: Record Sale (Reduce Stock);\n    end\n\n    LocalCRDT --\
  \ \"Background Sync (Hybrid)\" --> Gateway[OHC API Gateway];\n    \n    Gateway\
  \ --> Ledger[Cloud Postgres Inventory Ledger];\n    Gateway --> OpsAgent[Operations\
  \ Agent];\n    Gateway --> FinanceAgent[Finance Agent];\n    \n    subgraph AI Operations\
  \ Department\n        OpsAgent --> Forecaster[Demand Forecasting Model];\n     \
  \   Forecaster -. \"Weather/Event API\" .-> ExternalData[External Data Sources];\n\
  \        OpsAgent --> Draft[Draft Purchase Order];\n        Draft --> FinanceAgent:\
  \ Check Budget/Cash Flow;\n    end\n    \n    FinanceAgent -- \"Approved to Draft\"\
  \ --> Gateway;\n    Gateway -- \"Push Notification\" --> App: \"Approve Restock?\"\
  ;\n```\n\n### Mobile UX Flow (375px first)\n1. **Dashboard Alert:** A simple card\
  \ on the main dashboard reads: \"Heads up! Based on recent sales, you'll run out\
  \ of Vanilla Bean Extract by Friday.\"\n2. **Action Sheet:** Tapping the card opens\
  \ a translucent Glassmorphism bottom sheet showing the exact item, current stock\
  \ level, and the predicted depletion timeline.\n3. **One-Tap Resolution:** A primary\
  \ button reads \"Draft Order to Supplier\". Tapping it immediately opens a pre-filled\
  \ email/SMS draft to their saved supplier, requesting the optimal restock amount.\n\
  4. **Offline Resilience:** If Fatima is at a festival with no service, every POS\
  \ sale deducts from the Local SQLite CRDT. The Operations Agent forecasting pauses,\
  \ but the local stock level remains accurate. Once service returns, the CRDT syncs\
  \ to Postgres, and the Ops Agent processes the batch to generate any necessary alerts.\n\
  \n### Key Design Decisions\n- **CRDT for Inventory Tracking:** Inventory must be\
  \ tracked using Commutative Replicated Data Types (CRDTs) to ensure that concurrent\
  \ sales (e.g., one on the website, one in-person via the mobile POS) don't result\
  \ in lost updates or negative inventory when syncing from offline states.\n- **Agentic\
  \ Proactivity, Human Approval:** The Operations Agent models demand and drafts purchase\
  \ orders, but NEVER executes a spend without explicit human approval via a simple\
  \ push notification.\n- **Zero-Jargon UI:** Hide all supply chain terminology behind\
  \ an \"Advanced Settings\" toggle. Present insights as plain-language sentences.\n\
  \n## Implementation Prompt\nImplement the AI-Driven Predictive Inventory feature.\n\
  **Goal:** Enable the Operations Agent to proactively monitor inventory levels and\
  \ alert the user when stock is predicted to run low, providing a one-tap action\
  \ to draft a supplier reorder.\n**CUJ (Customer User Journey):**\n1. Priya sells\
  \ multiple dresses both online and in-store.\n2. The system tracks these sales via\
  \ an offline-capable ledger.\n3. The Operations Agent analyzes the sales velocity\
  \ and predicts a stockout in 5 days.\n4. Priya receives a plain-language notification\
  \ on her mobile dashboard.\n5. She taps the notification and reviews a drafted purchase\
  \ order to her supplier.\n6. She approves the draft, sending it instantly.\n**Acceptance\
  \ Criteria:**\n- A mobile-first (375px) dashboard component displays predictive\
  \ inventory alerts using the design system's Glassmorphism tokens.\n- The feature\
  \ seamlessly aggregates local (offline) POS sales and cloud-based e-commerce sales.\n\
  - The Operations Agent logic triggers alerts based on configurable (or dynamically\
  \ calculated) sales velocity thresholds, not just static reorder points.\n- The\
  \ system must not automatically spend money or send supplier emails without the\
  \ user's explicit consent via the UI.\n"
issue_priority: P1
issue_category: research
issue_type: task
issue_label:
- agent-report
issue_scope: Medium
assignees: []
