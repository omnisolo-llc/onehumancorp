# Smart Restock Forecasting and Alerting

## Problem Statement
Independent retailers and food-service businesses struggle immensely with accurate inventory planning. Purchasing too much stock ties up vital operating cash flow in dead, unsellable inventory; conversely, purchasing too little results in frustrating stockouts, damaged customer trust, and directly lost revenue.

## Research Report
Standard inventory management features found in platforms like Shopify are fundamentally reactive: they simply trigger an alert when a stock count hits absolute zero or a hard-coded low threshold. They completely fail to predict *when* the business will hit zero based on dynamic variables like historical sales velocity, upcoming seasonality, or external factors like local events. Small businesses urgently need predictive insights, not just descriptive tracking.

## Design Doc
### Architecture Vision
- **Entities**: Product, InventoryLevel, SalesHistory, ForecastModel, LeadTime.
- **UX Flow**:
  1. The background system continuously analyzes the sales velocity for a specific SKU, such as 'Vanilla Cupcakes', over the preceding 90 days.
  2. It identifies an upcoming, relevant holiday (e.g., Mother's Day) where historical data indicates sales typically spike by 200%.
  3. The system injects a timely alert into the user's Daily Briefing: 'Warning: You should order 50lbs of flour today to ensure you are ready for the projected volume this Mother's Day next week.'
- **Mobile UX**: Crucial restock alerts appear directly within the primary daily summary feed, ensuring they are seen, rather than remaining buried inside a dense, multi-column inventory data table.
- **Agent Integration**: The Analyst Agent executes sophisticated predictive models (e.g., time-series forecasting) over the continuous sales data pipeline, factoring in user-defined supplier lead times.

## Implementation Prompt
**Outcome**: Develop a predictive forecasting system that proactively warns the business owner to reorder stock *before* a stockout occurs, intelligently factoring in supplier lead times and historical sales velocity.
**Critical User Journey**:
1. The system accurately predicts an impending stockout for a key item within 7 days.
2. The system prominently alerts the owner via the first screen of the Daily Briefing.
3. The owner is prompted to place an order with their supplier in sufficient time to prevent the stockout.
**Acceptance Criteria**: The underlying logic must calculate a dynamic 'Days of Stock Remaining' metric rather than relying on static, raw inventory counts. The system must allow users to easily configure varying supplier lead times for different product categories.

## Priority
P2

## Estimated Scope
Medium
