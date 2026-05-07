# Native Integration of Paystack for African Markets

## Title
Native Integration of Paystack for African Markets

## Problem Statement
Small business owners in Africa face significant hurdles accepting online payments because global providers like Stripe are unavailable or do not support local payment methods (e.g., Mobile Money, USSD). They need a native, trusted payment gateway integrated directly into OHC to sell online without friction.

## Research Report
- **Strategy**: Direct integration with Paystack's API to handle checkouts and subscriptions.
- **Target Persona**: Merchants, creators, and service providers operating in Nigeria, South Africa, Ghana, and other supported African countries.
- **Advantages**: Paystack is highly trusted in Africa and supports localized payment methods (cards, bank transfers, USSD, Mobile Money). Their API is developer-friendly and reliable.
- **Risks**: Payout schedules and currency conversion complexities when merchants sell internationally.
- **Pricing**: Standard localized transaction fees (e.g., 1.5% + NGN 100 for local Nigerian transactions). No setup or monthly fees.
- **Compatibility**: Compatible with both Cloud mode (via multi-merchant routing or standard OAuth) and Standalone mode (using individual API keys).

## Design Doc
- During onboarding, if a user selects a supported African country, OHC suggests connecting Paystack.
- User navigates to Settings > Payments and clicks "Connect Paystack".
- The user completes the Paystack OAuth flow or inputs their API keys.
- At checkout, customers in the storefront see a seamless "Pay with Paystack" option, which opens the Paystack inline modal to complete the transaction using their preferred local method.
- Paystack webhooks notify OHC of successful payments to update the order status.
- **AI Integration**: The Finance Agent aggregates Paystack revenue alongside other income sources, presenting a unified dashboard in the user's local currency.

## Implementation Prompt
Integrate Paystack as a primary payment provider for merchants in supported African regions. The integration must support the Paystack Inline checkout on the frontend and robust webhook handling on the backend to verify and fulfill orders securely.
- **Acceptance Criteria**: Merchant in a supported region can connect Paystack. Storefront checkout dynamically offers Paystack. Successful payments via the inline modal trigger backend fulfillment via webhooks.
- **Priority**: P1
- **Estimated Scope**: Large
