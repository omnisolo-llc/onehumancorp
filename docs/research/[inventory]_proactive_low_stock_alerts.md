# Issue Brief: Proactive Low Stock Alerts

## Problem Statement
Small business owners selling physical products, like Priya (Boutique Owner), often find out an item is out of stock only when a customer complains or when they manually check shelves. They don't have time to review analytics dashboards daily.

## Research Report
- **Competitor Gap:** Shopify and Wix provide inventory tracking but rely on the user to check a dashboard or set up manual notification rules. This places the cognitive load on the user.
- **Pain Point Alignment:** Addresses "Operational Fatigue" (Rank #2).
- **Opportunity:** Shift the burden to "The Vigilant Manager" (Operations Agent) to automatically monitor sales velocity and notify the owner when action is needed.

## Design Doc
### High-Level Architecture
- **Inventory Trigger:** The Operations Agent subscribes to `OrderCompleted` events.
- **Sales Velocity Calculation:** The agent calculates the run rate for each product. If a product will run out within 7 days (based on current velocity), it triggers a `LowStockRisk` event.
- **Action Required Feed:** The alert is surfaced as a simple card in the mobile dashboard's "Action Required" feed, allowing the owner to 1-tap reorder or update stock levels.

### Implementation Prompt
Implement a background task in the Operations Agent that monitors inventory levels and sales velocity. When stock is predicted to run out soon, surface a proactive "Action Required" alert in the merchant's mobile dashboard. Provide a 1-tap action to adjust the inventory count.

## Priority
P2

## Estimated Scope
Small
