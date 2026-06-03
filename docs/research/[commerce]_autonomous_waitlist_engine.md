# [commerce] Autonomous Omnichannel Pre-Order and Waitlist Engine

## Problem Statement
Small businesses, especially those launching new or limited-quantity products, often face demand that exceeds their current capacity. They need a way to capture customer interest across various channels (like Instagram DMs or their website) and manage fulfillment smoothly when items become available, without the overhead of tracking lists manually.

## Research Report
Current platforms either lack robust waitlist functionality or require complex third-party apps that don't integrate cleanly with the core storefront. Many SMBs end up using spreadsheets to track pre-orders from different sources (social media, email, in-person). Our target audience needs an integrated solution where an AI agent can automatically handle waitlist sign-ups, answer capacity-related queries, and process conversions when inventory is added.

## Design Doc
- **Architecture**: A new `WaitlistEngine` component intercepts requests when stock is low or an item is in "Pre-Order" mode.
- **Data Model**:
  - `WAITLIST_CAMPAIGN`: Tracks the overall waitlist parameters (e.g., max capacity, product ID).
  - `PRE_ORDER_ENTRY`: Records individual customer entries, including their source channel and timestamp. Enforces row-level security using `tenant_id`.
- **AI Agent Responsibilities**:
  - *Promoter*: Manages the visibility of the waitlist on social media and the storefront.
  - *Ambassador*: Interacts with customers via channels like IG DM to capture pre-orders and answer status queries.
  - *Manager*: Monitors inventory and automatically triggers fulfillment/payment flows when stock arrives.
- **UI/UX**:
  - Clear "Join Waitlist" or "Pre-Order" buttons replacing "Add to Cart" when appropriate.
  - Dashboard widget for the business owner showing waitlist size and potential revenue.
  - Mobile-first approach, ensuring the entire management flow is accessible on a 375px screen.

## Implementation Prompt
Implement the data models `WAITLIST_CAMPAIGN` and `PRE_ORDER_ENTRY` with proper RLS. Build the `WaitlistEngine` core logic to handle incoming waitlist requests and integrate with the existing inventory management system. Create the necessary UI components for the waitlist sign-up flow, adhering to OHC Glassmorphism design tokens. Ensure the Ambassador agent can seamlessly transition a DM conversation into a waitlist entry. Write full test coverage, including unit tests for the data models and E2E tests for the sign-up flow.
