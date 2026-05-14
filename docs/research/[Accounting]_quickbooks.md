# [Accounting] OHC Tool Integration Research Brief: QuickBooks Integration

## Title
Automated Financial Sync with QuickBooks Online

## Problem Statement
Small business owners spend significant time manually reconciling their sales and expenses from operational platforms (like OHC) into their accounting software. This manual data entry is tedious, prone to errors, and delays financial visibility. They need a way for invoices, payments, and customers created in OHC to automatically sync to their accounting system.

## Research Report
QuickBooks Online (QBO) is the dominant accounting software for small businesses in North America and many international markets.

**Evaluated Tool:**

1. **QuickBooks Online (quickbooks.intuit.com)**
    *   **Focus:** Cloud-based accounting software for SMBs.
    *   **Pros:** Massive market share. Robust integration ecosystem.
    *   **Cons:** Complex initial setup to align with tax codes and account charts.
    *   **Pricing:** Starts at $35/mo.
    *   **Modes:** Cloud (via API).

**Recommendation:**
Integrating with QuickBooks Online is a near-mandatory feature for any platform handling invoicing or payments for SMBs. The integration should focus on a one-way sync from OHC to QBO, acting as the system of record for operational sales data.

## Design Doc
**Integration Approach: One-Way Financial Sync to QuickBooks Online**

1.  **Authentication & Configuration:**
    *   Implement standard authorization flow for users to connect their QBO company.
    *   Provide a mapping UI where users map OHC items/services to specific QBO Income Accounts and Tax Codes.

2.  **Customer Sync:**
    *   When a customer is invoiced in OHC, check if they exist in QBO. If not, synchronize their profile information.

3.  **Invoice & Payment Sync:**
    *   When an Invoice is finalized in OHC, generate a corresponding invoice record in the accounting system.
    *   When a Payment is recorded in OHC against an Invoice, generate a payment record and apply it to the corresponding invoice.

## Implementation Prompt
**Objective:** Implement financial data syncing from OHC to QuickBooks Online.

**Acceptance Criteria:**
1.  Implement the authentication flow to connect an external accounting account.
2.  Implement a synchronization engine that listens for finalized Invoices and Payments in OHC.
3.  Ensure that before syncing an Invoice, the corresponding Customer exists in the accounting system, creating them if necessary.
4.  Push the Invoice and subsequent Payment data, ensuring the payment is correctly linked to the invoice.

## Priority
P1

## Estimated Scope
Large
