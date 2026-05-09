# [email-marketing] Integrated Customer Newsletter & Marketing

**Title:** Integrate Email Marketing and Customer List Sync

**Problem Statement:**
Small business owners collect customer emails through sales and bookings but often fail to leverage this list for marketing. Manually exporting CSVs to tools like Mailchimp is tedious and prone to error, preventing them from sending promotions, newsletters, or seasonal updates easily.

**Research Report:**
* **Tools Evaluated:** Resend, SendGrid, Mailchimp API.
* **Ease of Use:** Developer-first tools like Resend offer very clean APIs to send transactional and bulk emails, allowing OHC to build a simplified, native email builder interface.
* **Key Advantages:**
  - High deliverability rates.
  - No need for the user to manage a separate Mailchimp account or understand complex list segmentation.
  - Native integration means customer lists in OHC are always perfectly synced.
* **Risks:**
  - Strict anti-spam compliance rules (CAN-SPAM, GDPR) require careful implementation of unsubscribe links and domain verification.
* **Pricing Estimate:** Free up to 3,000 emails/month (Resend); scalable PAYG after that.
* **Environment Support:** fully supported in Cloud mode. Standalone mode can connect to the API via external network requests.

**Design Doc:**
* **Trigger:** The user goes to the "Marketing" tab and selects "Create Email Campaign".
* **Actions:** OHC provides a simplified rich-text editor. The user selects their audience (e.g., "All Customers" or "Recent Customers"). OHC dispatches the emails via the integrated API provider and tracks open rates.
* **User Experience:** The owner sees a simple compose window, without the overwhelming features of a full CRM. They can view basic stats (sent, opened, clicked) directly on their dashboard.

**Implementation Prompt:**
Implement a lightweight email marketing feature within the OHC dashboard. Use an email delivery API to allow users to send bulk updates to their OHC customer list. Provide a simple WYSIWYG editor for composing messages, automatically inject mandatory unsubscribe links, and display basic campaign performance metrics (open rates).

**Priority:** P2
**Estimated Scope:** Medium