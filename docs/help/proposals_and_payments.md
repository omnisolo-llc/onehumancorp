# OmniSolo Help Center: Managing Quotes, Invoices, and Approvals

## Title: How to Send Proposals and Collect Payments Securely

**Problem Statement:**
As a small business owner, it's critical to know that the proposals you send to your clients contain the exact scope and pricing you agreed upon, and that payment links are secure and verifiable.

**Research Report / Current Capabilities:**
Based on our latest updates (referencing the current active capability map), OmniSolo now enforces stricter controls on how proposals and invoices are generated:

- **No More Placeholder Proposals:** OmniSolo ensures that the quote generated directly reflects the customer's inquiry and your explicit business rules. We validate owner-supplied line items and deposits with checked math. If pricing information is missing, the system will mark it as `NEEDS_PRICING` for your review, rather than guessing or fabricating a fixed amount. Optional, unselected items are strictly excluded from committed totals.
- **Secure Checkout Links:** When you create an invoice, OmniSolo generates a real, secure session with your connected payment provider (like Stripe). It will not generate fictitious checkout URLs. If a provider is unavailable, it will explicitly state the pending/unavailable status, ensuring you never send a broken link to a client.
- **Drafts vs. Sent Reminders:** The system clearly distinguishes between drafting a reminder and actually delivering it. We only persist actual, source-grounded drafts. Once an invoice is paid or canceled, stale drafts are automatically retired so you don't accidentally ask a customer twice.
- **Your Authority:** These actions operate under your standing authority. The system enforces your hard spend reservations, prevents unauthorized effect on your accounts, and allows you to revoke or stop actions at any time.

**How to Use This Feature:**
1. Open a Lead or Inquiry in the OmniSolo app.
2. Click **Generate Proposal**. Review the line items carefully. If any items are marked `NEEDS_PRICING`, fill in the correct amounts.
3. Approve the proposal to finalize the scope and price.
4. When ready, click **Create Invoice**. OmniSolo will securely connect to your payment provider to generate a verifiable checkout link.
5. The resulting email/SMS draft will be placed in your outbox for final review before sending.

**Cost & Expectations:**
These actions use your regular OHC subscription. There are no hidden markup fees on your customer's invoice. Note that standard payment processor fees (like Stripe's transaction fee) still apply and are managed directly in your provider dashboard.

**Exceptions and Recovery:**
If your payment provider disconnects or fails to create a link, the invoice status will remain "Draft/Pending Provider". You can simply try generating the link again later; the system is designed to retry safely without creating duplicate charges.

**Final Evidence & Skill Provenance:**
- Loaded Skill: `superpowers/skills/using-superpowers/SKILL.md` (via upstream Github cloning).
- Executed `cargo check` and examined test structure.
- Examined the active audit ledger and capability map in `RESEARCH.md` and `docs/research/business_capability_and_usage_economics_audit.md`.

---
*Scope: Documentation for Inquiry/Proposal and Final Payment workflows.*
