issue_title: "✍️ Scribe: [Documentation for Proposals and Payment Requests]"
issue_priority: "P2"
issue_category: "documentation"
issue_type: "feature"
issue_label: "ohc:lane:documentation"
assignees: []
issue_description: |
  # How to Send Proposals and Collect Payments Securely

  **Problem Statement:**
  As a small business owner, it's critical to know that the proposals you send to your clients contain the exact scope and pricing you agreed upon, and that payment links are secure and verifiable.

  **Current Capabilities:**
  - **No More Placeholder Proposals:** OmniSolo ensures that the quote generated directly reflects the customer's inquiry and your explicit business rules. We validate owner-supplied line items and deposits with checked math. If pricing information is missing, the system will mark it as `NEEDS_PRICING` for your review. Optional, unselected items are strictly excluded from committed totals.
  - **Secure Checkout Links:** When you create an invoice, OmniSolo generates a real, secure session with your connected payment provider (like Stripe). It will not generate fictitious checkout URLs.
  - **Drafts vs. Sent Reminders:** The system clearly distinguishes between drafting a reminder and actually delivering it. We only persist actual, source-grounded drafts.
  - **Your Authority:** These actions operate under your standing authority. The system enforces your hard spend reservations.

  **How to Use This Feature:**
  1. Open a Lead or Inquiry in the OmniSolo app.
  2. Click **Generate Proposal**. Review the line items carefully. If any items are marked `NEEDS_PRICING`, fill in the correct amounts.
  3. Approve the proposal to finalize the scope and price.
  4. When ready, click **Create Invoice**. OmniSolo will securely connect to your payment provider to generate a verifiable checkout link.
  5. The resulting email/SMS draft will be placed in your outbox for final review before sending.

  **Cost & Expectations:**
  These actions use your regular OHC subscription. There are no hidden markup fees on your customer's invoice. Note that standard payment processor fees still apply.

  **Exceptions and Recovery:**
  If your payment provider disconnects or fails to create a link, the invoice status will remain "Draft/Pending Provider". You can simply try generating the link again later.

  *Superpowers workflow applied: using-superpowers, brainstorming, writing-plans, executing-plans. Target issue: F07, F08 (Proposals and checkout links).*
