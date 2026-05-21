# [marketing] Klaviyo Automation Integration

## Title
Enable Advanced E-commerce Marketing Automation with Klaviyo

## Problem Statement
Small business owners, especially those running e-commerce shops (like our persona Maya), struggle to convert one-time buyers into repeat customers. Traditional mass-email tools (like Mailchimp) are often too generalized and fail to leverage real customer behavior (e.g., abandoned carts, specific product views, or purchase history). SMB owners need a marketing tool that automatically triggers highly personalized email and SMS campaigns based on specific shop activities without requiring them to manually segment lists or build complex data pipelines. They want a "set it and forget it" system that recovers lost sales and drives loyalty using data they already generate on OHC.

## Research Report
**Market Discovery & Competitor Analysis:**
Klaviyo is widely recognized as the premier marketing automation platform tailored specifically for e-commerce. It frequently ranks as a top-installed app on platforms like Shopify and Wix because of its deep integration with store data.

**Key Findings:**
1. **Target Audience Fit:** Klaviyo is explicitly built for data-driven e-commerce operators. It excels where competitors like Mailchimp fall short—specifically in advanced segmentation based on transactional data (e.g., "customers who bought X but not Y in the last 30 days").
2. **Feature Set for Non-Technical Users:** While its data capabilities are advanced, Klaviyo provides pre-built "Flows" (automations) like Abandoned Cart, Welcome Series, and Win-Back campaigns that SMB owners can activate with a few clicks.
3. **Omnichannel:** It bundles both Email and SMS marketing into a single platform, saving SMBs the hassle of managing separate tools for text message marketing.
4. **Developer Ecosystem:** Klaviyo supports modern authentication (managed OAuth) and real-time event syncing via Webhooks, meaning our integration can instantly push customer actions (page views, checkout started, order placed) to Klaviyo without delay or manual polling.
5. **Pricing & Viability:** Klaviyo offers a Free plan (up to a certain number of contacts/emails) which is perfect for new or very small businesses to start. As the business scales, pricing is contact-based. This model aligns perfectly with OHC's goal of supporting businesses as they grow. It operates as a SaaS platform but can securely connect via API to our Cloud (multi-tenant) and Standalone environments.

## Design Doc
**Integration High-Level Architecture:**
The integration will act as a bridge between the OHC unified event stream and Klaviyo's API.

1. **Triggers (OHC -> Klaviyo):**
   - When a user signs up on an OHC storefront, a profile is created in Klaviyo.
   - Key transactional events (e.g., `Viewed Product`, `Added to Cart`, `Started Checkout`, `Placed Order`) triggered in OHC will automatically dispatch an event payload to Klaviyo in real time.
2. **Actions (Klaviyo -> OHC):**
   - OHC will listen for key Klaviyo webhooks (e.g., `Unsubscribed`, `Email Bounced`) to ensure OHC's internal customer CRM maintains accurate contact preferences and complies with anti-spam laws.
3. **User Experience (SMB Owner):**
   - In the OHC dashboard, under "Marketing Integrations", the owner clicks "Connect to Klaviyo" and completes a standard OAuth flow.
   - Once connected, OHC automatically provisions baseline lists (e.g., "Master Newsletter", "Customers") and begins syncing historical and live customer data.
   - The owner logs into Klaviyo to design their emails using pre-populated OHC product data (images, prices, URLs).

## Implementation Prompt
**Goal:** Build a robust, real-time data sync between OHC and Klaviyo.
**Acceptance Criteria:**
- The SMB owner can connect their OHC account to Klaviyo using an OAuth flow via the OHC integrations dashboard.
- A toggle exists to enable/disable the automatic syncing of customer profiles and catalog data.
- When enabled, OHC standard e-commerce events (`Viewed Product`, `Added to Cart`, `Checkout Started`, `Order Completed`, `Order Cancelled`) are reliably sent to Klaviyo associated with the correct customer profile.
- The integration handles API rate limits gracefully (retries).
- Opt-in and opt-out preferences are synchronized bidirectionally.
- The implementer is free to design the specific event schema, webhook handlers, and background queueing mechanism.

## Priority
P1 (High) - Directly impacts revenue generation for e-commerce personas.

## Estimated Scope
Medium
