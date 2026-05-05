# [Finance] The Business Advisor: Plain-Language Health Briefing

## Problem Statement
Founders like **Carlos (Handyman)** or **Maya (Baker)** are "Financially Foggy." They see revenue in Stripe but don't know their actual profit after costs/fees without a spreadsheet. They find standard accounting dashboards (QuickBooks, Xero) overwhelming and full of jargon.

## Research Report
- **Competitor Gap**:
    - **Shopify**: "Analytics" tab is full of complex charts (Session rate, Conversion funnel) which micro-sellers don't understand.
    - **Wix**: Similar chart-heavy approach.
    - **GoDaddy**: Very thin financial reporting.
- **User Pain**: 35% of SMB owners feel "Financial Fog." They want to know "Am I making money?" not "What is my bounce rate?"
- **Evidence**: Reddit r/smallbusiness is full of "how do I track profit" threads.

## Design Doc
- **Architecture**:
    - **Agent**: `BusinessAdvisoryAgent` (The Advisor) + `FinanceAgent` (The Accountant).
    - **Tool**: `finance_report` (new tool) that aggregates `orders` (revenue) and `products.price_cents` (COGS/metadata).
    - **Output**: A text-based summary instead of a chart.
- **Mobile UX (375px)**:
    - **UI**: A "Daily Briefing" card at the top of the dashboard.
    - *"Good morning Maya. Yesterday you made $450 ($310 profit). Your 'Vegan Brownie' is the star this week. You've earned enough to cover next month's rent already!"*
- **AI Integration**: Uses LLM to translate raw SQL aggregates into encouraging, plain-language insights.

## Implementation Prompt
**Outcome**: Implement a "Plain-Language Financial Briefing" that replaces complex charts with actionable, human-readable business insights.
**Critical User Journey**:
1. Owner opens the app in the morning.
2. `BusinessAdvisor` agent runs a summary of the previous 24 hours.
3. Agent pulls data from `orders`, `order_items`, and `products`.
4. Agent generates a 3-sentence summary of profit, top sellers, and one "Win of the Day."
**Acceptance Criteria**:
- No charts allowed in the primary briefing view.
- Must focus on **Profit**, not just **Revenue**.
- Tone must be supportive and non-technical.

## Priority
P2

## Estimated Scope
Small
