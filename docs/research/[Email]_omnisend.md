# [Email Marketing] OHC Tool Integration Research Brief: Omnisend

## Title
E-commerce Focused Email and SMS Automation

## Problem Statement
Small retail or e-commerce businesses using OHC for order management need marketing tools that natively understand products, carts, and revenue. Generic email tools require extensive custom integration to send automated "abandoned cart" or "product recommendation" emails.

## Research Report
Omnisend specifically targets the e-commerce sector, differentiating itself from generic platforms like Mailchimp.

**Evaluated Tool:**

1. **Omnisend (omnisend.com)**
    *   **Focus:** Email and SMS marketing specifically built for e-commerce.
    *   **Pros:** Out-of-the-box workflows for e-commerce (abandoned carts, post-purchase). Integrated SMS. Excellent dynamic product blocks in their email builder.
    *   **Cons:** Heavily optimized for specific platforms; integrating a custom platform like OHC requires adhering to their specific e-commerce data models.

**Recommendation:**
If OHC expands its e-commerce capabilities (e.g., managing a storefront or shopping cart directly), integrating with a specialized e-commerce marketing platform is highly recommended. It excels at turning e-commerce data into actionable campaigns.

## Design Doc
**Integration Approach: E-commerce Data Sync**

1.  **Product and Order Sync:**
    *   Unlike simple contact sync, integrating an e-commerce focused marketing platform requires OHC to sync its product catalog and order history.
    *   OHC must implement a background sync to push Products, Orders, and Carts to the external system.

2.  **Marketing Execution:**
    *   With the data synced, the business owner uses the external UI to activate pre-built e-commerce automations.

## Implementation Prompt
**Objective:** Build the foundation for e-commerce data syncing.

**Acceptance Criteria:**
1.  Implement integration clients for syncing Products and Orders.
2.  Create background workers to perform initial full-syncs of historical product and order data.
3.  Implement event triggers to push real-time updates when an order is placed in OHC.

## Priority
P3

## Estimated Scope
Large
