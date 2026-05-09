# 🔎 Scout: Tool Integration Research Q4 Report

## Executive Summary
This report evaluates new tool integration candidates for One Human Corp (OHC) from the perspective of a small business owner. The goal is to identify and outline solutions that save time, reduce friction, and open new revenue streams for users across Cloud and Standalone environments.

In this iteration, the focus was placed on **Payment Processing** explicitly tailored for the Chinese market.

## Evaluated Tool: Alipay

### The Problem
Small business owners using OHC often struggle to close sales with Chinese customers and tourists because these buyers primarily rely on digital wallets rather than international credit cards. A lack of localized payment options leads to high cart abandonment rates for online sales and friction at physical point-of-sale systems.

### Research Findings
- **Market Dominance:** Alipay (Ant Group) is the world's largest mobile payment platform with over 1.3 billion users.
- **Consumer Behavior:** In China, digital payments via QR codes have largely replaced cash. A Nielsen report indicates over 90% of Chinese tourists prefer to use mobile payments overseas when the option is available.
- **Features for Non-Chinese Users:** Alipay includes a "Tourpass" feature allowing non-Chinese users to pre-load funds, demonstrating its flexibility.
- **Pricing:** While aggregators like Stripe charge higher fees (e.g., 2.9% + 30¢), direct integrations or specialized gateways can offer lower transaction fees (1.5% - 2.5%) without monthly retainers.
- **Compatibility:**
  - **Cloud:** Easily aggregated via major gateways.
  - **Standalone:** Requires the merchant to provide direct API credentials.

### Proposed Integration
The integration involves enabling Alipay as a checkout option for online storefronts and generating dynamic QR codes for in-person POS transactions.
- **User Experience:** The merchant simply toggles "Enable Alipay" in their payment settings and connects their gateway or credentials. Customers scan the generated QR code with their Alipay app or are redirected online to authenticate.
- **AI Synergy:** The "Finance & Operations" agent can track these cross-border settlements and proactively recommend localized marketing campaigns around major Chinese holidays (e.g., Golden Week).

A complete Issue Brief containing the Problem Statement, Design Doc, Implementation Prompt, Priority, and Estimated Scope has been drafted and saved to `docs/research/[payment]_alipay.md`.

## Next Steps
- Pass the drafted Issue Brief to an implementation agent.
- Continue investigating other categories (such as Calendar/Scheduling and Social Media Inbox) in future research passes to expand OHC's small business integrations.
