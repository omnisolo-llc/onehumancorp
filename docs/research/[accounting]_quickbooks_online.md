# QuickBooks Online Sync

## Title
Automated QuickBooks Online Sync for Sales and Invoicing

## Problem Statement
Small business owners often spend hours each week manually transferring sales data, invoices, and customer information from their operating platform (like OHC) into their accounting software. This double data entry is not only tedious and time-consuming but also prone to human error, leading to inaccurate financial records and tax headaches. They need a seamless way for their sales activity to automatically reflect in their books.

## Research Report
*   **Target Persona:** Carlos (Service Business Owner) or Maya (Retail/E-commerce), who both need accurate accounting without spending their weekends doing bookkeeping.
*   **Market Share:** QuickBooks Online (QBO) is the dominant accounting software for small businesses in North America.
*   **Developer Experience:** Intuit provides robust API documentation, a developer portal, and well-supported SDKs. The OAuth 2.0 flow is standard and reliable.
*   **Pricing:** API access is free for developers. End-users must have an active QuickBooks Online subscription (which most target users already have).
*   **SaaS Viability:** Extremely viable for Cloud (multi-tenant) via standard OAuth 2.0. For Standalone (local) deployments, OAuth redirect URIs might require a proxy service or specific configuration to route back to local instances.
*   **Competitor Landscape:** Almost all major e-commerce and POS platforms (Shopify, Square, Wix) offer direct, highly-rated QBO integrations. Lacking this puts OHC at a competitive disadvantage for mature small businesses.

## Design Doc
*   **Trigger:**
    *   *Real-time (Optional):* When an invoice is marked as "Paid" or a sale is completed in OHC.
    *   *Batch (Recommended baseline):* A daily end-of-day sync agent runs to bundle and push all closed transactions.
*   **Action:**
    *   Authenticate via OAuth 2.0.
    *   Map OHC customers to QBO customers (create if not exists).
    *   Push OHC sales/invoices to QBO as Sales Receipts or Invoices.
    *   Record Payments against those invoices in QBO.
*   **User Experience (UI):**
    *   A simple "Connect to QuickBooks" button in the Integrations settings.
    *   A status dashboard showing the last successful sync, total transactions synced, and any errors (e.g., "Could not sync Invoice #102: Missing tax code").
    *   A toggle for "Auto-sync daily" vs "Manual sync".

## Implementation Prompt
**User-Facing Outcome:**
Small business owners should be able to connect their existing QuickBooks Online account to OHC with a few clicks. Once connected, their daily sales, customer data, and tax collections should automatically flow into their QuickBooks ledger without manual intervention, keeping their accountant happy and their books balanced.

**Acceptance Criteria:**
1.  Users can authenticate and link their QuickBooks Online account securely.
2.  Users can toggle an auto-sync feature.
3.  When enabled, completed sales/invoices in OHC are successfully created as corresponding records in QBO.
4.  If a sync fails (e.g., due to a mapping error or API outage), the user is notified via the OHC UI with clear, actionable steps to resolve it.
5.  No technical jargon (like "OAuth tokens" or "API rate limits") is exposed to the user.

## Priority
`P1` (High) - Foundational requirement for any business doing significant volume.

## Estimated Scope
Large
