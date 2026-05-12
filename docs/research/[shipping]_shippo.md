# Issue Brief: Automated Shipping Labels

## Title
Implement Automated Shipping Labels for Small Business Owners

## Problem Statement
A local crafter sells 10 handmade mugs a week. Currently, she has to manually copy the customer's address, guess the box weight, buy a label on a post office website, and manually email the tracking number.

## Research Report
Shippo connects to multiple shipping carriers to compare rates and buy labels.

**Persona Impact:** The crafter opens an order in OHC, clicks 'Create Label'. OHC automatically suggests the cheapest shipping rate. She clicks 'Buy', and a PDF label pops up to print. OHC automatically emails the tracking link to the customer.

**Advantages:** Massive time saver. Saves the user money by comparing rates dynamically.

**Risks:** The user must accurately input the weight and dimensions of their products in OHC for the rates to be accurate.

**Pricing Estimate:** Extremely SMB friendly. Often no monthly fee, just a few cents per label generated.

**Environment:** Works perfectly in both Cloud and Standalone modes.

## Design Doc
1.  **1-Click Label:** A prominent 'Buy Shipping Label' button on the Order Details page.
2.  **Rate Comparison:** A simple dropdown showing the available carriers and their prices.
3.  **Auto-Tracking:** Automatic customer notifications when the package ships.

## Implementation Prompt
Integrate a shipping provider to allow users to purchase discounted shipping labels and automatically send tracking information to their customers without leaving OHC.

## Priority
P1

## Estimated Scope
Large

### Unique Considerations
The label generation process must support batching. If the crafter receives 50 orders over the weekend, she must be able to select all 50 and click 'Generate Labels' to produce a single, multi-page PDF for her thermal printer.
