# Scout 🔍: Integrate Paytm for UPI-First Payments in India

## Problem Statement
Small vendors in India rely heavily on Paytm for its brand recognition and extreme ease of use for UPI payments. They want to offer the same familiar "Paytm" experience to their online customers, allowing them to pay quickly using their Paytm wallet or linked bank accounts.

## Research Report
- **Tool**: Paytm Payment Gateway
- **Target Persona**: Small vendors and service providers in India.
- **Evaluation**: Massive brand trust in India. Very strong UPI and Wallet integration.
- **Ease of Use**: Good for the customer; merchant account verification can be strict but is standard for the region.
- **Pricing**: ~0% for UPI (depending on current government/bank regulations), ~2% for cards.
- **Reputation**: Ubiquitous in India for both offline and online payments.
- **Cloud vs. Standalone**: Compatible with both.

## Design Doc
- **Checkout Flow**: Customers can pay via the Paytm app or by scanning a dynamic UPI QR code natively on the checkout page.
- **Simplicity**: No need for customers to enter card details if they have the Paytm app installed.
- **Integration**: Webhooks notify OHC of successful payments to trigger order fulfillment.

## Implementation Prompt
Integrate Paytm Payment Gateway as a payment option. Focus on providing a seamless UPI-first payment experience for Indian customers, supporting both deep linking to the Paytm app and dynamic QR code generation.
- **Acceptance Criteria**: Merchant can link Paytm. Customers see "Pay with Paytm" at checkout. UPI and Wallet payments work.
- **Priority**: P2
- **Estimated Scope**: Medium
