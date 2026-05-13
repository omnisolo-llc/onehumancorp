# [strategy]_pricing_model_analysis.md

## Introduction
A major friction point for the SMB target market is the upfront cost associated with setting up a business. This document analyzes the pricing models of primary competitors and outlines OHC's strategy to maximize both user acquisition and long-term lifetime value (LTV).

## Competitor Pricing Landscape

### 1. Shopify
- **Model**: Hard SaaS with rigid tiers.
- **Entry Point**: $39/month (Basic plan). The $5/month "Starter" plan is practically unusable for a real storefront, acting mostly as a social media link-in-bio tool.
- **Hidden Costs**: Users typically need to spend an additional $50-$100/month on third-party apps to achieve basic functionality like abandoned cart recovery or advanced reviews.
- **Impact on SMBs**: High barrier to entry. Users feel they are "renting" the software and must generate sales immediately to justify the monthly cost, leading to high churn in the first 30 days if sales don't materialize.

### 2. Wix & Squarespace
- **Model**: Freemium (Wix) / Free Trial to SaaS (Squarespace).
- **Entry Point**: Wix offers a free tier, but it includes intrusive Wix branding and does not allow a custom domain. Paid e-commerce plans start around $27-$30/month.
- **Impact on SMBs**: The free tier is often used for experimentation, but users rarely launch a serious business on it. The transition to a paid plan is a significant friction point.

### 3. Square Online
- **Model**: Transaction-based Freemium.
- **Entry Point**: $0/month. Users only pay processing fees (e.g., 2.9% + 30¢) when they make a sale.
- **Impact on SMBs**: Extremely low barrier to entry. It aligns the platform's success with the merchant's success. However, advanced features are locked behind a $29/month Plus plan.

## The OHC Pricing Strategy: "Success-Aligned Freemium"

To achieve rapid market penetration and build the largest network of SMBs, OHC must remove the monthly subscription barrier for new businesses.

### Phase 1: The Zero-Cost Launch
- **The Offer**: Free to launch. Free to build. Free to use AI features.
- **Revenue Model**: OHC takes a slightly higher transaction fee on sales (e.g., 3.5% + 30¢) on the free tier.
- **The Psychology**: The user feels OHC is a partner. "We don't make money until you make money." This completely removes the anxiety of a monthly bill while they are trying to figure out their business.

### Phase 2: The "Growth" Tier
- **The Trigger**: Once a merchant reaches a certain GMV threshold (e.g., $1,000/month), the higher transaction fee becomes more expensive than a flat monthly rate.
- **The Offer**: The merchant can upgrade to OHC Pro for $29/month, which drops the transaction fee to the industry standard (2.9% + 30¢) and unlocks premium features (e.g., advanced API access, multi-staff accounts).
- **The Psychology**: The upgrade is driven by math, not restriction. The user upgrades to save money, which is a positive UX interaction.

### Phase 3: The Ecosystem Upsell
- **The Offer**: Because OHC handles everything natively, we can upsell high-value, high-margin services that normally require separate subscriptions:
  - **OHC Capital**: Micro-loans based on sales history (taking a percentage fee).
  - **OHC Payroll**: For merchants who hire their first employee.
  - **OHC Marketing**: A managed ad-spend service where OHC AI runs Facebook ads for a flat 10% management fee.

```mermaid
graph LR
    A[User Signs Up (Free)] --> B[Builds Store with AI (Free)]
    B --> C[Makes First Sale]
    C --> D{OHC takes 3.5% fee}
    D --> E[Store Grows > $1k/mo]
    E --> F[User Upgrades to Pro ($29/mo)]
    F --> G[OHC takes 2.9% fee]
    F --> H[Upsell: OHC Capital/Marketing]
```
