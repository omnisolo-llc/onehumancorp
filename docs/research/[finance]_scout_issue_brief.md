# Issue Brief: Zero-Friction Receipt OCR & Tax Categorization

## Problem Statement
Tax season is a universal nightmare for non-technical SMBs. They frequently co-mingle personal and business expenses, lose physical receipts, and struggle with complex accounting software like QuickBooks. Hiring a dedicated bookkeeper is too expensive.

## Research Report
Financial anxiety is a leading cause of burnout among solopreneurs. By tracking all inbound revenue natively and allowing users to seamlessly snap photos of expense receipts, OHC can use Vision AI to auto-categorize transactions into standard tax buckets (e.g., Schedule C categories for US users). This provides a massive, immediate value-add: a one-click export for their accountant at year-end.

## Design Doc
**High-Level Architecture & Entities:**
- `ExpenseRecord`: Entity capturing outgoing funds.
- `TaxCategory`: Taxonomy for classification.
- Integrations: Vision AI model optimized for OCR and structured data extraction.

**Mobile UX Flow:**
1. **Action:** User buys flour for bakery. Opens OHC app, taps 'Log Expense'.
2. **Capture:** Camera opens. User snaps photo of the physical receipt.
3. **Processing:** AI extracts data: Merchant (Costco), Date (Oct 12), Amount ($45.20).
4. **Categorization:** AI intelligently infers category: "Cost of Goods Sold / Supplies".
5. **Save:** Record is saved and immediately reflected in the dashboard's P&L summary.

**AI Agent Integration Points:**
- AI performs OCR on varied receipt formats.
- AI maps the extracted line items to standard tax categorization taxonomies.

## Implementation Prompt
Build an expense tracking module that allows users to capture receipts via image upload. Utilize OCR and AI to asynchronously extract the merchant name, total amount, date, and infer the most likely tax category.

**Critical User Journey (CUJ):**
1. User uploads a photo of a receipt.
2. System extracts text via OCR and parses structured financial data.
3. AI assigns a tax category based on merchant context.
4. Expense is logged and aggregated into the platform's financial reporting view.

**Acceptance Criteria:**
- Uploading a sample receipt image must successfully extract correct amount, date, and merchant.
- The AI must successfully infer a logical category for common expenses (e.g., Home Depot = Supplies/Maintenance).
- The system must generate a basic CSV export of logged expenses.

## Priority
P3

## Estimated Scope
Medium
