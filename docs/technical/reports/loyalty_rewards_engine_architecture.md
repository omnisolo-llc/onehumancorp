# Autonomous Multi-Tenant Loyalty and Rewards Engine Research Report

## 1. Executive Summary

This report proposes the architecture and design of an Autonomous Multi-Tenant Loyalty and Rewards Engine for OneHumanCorp (OHC). This engine will empower small business owners (like Maya, Carlos, Priya, Leo, and Fatima) to easily create, manage, and automate customer loyalty programs with zero technical setup. By integrating deeply with OHC's existing multi-tenant architecture and AI agents, the engine will drive repeat business and improve customer retention automatically.

## 2. Core Personas and Use Cases

*   **Maya (The Home Baker):** "Buy 10 cakes, get 1 free" punch card. Automated birthday rewards (e.g., free cupcake).
*   **Carlos (The Freelance Handyman):** Referral bonuses (e.g., 10% off next service if you refer a neighbor).
*   **Priya (The Boutique Owner):** Points-based tier system (Silver, Gold, Platinum) with early access to new collections.
*   **Leo (The Music Tutor):** Milestone rewards (e.g., 50% off the 5th lesson).
*   **Fatima (The Food Cart Operator):** Simple digital punch card for repeat pre-orders.

## 3. System Architecture & Design

### 3.1. Database Schema (PostgreSQL)

The engine will utilize OHC's multi-tenant PostgreSQL database with Row-Level Security (RLS) on `tenant_id`.

*   **`loyalty_programs`**: Defines the rules and type of program (points, punch card, tiers).
    *   `id`, `tenant_id`, `name`, `type`, `config` (JSONB for flexible rules), `is_active`.
*   **`customer_loyalty_accounts`**: Tracks individual customer progress.
    *   `id`, `tenant_id`, `customer_id`, `program_id`, `points_balance`, `current_tier`.
*   **`loyalty_transactions`**: Audit log of all points earned/redeemed.
    *   `id`, `tenant_id`, `customer_id`, `program_id`, `amount` (+/-), `reason`, `created_at`.
*   **`rewards`**: Available rewards to redeem.
    *   `id`, `tenant_id`, `program_id`, `name`, `cost_in_points`, `discount_type`, `discount_value`.

### 3.2. AI Agent Integration

*   **The Promoter (Marketing):** Automatically suggests creating a loyalty program if customer retention is low. Generates promotional emails/SMS when a customer unlocks a reward.
*   **The Salesperson (Sales):** Uses loyalty points as an upsell tactic ("You're only 50 points away from a free coffee!").
*   **The Manager (Operations):** Automatically updates loyalty balances upon successful order completion or booking fulfillment.

### 3.3. API Layer (gRPC/REST)

*   `CreateLoyaltyProgram(TenantID, Config)`
*   `EarnPoints(TenantID, CustomerID, Amount, Reason)`
*   `RedeemReward(TenantID, CustomerID, RewardID)`
*   `GetCustomerLoyaltyStatus(TenantID, CustomerID)`

## 4. Mobile-First UI/UX Implementation (Flutter)

*   **Business Owner View (Management):** A simple, one-tap "Enable Loyalty Program" toggle. AI handles the configuration based on the business type. No complex rules engines exposed to the user.
*   **Customer View (Storefront):** A clear, visually appealing progress bar (e.g., 3 out of 5 punches) displayed prominently on the customer's profile and checkout screen.

## 5. Next Steps & Action Plan

```yaml
issue_title: "[feature] Implement Multi-Tenant Loyalty and Rewards Engine Core"
issue_priority: "P1"
issue_description: "Implement the database schema and core API endpoints for the loyalty and rewards engine, ensuring RLS and AI agent integration."
issue_todo_list:
  - [x] Research and design database schema for loyalty programs.
  - [ ] Implement `loyalty_programs` and `customer_loyalty_accounts` tables in PostgreSQL.
  - [ ] Create gRPC/REST APIs for earning and redeeming points.
  - [ ] Integrate with 'The Manager' AI agent for automated point allocation.
issue_label: ["feature", "backend", "loyalty"]
```
