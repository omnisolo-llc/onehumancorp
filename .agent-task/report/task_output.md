issue_title: "Implement Autonomous Micro-Influencer & Affiliate Revenue Share Engine"
issue_description: |
  # Research Report: Autonomous Micro-Influencer & Affiliate Revenue Share Engine

  ## Problem Statement
  Small business owners, particularly those in ecommerce, boutique retail, and specialized service sectors (e.g., Maya the baker, Priya the boutique owner), often rely on local word-of-mouth and social media micro-influencers to drive acquisition. However, setting up an affiliate or revenue-share program is incredibly tedious. It requires:
  1. Negotiating terms with individual creators.
  2. Generating unique discount codes or tracking links.
  3. Manually tracking attributions through spreadsheet exports.
  4. Manually calculating payouts at the end of the month.
  5. Handling the complexities of tax reporting for 1099 contractors.

  Our personas don't have the time or technical expertise to wire together Shopify plugins, Refersion, PayPal, and QuickBooks. If a local TikToker wants to promote Maya's custom cakes or Priya's boutique, the business owner needs an instant, zero-touch way to give that creator a unique link/code, track their sales, and split the payout invisibly without ever leaving their mobile phone.

  ## Market Gap
  Current market solutions for SMBs are heavily fragmented:
  - **Shopify:** Requires third-party apps (e.g., UpPromote, Refersion) which cost $30-$100/mo and still require manual payout processing (unless wired explicitly to a mass payout API, which is complicated to set up).
  - **Wix & Squarespace:** Very rudimentary built-in coupon codes but lack sophisticated, agent-driven multi-party revenue splitting at the time of transaction.
  - **Stripe Connect:** Excellent for multi-party payouts but heavily developer-focused. A non-technical user cannot set up Connect destination charges without writing code.

  ## Proposed Architecture
  By leveraging OneHumanCorp’s internal ledger and instant payout capabilities, we can automate the entire lifecycle. When a user (Maya) wants to partner with an influencer (Alex), her AI Operations Agent can automatically generate a unique link/code, update the storefront's edge cache to track attribution, monitor sales from that link, and split the revenue in the internal multi-tenant ledger instantly at checkout.

  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph Mobile Client (375px)
          A[OHC Mobile Dashboard] -->|Requests Affiliate Link| B(AI Operations Agent)
          C[Checkout Page] -->|Submits Payment + Affiliate Code| D(Universal Checkout Engine)
      end

      subgraph OHC Backend
          B -->|Provisions Trackable Entity| E[Edge Cache & Link Resolver]
          D -->|Processes Transaction| F[Split Payout Ledger]
          F -->|Allocates Base Revenue| G[(Maya's OHC Wallet)]
          F -->|Allocates Commission| H[(Influencer's Wallet/Payout)]
          F -->|Triggers Notification| I(AI Communications Agent)
      end

      I -->|Sends SMS/Email via OHC| J[Influencer Inbox]
      E -.->|Attribution Metrics| K[Analytics Aggregator]
      K -->|Performance Updates| A
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)

  **Screen 1: Partner Hub (Home)**
  - Translucent glass card header: "Growth Partners & Affiliates"
  - Primary CTA Button: "Create New Partner Link" (prominent, centered)
  - List view of active partners: Avatar, Name, Revenue Generated (e.g., "Alex @tiktok - $450 generated").

  **Screen 2: Create Partner Link (BottomSheet)**
  - Input: Partner Name / Social Handle
  - Slider: Commission Rate (e.g., 10% - defaults to standard rate)
  - Toggle: "Auto-Payout at end of month"
  - Button: "Generate Magic Link"

  **Screen 3: Success & Share**
  - Large QR Code for in-person sharing.
  - Tap-to-copy URL link.
  - "Send via WhatsApp/SMS" button utilizing the system share sheet.

  ### Key Design Decisions
  - **Zero-Touch Payouts:** Using the existing OHC multi-tenant ledger, revenue splits occur synchronously at transaction time, storing the commission in a sub-ledger until the auto-payout date.
  - **Edge-Level Attribution:** Trackable links are resolved at the edge (CDN layer) to inject a highly durable attribution cookie/token before routing to the multi-tenant SaaS storefront, ensuring high performance.
  - **Agent Orchestration:** The AI Operations Agent handles the onboarding of the influencer (sending them an onboarding link to collect their payout details securely) and the AI Communications Agent sends daily/weekly performance briefs.

  ## Implementation Prompt
  **To the Implementer Agent:**
  Build the "Autonomous Micro-Influencer & Affiliate Revenue Share Engine" core business logic and behavior.

  **Core User Journey (CUJ):**
  1. Maya navigates to the "Partners" tab on her mobile app and clicks "Create Link".
  2. She enters "TikTok Alex" and sets a 15% commission.
  3. The system generates a unique trackable URL (e.g., `maya.ohc.com/p/alex`).
  4. A customer purchases a $100 custom cake using that link.
  5. At checkout, the system automatically routes $85 to Maya's ledger balance and $15 to Alex's commission ledger balance.

  **Acceptance Criteria:**
  - Define a multi-tenant data model to handle affiliate tracking and revenue sharing correctly.
  - Implement the logic to generate trackable links/codes.
  - Integrate with the core checkout engine to intercept payments and correctly append split ledger entries.
  - Ensure the logic is fully compatible with headless and mobile clients.
  - Route commissions to the internal OHC wallet/ledger system first, without prescribing specific external integrations for payouts.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
