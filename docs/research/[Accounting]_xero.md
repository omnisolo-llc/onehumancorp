# [Accounting] OHC Tool Integration Research Brief: Xero Integration

## Title
Global Cloud Accounting Sync with Xero

## Problem Statement
While QuickBooks dominates North America, Xero is the leading cloud accounting platform for small businesses in many international markets (especially UK, Australia, New Zealand, and parts of Europe). To serve a global user base, OHC must offer seamless financial data syncing to Xero to eliminate manual bookkeeping for these users.

## Research Report
Xero is a highly popular, developer-friendly cloud accounting platform known for its beautiful UI and strong international presence.

**Evaluated Tool:**

1. **Xero (xero.com)**
    *   **Focus:** Cloud accounting for global SMBs.
    *   **Pros:** Very strong market share outside the US. Good internationalization support.
    *   **Cons:** Requires careful mapping of financial concepts (accounts, tax rates) to ensure data integrity.

**Recommendation:**
Xero should be integrated alongside QuickBooks to provide comprehensive accounting support for a global user base. The integration architecture can largely mirror the QuickBooks sync, focusing on pushing sales and payment data to the accounting ledger.

## Design Doc
**Integration Approach: Financial Sync to Xero**

1.  **Authentication & Configuration:**
    *   Implement standard authorization flow.
    *   Allow users to map OHC transactions to specific Account Codes and Tax Types.

2.  **Contact Sync:**
    *   When an invoice is generated, ensure the OHC Customer exists as a synchronized Contact.

3.  **Invoice & Payment Sync:**
    *   Translate OHC Invoices into external invoice records.
    *   Translate OHC Payments into external payment records applied to the corresponding invoice.

## Implementation Prompt
**Objective:** Implement financial data syncing from OHC to Xero.

**Acceptance Criteria:**
1.  Implement the authentication flow to connect an external accounting account.
2.  Utilize the synchronization engine to support pushing finalized Invoices and Payments based on tenant configuration.
3.  Ensure that before syncing an Invoice, the corresponding Contact exists in the external system.
4.  Push the Invoice and Payment data, handling currency and tax code mappings appropriately.

## Priority
P2

## Estimated Scope
Large
