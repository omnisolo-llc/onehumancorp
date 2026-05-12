# Issue Brief: Mercado Pago (LATAM Payments)

## Title
Implement Mercado Pago (LATAM Payments) for Small Business Owners

## Problem Statement
A boutique owner in Brazil cannot use Stripe because her customers prefer paying with Pix (instant bank transfers). If she doesn't offer Pix, she loses the sale.

## Research Report
Mercado Pago is the dominant payment processor in Latin America, supporting local payment methods.

**Persona Impact:** The Brazilian boutique owner can securely accept payments using the methods her customers trust most. The money flows smoothly into her local bank account.

**Advantages:** Unlocks the massive LATAM market. Essential for product-market fit in the region.

**Risks:** The onboarding process for merchants in LATAM can involve stringent local identity verification steps.

**Pricing Estimate:** Transaction fees are competitive but vary by specific country.

**Environment:** Works in both Cloud and Standalone modes.

## Design Doc
1.  **Region Detection:** Automatically suggest Mercado Pago during onboarding if the user's business address is in a supported LATAM country.
2.  **Checkout Experience:** Ensure the OHC checkout page prominently displays local methods like Pix or OXXO alongside standard credit cards.

## Implementation Prompt
Integrate Mercado Pago so businesses in Latin America can accept payments using the local methods (like Pix) that their customers demand.

## Priority
P0

## Estimated Scope
Large

### Unique Considerations
For OXXO payments in Mexico, the payment is completed in cash at a convenience store days later. The OHC UI must clearly reflect this 'Pending Cash' state to the business owner so they do not ship the goods prematurely.
