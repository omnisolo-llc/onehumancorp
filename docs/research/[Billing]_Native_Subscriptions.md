# Issue Brief: Native Recurring Subscriptions

## 1. Problem Statement
Service providers (like tutors and consultants) and businesses offering physical goods (like coffee roasters) rely on recurring revenue. Currently, setting up subscriptions often requires complex third-party tools that are difficult for non-technical users to configure and integrate.

## 2. Research Report
**Findings:**
- Subscription models provide predictable revenue, a major goal for SMBs.
- Competitors like Shopify offer native subscriptions, but setup can still be convoluted.
- Service-based businesses (our beachhead) strongly desire "set and forget" billing for ongoing clients.

**Sources:**
- Reddit (r/smallbusiness): "I need a simple way to bill my weekly clients without chasing invoices."
- Competitor analysis: Shopify Subscriptions adoption rates.

## 3. Design Doc
### High-Level Architecture
- **Entities**: SubscriptionPlan, Customer, BillingCycle, Invoice.
- **Integration**: Leverage Stripe Billing infrastructure invisibly.
- **Trigger**: Customer purchases a recurring service or product.

### UI / UX Flow (Mobile First - 375px)
1.  **Product Setup**: When adding a product or service, a simple checkbox: "Offer as a subscription."
2.  **Frequency Selection**: Plain-language options: "Every week," "Every month."
3.  **Customer Portal**: A simple interface for customers to update payment methods or pause subscriptions.

### AI Integration Points
- AI-driven dunning management: The Follow-Up Closer agent can automatically handle failed payment retries with personalized, polite messages.

## 4. Implementation Prompt
**User-Facing Outcome:**
Business owners can offer recurring services or products with a single click, enjoying predictable revenue while the system automatically handles billing, invoices, and failed payments.

**Critical User Journey (CUJ):**
1.  Owner creates a "Weekly Music Lesson" service and checks the subscription box.
2.  Customer signs up and enters payment details once.
3.  System automatically charges the customer weekly and sends receipts.

**Acceptance Criteria:**
- Must hide the complexity of Stripe Billing behind a simple "Enable Subscription" toggle.
- Must include a clean, accessible portal for customers to manage their own subscriptions.
- Must gracefully handle failed payments.

## 5. Priority
`P2`

## 6. Estimated Scope
Medium
