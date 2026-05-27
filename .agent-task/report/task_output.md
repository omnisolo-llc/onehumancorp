issue_title: '[architecture] Autonomous Universal Tax Compliance & Remittance Engine'
issue_description: "# [architecture] Autonomous Universal Tax Compliance & Remittance\
  \ Engine\n\n## Problem Statement\nSmall business owners like Maya (baker), Carlos\
  \ (handyman), and Priya (boutique owner) want to sell their goods and services without\
  \ worrying about the labyrinth of tax jurisdictions. As Maya starts shipping her\
  \ vegan cookies out-of-state, she unwittingly triggers economic nexus in three different\
  \ states, each with unique tax rates, filing frequencies, and exemptions. Priya\
  \ selling digital products globally faces VAT complexities. Currently, SMBs either\
  \ ignore the problem until audited, or spend thousands of dollars on complex accounting\
  \ software and CPAs. They need an invisible, zero-config engine that automatically\
  \ calculates, collects, files, and remits taxes globally, allowing them to remain\
  \ legally compliant without touching a single tax form or doing mental math.\n\n\
  ## Research Report\n*   **Shopify Tax:** Requires configuration, monitoring nexus\
  \ thresholds, and manual intervention to set up properly. It calculates tax at checkout,\
  \ but remittance is still largely up to the merchant or requires third-party apps\
  \ (like TaxJar/Avalara) with steep learning curves and separate logins.\n*   **Wix\
  \ / Squarespace:** Basic tax calculation features, often relying on integrations.\
  \ They do not proactively manage nexus or autonomously file returns.\n*   **Stripe\
  \ Tax:** Good developer API, but still requires the merchant to register in jurisdictions\
  \ and actively handle filing and remittance outside of Stripe's automated calculation.\n\
  *   **OneHumanCorp (OHC) Differentiation - \"Zero-Touch Compliance\":** OHC eliminates\
  \ the concept of \"Tax Settings.\" The Autonomous Tax Engine acts as an invisible\
  \ CPA. It monitors sales volume per jurisdiction, proactively registers the business\
  \ for nexus, calculates exact local tax at checkout (accounting for product type\
  \ exemptions like non-taxable clothing in PA), and automatically sweeps tax collected\
  \ into a reserved treasury ledger to seamlessly file and remit on behalf of the\
  \ user when due.\n\n## Design Doc\n\n### Architecture Diagram\n```mermaid\nerDiagram\n\
  \    ORDER_EVENT ||--o{ TAX_ENGINE : \"Triggers Calculation\"\n    \n    TAX_ENGINE\
  \ {\n        string tenant_id \"Multi-tenant isolation\"\n        string transaction_id\n\
  \    }\n    \n    TAX_ENGINE ||--o{ NEXUS_MONITOR : \"Consults & Updates\"\n   \
  \ TAX_ENGINE ||--o{ JURISDICTION_RULES : \"Checks exemptions\"\n    \n    NEXUS_MONITOR\
  \ {\n        string state_or_country\n        decimal current_volume\n        boolean\
  \ is_registered\n    }\n\n    TAX_ENGINE ||--o{ TREASURY_LEDGER : \"Sweeps Tax Funds\"\
  \n    \n    TREASURY_LEDGER {\n        string account_id\n        decimal tax_reserved_balance\n\
  \    }\n\n    TAX_ENGINE ||--o{ AGENT_DEPARTMENTS : \"Consults (Legal, Finance)\"\
  \n    \n    AGENT_DEPARTMENTS ||--o{ AUTO_FILER : \"Triggers Remittance\"\n    AUTO_FILER\
  \ ||--o{ GOVERNMENT_PORTAL : \"Files Return\"\n```\n\n### UI Wireframes & 375px\
  \ Baseline\n**Core Layout: macOS-style Translucent Glass + Ubiquiti UniFi Modular\
  \ Dashboard Cards**\n*   **Global Viewport:** 375px width (Mobile First). No horizontal\
  \ scrolling.\n*   **Dashboard Card (Financial View):**\n    *   A unified \"Sales\
  \ & Compliance\" card. Frosted glass background (`rgba(255, 255, 255, 0.05)` with\
  \ `backdrop-filter: blur(10px)`).\n    *   Large text: `$12,450 Available`. Subtext:\
  \ `$845 Tax Reserved & Handled \u2728`.\n*   **Tax Compliance Detailed View (If\
  \ tapped):**\n    *   Simple visual map or list of active states.\n    *   \"\u2728\
  \ AI is monitoring 5 states. You are actively registered in NY and CA. Returns filed\
  \ automatically next Tuesday.\"\n    *   No complex tables or settings. Just a reassuring\
  \ status screen.\n\n### Mobile UX Flow\n1. **Notification:** Maya receives a push\
  \ notification: \"\u2728 You've reached the tax threshold in Texas. The Legal Agent\
  \ has automatically registered your business and started collecting TX tax. No action\
  \ needed.\"\n2. **Checkout (Customer View):** When a customer in Texas buys a cake,\
  \ the exact local tax is calculated and displayed transparently.\n3. **Settlement\
  \ (Maya's View):** The payment settles. Maya sees her net profit available for payout\
  \ immediately, while the tax portion is invisibly swept into the reserved Treasury\
  \ Ledger.\n4. **Filing:** At the end of the month, Maya gets a plain-language summary:\
  \ \"\u2728 $142 in taxes were filed and paid to Texas on your behalf.\"\n\n### AI\
  \ Agent Integration Points\n*   **Legal Department:** Monitors state and country\
  \ nexus thresholds dynamically. Autonomously handles the paperwork to register the\
  \ business in new jurisdictions using the owner's secured identity.\n*   **Finance\
  \ Department:** Maintains the real-time `TREASURY_LEDGER`. Sweeps tax revenue at\
  \ the moment of transaction to prevent the owner from accidentally spending tax\
  \ money.\n*   **Operations Department:** Classifies new products added to the catalog\
  \ (e.g., categorizing \"Vegan Cake\" as grocery vs. prepared food depending on the\
  \ state's taxability rules) using semantic understanding.\n\n### Key Design Decisions\
  \ (Why, not How)\n*   **Automated Sweeping:** SMBs often spend tax money accidentally\
  \ because it sits in their main bank account. Sweeping funds instantly to a reserved\
  \ ledger is a core protective feature.\n*   **Invisible Registration:** Forcing\
  \ a baker to navigate the Texas Comptroller website is a failure. The AI must handle\
  \ the registration invisibly using the platform's trusted identity layer.\n*   **Zero\
  \ Trust & Security:** Multi-tenant isolation is critical. `TREASURY_LEDGER` and\
  \ `NEXUS_MONITOR` data must be strictly scoped to `tenant_id` to prevent commingling\
  \ of funds or data leaks.\n\n## Implementation Prompt\n**To the Implementer Swarm:**\n\
  Your goal is to build the foundational architecture for the Autonomous Universal\
  \ Tax Compliance & Remittance Engine. Maya should be able to sell her goods globally\
  \ without ever configuring tax settings manually.\n\n**Customer User Journey (CUJ):**\n\
  1. Maya signs up and creates a product.\n2. The system categorizes the product for\
  \ tax purposes.\n3. Maya receives orders from multiple states.\n4. The engine tracks\
  \ her sales volume against nexus thresholds.\n5. When a threshold is met, the AI\
  \ auto-registers her and starts calculating, collecting, and reserving tax on subsequent\
  \ orders from that jurisdiction.\n\n**Acceptance Criteria:**\n*   **Mobile Parity:**\
  \ Design the system to power the simple 375px financial UI cards (Net vs. Tax Reserved)\
  \ without exposing the complex underlying ledger to the client.\n*   **Data Model:**\
  \ Implement the `NEXUS_MONITOR` and `TREASURY_LEDGER` schemas with strict `tenant_id`\
  \ PostgreSQL RLS policies.\n*   **Agent Trigger:** Create the event hook that triggers\
  \ the Legal AI Agent when a `NEXUS_MONITOR` threshold hits 100%.\n*   **Isolation:**\
  \ Ensure the tax calculation service cannot access data outside the requested transaction's\
  \ tenant context.\n*   **Simplicity:** Do not build a user-facing settings screen\
  \ for configuring tax rates. The system must be zero-config.\n\n## Estimated Scope\n\
  Large\n"
issue_priority: P0
issue_category: research
issue_type: task
issue_label:
- agent-report
assignees: []
