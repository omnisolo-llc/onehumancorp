# [Payment] Square POS Integration for Retail

## Title
🔍 Scout: Integrate Square for Seamless Online and In-Store Operations

## Problem Statement
Boutique owners like Priya have a physical shop and an OHC online store. Currently, if she sells an item in-person, OHC doesn't know about it. A customer online might buy that same item later, leading to inventory errors. She needs her OHC website and her physical shop to be perfectly synced.

## Research Report
- **Tool**: Square
- **Target Persona**: Priya (Boutique Owner), Brick-and-Mortar shops, Food Carts.
- **Value Proposition**: Square is the standard for physical retail. By syncing OHC with Square, we provide a unified inventory brain for the business.
- **Key Advantages**:
  - **Automatic Inventory Sync**: When an item is sold in the shop, it's instantly updated on the website.
  - **Catalog Import**: Owners can bring their existing Square products into OHC with one click.
  - **Unified Sales View**: See all sales in one dashboard.
- **Risks**: Requires handling of sync conflicts.
- **Pricing**: No additional cost for the synchronization.
- **Compatibility**: Fully supported in both Cloud and Standalone modes.

## Design Doc
- **User Experience**:
  - The owner clicks "Sync with Square" in the OHC Inventory tab.
  - They log in to their Square account securely.
  - OHC imports their products and current stock counts.
  - Sales made on Square hardware update the online stock automatically.
  - The owner sees a "Synced" status on their product list.
- **Visuals**: A unified inventory view showing shop vs. warehouse stock.

## Implementation Prompt
Develop a synchronization service between OHC and Square. Support the initial import of product catalogs. Implement a real-time listener for inventory change events to ensure OHC stock counts are accurate. Allow the OHC dashboard to aggregate sales data from both the online store and physical Square POS for a unified report.

## Priority
P1

## Estimated Scope
Large
