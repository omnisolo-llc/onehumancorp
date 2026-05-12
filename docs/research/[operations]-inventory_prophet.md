# Agentic Inventory Prophet

## Problem Statement
Managing inventory is a manual, error-prone process. Small boutiques like Priya's often sell out of items in-store and forget to update their online shop, leading to canceled orders and angry customers.

## Research Report
- **Findings**: Inventory sync issues cause significant customer dissatisfaction and lost trust.
- **Competitors**: Standard platforms rely on manual entry or complex ERP integrations.
- **Evidence**: Reddit r/ecommerce (Nov 2023) shows constant complaints about overselling products due to bad syncing.

## Design Doc
- **Architecture Flow**:
  - Sales data stream continuously updates inventory levels.
  - An AI background agent monitors thresholds.
  - When stock is low, agent drafts supplier reorder emails.
- **Mobile UX (375px first)**:
  - Simple alerts for low stock.
  - One-tap "Reorder" button that sends the AI-drafted email to the supplier.

```mermaid
graph LR;
    Sales[Sales Event] --> Inv[Inventory DB];
    Inv --> Agent[Prophet Agent];
    Agent --> Alert[Low Stock Alert];
    Alert --> Email[Draft Supplier Email];
```

## Implementation Prompt
**Outcome**: An intelligent background service that tracks inventory velocity and proactively alerts the owner before items run out, offering one-click reorder solutions.
**Critical User Journey (CUJ)**:
1. Item stock drops below predicted 7-day run rate.
2. User gets an alert on their phone.
3. User sees an AI-drafted email to their saved supplier.
4. User taps 'Send'.
**Acceptance Criteria**: The system must use predictive velocity, not just static hard thresholds, to trigger alerts.

## Priority
P2

## Estimated Scope
Medium
