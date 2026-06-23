issue_title: "Implement AI-Driven Omnichannel Loyalty & Referral Loop Architecture"
issue_description: |
  # Research Report: AI-Driven Omnichannel Loyalty & Referral Loop Architecture

  ## Title
  Implement AI-Driven Omnichannel Loyalty & Referral Loop Architecture

  ## Problem Statement
  Small business owners and creators (like Maya the Baker or Priya the Boutique Owner) struggle with customer retention and predictable organic growth. Existing platforms like Shopify or Wix require owners to stitch together multiple expensive, confusing third-party apps (e.g., Yotpo, Smile.io, Klaviyo) to build basic loyalty or referral programs. These programs are often rigid, strictly points-based, fail to bridge the gap between online and in-store (POS) seamlessly, and lack proactive AI engagement. Owners need an integrated, zero-configuration system that invisibly tracks loyalty across all channels and autonomously rewards top customers and referrals.

  ## Research Report
  - **Market Landscape & Competitors:**
    - **Shopify:** Relies almost entirely on the app ecosystem (Smile.io, Yotpo). These apps introduce their own dashboards, subscription fees ($50-$500/mo), and injected scripts that degrade storefront performance. Omnichannel (POS + Web) sync is famously brittle.
    - **Square/Weebly:** Offers a built-in loyalty program that is mostly POS-centric and rigidly tied to simple punch-card or points mechanics. Lacks proactive AI.
    - **Wix/Squarespace:** Offer very basic discount codes, but full referral tracking or multi-tier loyalty requires complex setup or external tools.
  - **The OHC Opportunity:** By leveraging the core "Consolidated Memory" and "Customer Identities" schemas, OHC can build a native loyalty engine that doesn't feel like a separate product. The system can track every interaction (purchases, referrals, positive feedback) as a `LoyaltyEvent`. The AI "Promoter" and "Ambassador" agents can use this data to autonomously send targeted rewards, run referral campaigns, and identify churn-risk VIPs without the owner needing to configure point values or tiers manually.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Identity] --> B(Consolidated Memory Ledger)
      C[POS Transaction] --> B
      D[E-commerce Checkout] --> B
      E[Referral Link Click] --> B
      B --> F{Loyalty Engine / CRDT}
      F --> G[Dynamic Tiers & Rewards]
      G --> H[The Promoter Agent]
      H --> I[Automated SMS/Email: "You unlocked a reward!"]
      H --> J[Owner Feed: "Maya, 3 VIPs are at risk of churning. I drafted a discount offer."]
  ```

  ### Mobile UX Flow (375px)
  1. **Owner View (The Feed):** The owner sees a clean card: "Your referral program generated 5 new customers this week. AI drafted a thank-you note to the top referrer."
  2. **Owner Settings:** A simple toggle in the "Customers" tab: `[x] Enable Auto-Rewards & Referrals`. No complex point configuration required; the AI determines optimal reward thresholds based on margin and LTV.
  3. **Customer View (Wallet/Profile):** Customers access a dynamic Apple Wallet-style pass or a personalized link showing their relationship status, available perks, and a one-tap copyable referral link.

  ### AI Agent Integration
  - **The Promoter (Marketing):** Automatically generates and distributes unique referral links. Monitors the `LoyaltyEvent` stream to trigger hyper-personalized reward messages (e.g., "Happy 1-year anniversary of your first cake order!").
  - **The Ambassador (Customer Success):** Uses loyalty status to prioritize support responses and automatically applies perks (like free expedited shipping) for top-tier customers.
  - **Finance/Decision Assistant:** Tracks the true CAC (Customer Acquisition Cost) of the referral program versus the LTV of acquired customers, presenting a plain-language summary to the owner.

  ### Key Design Decisions
  - **Event-Sourced Loyalty:** Loyalty is calculated dynamically from the immutable `Consolidated Memory` event stream, not stored as a mutable points counter. This prevents sync issues between offline POS and online storefronts.
  - **Tenant Isolation:** All loyalty events and AI prompts are strictly bound to the `tenant_id` via PostgreSQL RLS.
  - **Zero-Config Default:** The system uses standard ecommerce metrics to auto-configure standard referral rewards, removing the initial setup paralysis for the owner.

  ## Implementation Prompt
  Implement the core `loyalty_events` and `referral_links` database schemas, ensuring strict multi-tenant RLS. Extend the `CustomerIdentity` model to compute a dynamic loyalty score based on these events. Create the necessary service-layer methods for the Promoter agent to query top referrers and generate unique referral links. Expose these capabilities via gRPC/REST for the mobile-first frontend to display loyalty status to the owner and the customer.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
