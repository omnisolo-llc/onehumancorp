# Email Marketing Integration

**Problem Statement:**
Small business owners accumulate customer contact information but have no simple way to send professional newsletters, promotions, or updates. Tools like Mailchimp are often too complex and expensive for basic needs.

**Research Report:**
* **Tool Evaluated:** Resend API
* **Ease of Use:** Developer-friendly API allows OHC to abstract away all complexity, providing a dead-simple text editor for the business owner.
* **Pricing:** Very generous free tier (3,000 emails/mo), then affordable scaling.
* **Reputation:** Modern, reliable, and focused on deliverability.
* **Hybrid Context:** Cloud mode can use shared/managed API keys. Standalone mode might require the user to provide their own API key or rely on an OHC proxy service.

**Design Doc:**
* **Trigger:** The business owner selects "Create Campaign" from the Customer list view.
* **Action:** The owner writes a message, and OHC uses Resend to deliver it to the selected customer segments.
* **User Experience:** The owner sees a simple compose window (like writing a standard email). They select "All Customers" or "Recent Buyers," click send, and see a basic report of how many people opened it.

**Implementation Prompt:**
Create a "Broadcast" feature within the CRM module. Allow the user to draft a plain-text or rich-text email and send it to a selected list of their contacts. The UI should display basic status (Sent, Delivered, Opened) without overwhelming the user with complex marketing metrics.

**Priority:** P2
**Estimated Scope:** Medium
