# [accounting] QuickBooks Online Autonomous Sync

## Title
Implement QuickBooks Online Autonomous Accounting Sync

## Problem Statement
Small business owners (like Carlos the contractor or Priya the boutique owner) often spend hours every week manually entering sales data, invoices, and expenses into their accounting software. This manual data entry is error-prone and takes time away from running their actual business. When tax season approaches, a disconnected or messy ledger causes massive stress and often leads to higher CPA fees. They need their sales and operations platform to automatically talk to their accounting system without human intervention.

## Research Report
- **Market Need & Findings**: QuickBooks Online (QBO) holds a massive market share for SMB accounting in North America and globally. An overwhelming majority of CPAs and bookkeepers require their SMB clients to use it. A deep integration is a baseline expectation for any serious business operating platform.
- **Ease of Use**: While the QBO interface itself can be complex for non-accountants, the *integration* simplifies their life dramatically by removing manual entry.
- **Pricing & Viability**: QBO is a SaaS tool (starting around $30/mo) that businesses already pay for.
- **Capabilities & Limits**: Intuit provides a robust REST API with OAuth 2.0 authentication and reliable webhooks for real-time updates. The API covers Invoices, Payments, Customers, and Journal Entries comprehensively.
- **Hybrid SaaS Viability**: In a multi-tenant cloud environment, each tenant authenticates their own QBO connection via OAuth. In a standalone local environment, the local instance can securely store the OAuth tokens and communicate directly with Intuit's cloud, making it perfectly viable for OHC's hybrid architecture.

## Design Doc
- **Integration Flow**: The user links their QBO account via a standard OAuth flow in the OHC integrations dashboard.
- **Trigger**: The sync is triggered automatically when an invoice is paid, a daily sales batch is closed, or a payment is refunded in OHC.
- **Actions Taken**: OHC translates the event into QBO API payloads (e.g., creating a Sales Receipt, Payment, or Journal Entry) and securely pushes it to QBO.
- **User Visibility**: The business owner sees a "QuickBooks: Connected" status indicator. In their daily AI briefing, they see a simple summary: "Synced 15 transactions to QuickBooks today."

## Implementation Prompt
**User-Facing Outcome:**
Provide a one-click "Connect to QuickBooks" button in the Settings > Financials area. Once authorized, all closed sales, paid invoices, and processed refunds should automatically flow into the connected QuickBooks Online account.

**Acceptance Criteria:**
- A business owner can securely connect their QBO account via OAuth.
- Daily sales and payment data sync seamlessly without manual intervention.
- The UI provides a clear, plain-language log of synced transactions (e.g., "Invoice #102 synced to QuickBooks").
- If the QBO connection breaks or a sync fails, the system proactively alerts the owner with simple instructions on how to fix it (e.g., "Please reconnect your QuickBooks account").

## Priority
P1 (High)

## Estimated Scope
Medium
