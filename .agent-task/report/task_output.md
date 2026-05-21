# Integration Research Report: QuickBooks Online

## Overview
This report details the evaluation of QuickBooks Online (QBO) as a high-impact third-party integration for One Human Corp (OHC). As part of our mission to solve real problems for small business owners, automated accounting synchronization was identified as a critical gap and opportunity.

## Track 1: Dynamic Integration & Market Need Discovery
Through analyzing competitor platforms, app marketplaces, and SMB discussion forums (such as r/smallbusiness), accounting and bookkeeping consistently emerged as top pain points. Small business owners, representing our core personas like Carlos (contractor) and Priya (boutique owner), spend disproportionate amounts of time on manual data entry or pay significant fees to bookkeepers for mundane data transfer.

**Integration Target:** QuickBooks Online by Intuit.
**Rationale:** QBO is the dominant accounting software for SMBs in North America. An integration here is not just a feature; it is often a strict requirement for a business to adopt a new operational platform, as mandated by their CPA.

## Track 2: Selected Tool Deep-Dive Evaluation

### User-First Value Mapping
- **The Problem:** The business owner has to manually copy sales, invoices, and payments from their operational system (OHC) into their accounting system (QBO).
- **The Solution:** A seamless, invisible integration that automatically syncs this financial data.
- **Value to Persona:** Carlos doesn't need to spend his Friday evenings typing invoices into QuickBooks. Priya doesn't have to worry about discrepancies between her POS sales and her bank deposits when tax time arrives.

### Capabilities & Limits
- **API Quality:** Intuit offers a comprehensive REST API covering all necessary financial objects (Invoices, Customers, Payments, Sales Receipts, Journal Entries).
- **Authentication:** Standard OAuth 2.0 flow.
- **Reliability:** High reliability with webhook support for real-time bi-directional synchronization if needed, though pushing data from OHC to QBO is the primary requirement.

### SaaS Viability & Architecture
- **Pricing:** QBO is a paid SaaS tool for the business owner. The API access for developers (OHC) is free, making it economically viable.
- **Multi-tenant Cloud:** In OHC's cloud mode, each tenant will securely authorize their own QBO account via OAuth. OHC will manage the token lifecycle per tenant.
- **Standalone Local:** In standalone mode, the local application can manage the OAuth tokens locally and communicate directly with Intuit's cloud API, ensuring the integration works regardless of OHC's deployment model.

## Track 3: Strategic Integration Dispatch
A detailed issue brief has been drafted according to the Mission Queue Protocol. It focuses on the business value and user experience, leaving specific API mapping and database design to the implementation team.

**Output Generated:** `docs/research/[accounting]_quickbooks_online.md`

## Proposed Next Steps
1. Engineering team to review the issue brief.
2. Allocate a developer to register an Intuit Developer account and create the initial sandbox app for OHC.
3. Design the database schema (Tenant-level) to securely store Intuit OAuth tokens.
4. Implement the backend sync logic targeting the QBO Sales Receipt and Payment API endpoints.
