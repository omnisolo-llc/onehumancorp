# Automated Referral Score Optimizer

## Problem Statement
SMBs, particularly service providers, rely heavily on word-of-mouth marketing for high-quality leads. However, they almost entirely lack systematic, low-friction ways to track, encourage, and financially reward successful referrals. Implementing complex, enterprise-style loyalty programs is far too expensive and administratively heavy to set up and manage.

## Research Report
Our quantitative research indicates that a high 'referral_score' (quantifying how likely a customer is to actively refer others) is the single strongest predictor of long-term SMB survival and profitability. Despite this, only an estimated 15% of micro-SMBs have any form of active referral program. Legacy tools like Smile.io or Yotpo require heavy configuration, ongoing management, and significant monthly fees that deter adoption.

## Design Doc
### Architecture Vision
- **Entities**: CustomerRecord, ReferralLink, RewardLedger, DiscountCode.
- **UX Flow**:
  1. Immediately following a 5-star rating post-service, the customer automatically receives a unique, trackable referral link via SMS.
  2. When a friend clicks the link and completes a purchase, the system autonomously applies a predetermined discount to the new customer's cart.
  3. Simultaneously, the system automatically issues a reward (e.g., store credit or a future discount) to the original referrer.
- **Mobile UX**: A highly simplified dashboard view for the owner showcasing 'Top Referrers' and 'Total Revenue from Referrals', abstracting away the complex point calculations.
- **Agent Integration**: The Growth Agent autonomously manages the entire lifecycle: link generation, tracking attribution, and automatically applying the financial rewards to the respective accounts without human intervention.

## Implementation Prompt
**Outcome**: Engineer a fully automated, zero-configuration referral program that seamlessly incentivizes and tracks word-of-mouth marketing.
**Critical User Journey**:
1. A highly satisfied customer completes a purchase or service.
2. The system prompts the customer to share a unique link with friends.
3. A friend utilizes the link to make their first purchase.
4. Both the referrer and the new customer are rewarded instantly and automatically.
**Acceptance Criteria**: The system must operate entirely in the background; it must absolutely not require manual discount code creation, point tracking, or ledger reconciliation by the business owner.

## Priority
P2

## Estimated Scope
Medium
