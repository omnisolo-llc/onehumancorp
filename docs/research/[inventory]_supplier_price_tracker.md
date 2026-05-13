# Automated Supplier Price Tracker

## Problem Statement
SMB profit margins are frequently and silently squeezed by slowly rising supplier costs. Busy owners often simply do not notice that the wholesale cost of flour, lumber, or specific parts has incrementally increased by 10% over six months, severely eating into their net profits.

## Research Report
Inflation and rising Cost of Goods Sold (COGS) are top concerns for SMBs globally. However, the process of tracking COGS manually across dozens of invoices is incredibly tedious. Owners desperately need an automated system that alerts them the moment their margin on a specific product drops below a sustainable threshold.

## Design Doc
### Architecture Vision
- **Entities**: SupplierInvoiceRecord, MaterialCostLedger, ProductMarginProfile.
- **UX Flow**:
  1. The user forwards their digital supplier invoice emails directly to a dedicated OHC system address.
  2. The system utilizes OCR and LLMs to extract the individual line items and updates the internal material cost database.
  3. If a cost increase is detected, the system immediately alerts the user: 'Warning: The wholesale cost of Flour increased. Your net margin on Vanilla Cupcakes is now only 15%. Should we increase the retail price to $4.50?'
- **Mobile UX**: A critical alert card surfaced prominently within the Daily Briefing interface.
- **Agent Integration**: The Procurement Agent is tasked with parsing unstructured invoices, maintaining the cost ledger, and continuously monitoring margin health against user-defined targets.

## Implementation Prompt
**Outcome**: Build a system that autonomously tracks raw material costs from ingested invoices and proactively recommends necessary retail price adjustments to fiercely protect the business's profit margins.
**Critical User Journey**:
1. A supplier quietly raises their wholesale prices.
2. The system detects the incremental increase upon ingesting the latest forwarded receipt.
3. The system informs the owner of the margin compression and suggests a specific, actionable retail price hike.
**Acceptance Criteria**: The underlying parsing engine must accurately and reliably extract line-item data from unstructured PDF and image-based invoices utilizing advanced OCR/LLM techniques.

## Priority
P2

## Estimated Scope
Large
