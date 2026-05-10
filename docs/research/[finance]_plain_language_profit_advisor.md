# Issue Brief: [Finance] Plain-Language Profit Advisor

## Title
The Accountant: Plain-Language Financial Health & Profit Monitoring

## Problem Statement
Small business owners like Maya (Baker) and Priya (Boutique Owner) often see "Revenue" in their bank account but have no idea if they are actually making a "Profit" after ingredients, shipping, and transaction fees. They find QuickBooks and Excel intimidating. They need an "Accountant" who translates raw transaction data into a simple "Available to Spend" vs. "Save for Taxes" summary in plain language.

## Research Report
- **The "Financial Fog"**: 40% of small businesses fail due to cash flow mismanagement, not lack of sales.
- **Competitor Gap**:
  - **Shopify Analytics**: Show "Total Sales" prominently, but "Net Profit" is buried in complex reports or requires a 3rd party app.
  - **QuickBooks**: Too complex for a 10-minute-a-day user; requires manual categorization.
- **OHC Innovation**: "The Accountant" uses AI to auto-categorize expenses (e.g., "Maya just bought flour -> COGS") and provides a daily "Reality Check" notification.
- **Pain Points Addressed**:
  - Financial Fog (Understanding real profit).
  - Technical Jargon (Replacing "EBITDA" with "Take Home Pay").

## Design Doc

### High-Level Architecture (Mermaid.js)
```mermaid
graph TD
    Sales[Sales: Stripe/POS] --> Hub[OHC Hub]
    Expenses[Expenses: Scanned Receipts/Bank Sync] --> Hub
    Hub --> Accountant[The Accountant Agent]

    subgraph Analysis
        Accountant -->|Categorize| COGS[Cost of Goods Sold]
        Accountant -->|Calculate| Tax[Tax Liability Reserve]
        Accountant -->|Trend| Insights[Growth Trends]
    end

    Insights --> Dashboard[Mobile Dashboard: Plain Language]
    Dashboard -->|Notification| Owner[Maya: "You made $200 profit today!"]
```

### Mobile UX Flow (375px)
1.  **Profit Widget**: A prominent Glassmorphism card on the home screen:
    -   **"Take Home Today: $212"** (Large font).
    -   **"Revenue: $350"** (Smaller, subtle).
    -   **"Saved for Tax: $45"** (Auto-calculated).
2.  **Plain Language Briefing**: A "Daily Health" button that opens an AI summary:
    -   "Your vegan cakes are 20% more profitable than your chocolate ones this week."
    -   "You spent $50 more on shipping than usual. Should I look for cheaper labels?"
3.  **1-Tap Category Fix**: If the AI is unsure: "I saw a $12 charge at 'The Flour Mill'. Is this an Ingredient (COGS) or a Utility?" [Ingredient] [Utility]

### AI Agent Integration
- **Triggers**: Nightly at 11 PM (Daily Brief) or on `transaction.created`.
- **Context**: Accesses `order` items to calculate margins and `autodream_memories` for historical seasonal comparisons.
- **Approval Logic**: Auto-execute for internal categorization; Draft-for-Review for tax-saving suggestions or "Upgrade Plan" ROI analysis.

## Implementation Prompt
**To Implementer Agent:**
Implement "The Accountant" agent department. Create a financial processing engine that subscribes to `transaction.success` and `expense.recorded` events. The agent must automatically categorize transactions into "Profit," "COGS," "Tax," and "Fees" using LLM-based classification. Build the mobile-first (375px) "Profit Widget" and "Financial Briefing" UI using OHC design tokens (Outfit font, blur(20px)). Ensure that technical jargon is strictly avoided in the UI—use "Take Home" instead of "Net Income." Implement a "Reality Check" notification system that alerts the owner to significant margin drops or high-profit trends.

## Priority
P1 (Retention & Value)

## Estimated Scope
Medium
