# Issue Brief: Autonomous Purchase Order Generation

## Problem Statement
When inventory drops, SMB owners have to manually email or call suppliers to reorder, which is time-consuming and often delayed, leading to out-of-stock scenarios.

## Research Report
Automating the procurement cycle for fast-moving consumer goods reduces stockouts by up to 40%. By storing supplier contact info and standard order quantities, OHC can use an agent to draft and send POs automatically.

## Design Doc
**Architecture:**
- `Supplier` and `PurchaseOrder` entities.
- Email integration (SendGrid/Postmark) for dispatching POs.
**AI Integration:**
- AI agent monitors stock and autonomously emails the supplier requesting a restock based on predefined thresholds.

## Implementation Prompt
Build a background worker that monitors inventory levels. When a threshold is crossed, automatically generate a PDF Purchase Order and email it to the linked supplier. Acceptance criteria: A mock low-stock event triggers an email payload containing a formatted PO.

## Priority
P3

## Estimated Scope
Large
