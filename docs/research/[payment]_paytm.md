## [Payment] Issue Brief

**Title**: Scout 🔍: Integrate Paytm for the Indian Market
**Problem Statement**:
Stripe is not always the best option in all regions. Small businesses in India prefer localized payment gateways like Paytm for better acceptance rates and lower fees.
**Research Report**:
- **Tool**: Paytm Payment Gateway
- **Evaluation**: Paytm is widely used in India. Supporting it expands OHC's reach and provides a familiar checkout experience for Indian customers.
- **Ease of Use**: Business owners enter their Paytm API credentials in the settings.
- **Pricing**: Transaction-based fees, generally competitive in the Indian market.
- **Cloud vs. Standalone**: Works in both Cloud and Standalone modes by configuring API keys.
**Design Doc**:
- User configures Paytm credentials in the 'Payments' settings.
- During checkout, customers in India see Paytm as a payment option.
- OHC handles the payment redirect and webhook verification.
**Implementation Prompt**:
Add Paytm as a payment gateway option. Provide fields for the user to input their Merchant ID and Secret Key. Implement the checkout flow and payment verification webhook.
**Priority**: P2
**Estimated Scope**: Medium
