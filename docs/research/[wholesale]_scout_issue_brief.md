# Issue Brief: B2B Wholesale Portals and Tiered Pricing

## Problem Statement
Many product-based SMBs eventually want to sell wholesale to other retailers, but their current platform only supports B2C retail pricing.

## Research Report
B2B e-commerce requires password-protected catalogs, bulk ordering interfaces, and custom price lists. Adding this capability allows OHC merchants to dramatically increase their order volume.

## Design Doc
**Architecture:**
- `PriceList` and `CustomerGroup` entities.
- Wholesaler authentication portal.
**AI Integration:**
- AI analyzes bulk order patterns and suggests optimal wholesale discount tiers.

## Implementation Prompt
Add support for customer groups and customer-specific price lists. Create a streamlined bulk-ordering UI. Acceptance criteria: A customer assigned to the 'Wholesale' group correctly sees discounted prices on the catalog compared to a standard retail customer.

## Priority
P3

## Estimated Scope
Large
