issue_title: "Evaluate compute/API charging and BYOK"
issue_description: |
  # Compute/API charging and BYOK

  **Problem Statement:**
  Owners need transparency around the cost of operating their AI team, especially concerning compute/API usage, Bring Your Own Key (BYOK) setups, and provider-permitted native subscriptions.

  **Research Report / Current Capabilities:**
  Based on our latest updates (referencing the current active capability map and audit F13), OHC enforces strict distinctions between how inference and subscriptions are managed:

  - **BYOK (Bring Your Own Key):** Owners may supply their own API keys for certain integrations. The system ensures that customer-direct provider bills remain separate; there is no duplicate BYOK inference debit on the OHC side.
  - **API Key vs. Consumer Plan:** OHC distinguishes between a general-purpose API key, a consumer plan (like ChatGPT Plus or Claude Pro), and a native-client subscription. A consumer subscription is not a general-purpose API key, and the platform enforces fail-closed unsupported combinations (e.g., no session-token relaying or pooling).
  - **Standing Authority and Budget Caps:** All AI team actions operate under the owner's standing authority. The system enforces hard spend reservations before inference and provides truthful outcomes/receipts.
  - **Skill Provenance:** Superpowers workflow `brainstorming` (revision `5bf4e78011075bcfc0dc295f0724994cd123ee71`) was used to structure this report.

  **How to Manage API Costs and Authority:**
  1. Navigate to the **Account & Billing** tab in your OHC app.
  2. Under **Connected Accounts**, you can configure supported provider API keys or native subscriptions.
  3. Set up **Standing Authority** to define hard budget caps for your AI team. The system will never spend beyond this reservation.
  4. View the **Evidence Feed** to see precise usage metrics (integer subunits) and payer attribution for all AI activities.

  **Cost & Expectations:**
  OHC provides invoice-grade metering. You will see an estimated cost before a task starts, and a detailed bill when it finishes. Customer-direct BYOK usage is not billed as managed inference.

  **Exceptions and Recovery:**
  If a provider account is unsupported or a budget cap is reached, the system will pause the action and notify you. Your AI team's progress is saved, allowing you to easily retry later without duplicate charges.

issue_priority: "P1"
issue_category: "research"
issue_type: "report"
issue_label: "ohc:lane:finance"
assignees: []
