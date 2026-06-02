```yaml
issue_title: "[research] Autonomous Multi-Tenant Loyalty and Rewards Engine"
issue_priority: "P1"
issue_description: "Implement a multi-tenant loyalty and rewards engine. The system will empower non-technical small business owners (e.g., Maya the Home Baker, Carlos the Freelance Handyman) to effortlessly implement and manage loyalty programs, driving repeat business without complex technical setup. The implementation will include a conversational setup flow, autonomous point tracking via The Manager agent, personalized milestone notifications via The Ambassador, and basic performance reporting via The Advisor."
issue_todo_list:
  - [ ] Implement PostgreSQL schema (loyalty_programs, accounts, transactions, rewards) with row-level security.
  - [ ] Build gRPC/REST APIs for program and account management.
  - [ ] Integrate point earning/redemption logic into the core checkout flow (The Manager).
  - [ ] Implement automated milestone notifications (The Ambassador).
  - [ ] Develop conversational setup flow and performance reporting UI for business owners.
issue_label: ["research", "high-impact", "loyalty"]
```

# OHC Research Report: Autonomous Multi-Tenant Loyalty and Rewards Engine

## Executive Summary
This report outlines the architectural design for a multi-tenant loyalty and rewards engine within the OHC platform. This system is designed to empower non-technical small business owners (e.g., Maya the Home Baker, Carlos the Freelance Handyman) to effortlessly implement and manage loyalty programs, driving repeat business without complex technical setup.

## Target Personas & Use Cases

1.  **Maya (The Home Baker):** Wants a simple "Buy 9 cakes, get the 10th free" punch card system. Needs this to work seamlessly whether orders are placed online via Instagram DMs or through her OHC storefront.
2.  **Priya (The Boutique Owner):** Needs a tiered points system (e.g., Bronze, Silver, Gold) based on total spend, rewarding high-value customers with early access to new collections and exclusive discounts. Requires integration with both online and in-store (POS) purchases.
3.  **Fatima (The Food Cart Operator):** Requires a highly simplified, mobile-first interface to quickly award points or stamps during fast-paced, high-volume transactions, potentially using QR codes.

## Architecture Design

The Loyalty and Rewards Engine will be built as a core, multi-tenant service within the existing Go + Bazel backend, leveraging PostgreSQL for robust data storage and the AI Agent Swarm for autonomous management.

### 1. Data Model (PostgreSQL)

A new set of tables will be introduced, adhering strictly to OHC's row-level tenant isolation policy.

*   `loyalty_programs`: Defines the structure of the program for a specific tenant (e.g., Points-based, Punch Card, Tiered).
    *   `id` (UUID, Primary Key)
    *   `tenant_id` (UUID, Foreign Key)
    *   `program_type` (Enum: POINTS, PUNCH_CARD, TIERED)
    *   `name` (String)
    *   `description` (String)
    *   `config` (JSONB: stores specific rules, e.g., points per dollar, stamps required for reward)
    *   `is_active` (Boolean)

*   `customer_loyalty_accounts`: Tracks individual customer progress within a tenant's program.
    *   `id` (UUID, Primary Key)
    *   `tenant_id` (UUID, Foreign Key)
    *   `customer_id` (UUID, Foreign Key)
    *   `program_id` (UUID, Foreign Key)
    *   `current_balance` (Integer/Decimal: Points or Stamps)
    *   `tier_id` (UUID, optional, Foreign Key to tiers table if applicable)

*   `loyalty_transactions`: An immutable ledger of all loyalty activities (earning/redeeming).
    *   `id` (UUID, Primary Key)
    *   `tenant_id` (UUID, Foreign Key)
    *   `account_id` (UUID, Foreign Key)
    *   `transaction_type` (Enum: EARN, REDEEM, ADJUSTMENT)
    *   `amount` (Integer/Decimal)
    *   `reference_order_id` (UUID, optional, linking to the specific purchase)
    *   `timestamp` (Timestamp)

*   `loyalty_rewards`: Defines available rewards.
    *   `id` (UUID, Primary Key)
    *   `tenant_id` (UUID, Foreign Key)
    *   `program_id` (UUID, Foreign Key)
    *   `reward_type` (Enum: DISCOUNT_PERCENTAGE, FIXED_AMOUNT_OFF, FREE_ITEM)
    *   `cost` (Integer/Decimal: Points/Stamps required)
    *   `details` (JSONB)

### 2. AI Agent Integration (The Swarm)

The real power of the OHC system lies in its autonomous agents. The loyalty engine will be deeply integrated with existing departments:

*   **The Manager (Operations):**
    *   *Trigger:* When an order is completed (online or POS), The Manager automatically calculates and credits loyalty points/stamps to the customer's account based on the active program rules.
    *   *Action:* Automatically applies eligible rewards to a cart if the customer chooses to redeem them during checkout.
*   **The Ambassador (Customer Success):**
    *   *Trigger:* When a customer reaches a new tier or earns a reward.
    *   *Action:* Drafts and sends personalized, friendly notifications (SMS, Email, or DM) celebrating the milestone. Example: "Hi Sarah! You just unlocked a free cupcake with Maya's Bakery! Use code FREECAKE at checkout."
*   **The Promoter (Marketing):**
    *   *Trigger:* Business Advisory flags a slow sales period.
    *   *Action:* Automatically generates a promotion: "Double Points Weekend!" and schedules social media posts and email campaigns to announce it, creating the necessary temporary rules in the `loyalty_programs` config.
*   **The Advisor (Business Advisory):**
    *   *Action:* Analyzes loyalty program performance. Provides weekly insights to the business owner: "Your Punch Card program is driving 15% more repeat orders. Customers are redeeming rewards on average every 4 weeks. Consider offering a bonus stamp on Tuesdays to boost slow weekday sales."

### 3. Frontend Implementation (Flutter)

The UI must adhere to OHC's Radical Simplicity and Mobile-First constraints.

*   **Owner View (The App):**
    *   A completely conversational setup. The owner doesn't configure "points per dollar." They answer AI prompts: "What kind of reward do you want to give?" -> "A free coffee after 10 purchases." The AI handles the data mapping.
    *   A simple dashboard showing total members, points issued, and rewards redeemed.
*   **Customer View (Storefront/Link-in-bio):**
    *   A clean, visually appealing "Digital Wallet" section where customers can instantly see their current status, points balance, and available rewards.
    *   Integration with Apple Wallet / Google Wallet for easy access during in-person (POS) transactions.

### 4. Technical Considerations & Interoperability

*   **Idempotency & Concurrency:** Given the financial nature of loyalty points, all transactions must be strictly idempotent to prevent double-crediting. Redis Redlock will be crucial during concurrent checkouts.
*   **Cloud vs. Standalone Sync:** The loyalty ledger (`loyalty_transactions`) must be reliably synchronized when switching between Cloud and Standalone modes. The `StateHandoff` protocol must include loyalty account balances to ensure a seamless customer experience if the business owner goes offline.

## Proposed Implementation Plan

1.  **Phase 1: Foundation (Database & API)** - Implement the PostgreSQL schema, row-level security, and basic CRUD gRPC/REST APIs for programs and accounts.
2.  **Phase 2: Core Logic (Operations Agent)** - Integrate point earning/redemption logic into the checkout flow (The Manager). Ensure idempotency.
3.  **Phase 3: AI Augmentation (Ambassador & Advisor)** - Implement automated notifications for milestones and basic reporting for the business owner.
4.  **Phase 4: Frontend (Mobile-First UI)** - Build the conversational setup flow for owners and the digital wallet view for customers.
