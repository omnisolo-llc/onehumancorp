# Scout: Tool Integration Research

# Unified Inbox Integration via Meta Graph API

**Title**: Unified Inbox Integration via Meta Graph API
**Problem Statement**: Small business owners like Fatima receive customer inquiries across Instagram, Facebook, and WhatsApp, often missing messages and losing sales because they can't monitor all apps at once. They need a single place to view and respond to all customer messages.

**Research Report**:
- The Meta Graph API provides comprehensive access to Instagram DMs, Facebook comments, and WhatsApp messages.
- It has widespread adoption, making it the standard for social media integrations.
- **Ease of Use**: Once connected via a simple OAuth flow, the business owner doesn't need to interact with Meta's developer tools.
- **Pricing**: Free for standard usage; WhatsApp Business API has per-conversation pricing after the first 1,000 free tier.
- **Reputation**: Highly reliable, though subject to strict review processes.
- **Cloud vs Standalone**: Works in Cloud mode well. In Standalone mode, users might need to provide their own API credentials or use an OHC proxy.
- **Key Advantages**: Unifies the most popular communication channels.
- **Key Risks**: Meta's strict review processes and API changes.

**Design Doc**:
- The user navigates to the "Communications" tab and clicks "Connect Social Accounts."
- They are redirected to Meta's secure login to authorize OHC.
- Once connected, a "Unified Inbox" widget aggregates all incoming messages, showing the source icon next to each message. The user can reply directly from the widget, and OHC routes it back to the correct platform.

**Implementation Prompt**: Create a unified inbox interface where users can authenticate their Meta accounts (Instagram, Facebook, WhatsApp) and seamlessly read and reply to messages from one centralized dashboard.

**Priority**: P0
**Estimated Scope**: Large

---

# Integrate Cal.com for Zero-Config Booking & Calendar Sync

**Title**: Integrate Cal.com for Zero-Config Booking & Calendar Sync
**Problem Statement**: Service providers like Carlos the Handyman lose customers due to back-and-forth scheduling via text. They need a public booking link that syncs automatically with their personal Google or Outlook Calendar.

**Research Report**:
- Cal.com is an open-source scheduling infrastructure that handles timezone math, calendar conflict resolution, and booking pages natively.
- **Ease of Use**: Highly intuitive for non-technical users to set their availability.
- **Pricing**: Free tier available for individuals.
- **Reputation**: Strongly respected open-source alternative to Calendly.
- **Cloud vs Standalone**: Perfectly compatible with both Cloud (SaaS) and Standalone OHC modes due to its open-source and embeddable nature.
- **Key Advantages**: Eliminates scheduling friction, free tier suitable for SMBs.
- **Key Risks**: Requires reliable synchronization with third-party calendars.

**Design Doc**:
- Users connect their calendar via a one-click OAuth button in the "Operations" tab.
- "The Manager" AI sets up the booking link dynamically based on the user's defined business hours.
- A public booking widget is displayed on their OHC storefront. When a customer books a slot, Cal.com manages the calendar event and conflict resolution transparently.

**Implementation Prompt**: Embed Cal.com's infrastructure so users can sync their personal calendars and provide a public booking widget on their storefront that prevents double-booking.

**Priority**: P0
**Estimated Scope**: Medium

---

# Simple Email Campaigns via Resend Integration

**Title**: Simple Email Campaigns via Resend Integration
**Problem Statement**: Small business owners need an easy way to send newsletters, promotions, or updates to their customer list without learning complex marketing platforms like Mailchimp.

**Research Report**:
- Resend is a developer-friendly email platform with a strong focus on deliverability and simplicity.
- **Ease of Use**: OHC can abstract the complexity, allowing the user to simply write an email in a rich text editor and click "Send to all customers."
- **Pricing**: Generous free tier (up to 3,000 emails/month), which covers most SMB needs.
- **Reputation**: Excellent deliverability and modern API.
- **Cloud vs Standalone**: Works seamlessly in Cloud mode. Standalone mode might require users to input their own API key.
- **Key Advantages**: High deliverability, simple abstraction for end-users.
- **Key Risks**: Strict spam compliance rules could lead to account suspensions if users abuse the system.

**Design Doc**:
- Users access the "Marketing" tab and select "New Campaign."
- They write their message using a simple editor and select their target audience (e.g., "All Customers" or "Recent Buyers").
- OHC handles the distribution via Resend, displaying basic open-rate analytics back to the user in a digestible format.

**Implementation Prompt**: Integrate Resend to allow business owners to compose and send email campaigns directly to their customer lists, and display basic analytics (open rates, clicks) within the dashboard.

**Priority**: P1
**Estimated Scope**: Medium

---

# LATAM Payment Processing Integration via Mercado Pago

**Title**: LATAM Payment Processing Integration via Mercado Pago
**Problem Statement**: Users in Latin America need a reliable, localized way to accept payments, as global providers like Stripe are not universally accessible or preferred by their customers.

**Research Report**:
- Mercado Pago is the dominant payment processor in LATAM, supporting local payment methods (like Pix in Brazil or OXXO in Mexico).
- **Ease of Use**: Familiar to the target demographic, straightforward onboarding for merchants.
- **Pricing**: Competitive local rates, no monthly fees.
- **Reputation**: Highly trusted across Latin America.
- **Cloud vs Standalone**: Fully supported in both modes via API integrations.
- **Key Advantages**: Unlocks the LATAM market by supporting essential local payment methods.
- **Key Risks**: Varying settlement speeds and currency fluctuations across different countries.

**Design Doc**:
- Users in supported regions see Mercado Pago as a payment option in the "Settings > Payments" area.
- They authenticate their existing Mercado Pago account or create a new one.
- The OHC storefront checkout seamlessly redirects or embeds the Mercado Pago checkout flow, returning the user to a success page upon completion.

**Implementation Prompt**: Add Mercado Pago as a native payment option for LATAM users, enabling them to accept local payment methods effortlessly on their storefronts.

**Priority**: P0
**Estimated Scope**: Large

---

# Automated Shipping Label Generation with Shippo

**Title**: Automated Shipping Label Generation with Shippo
**Problem Statement**: E-commerce sellers waste hours manually calculating shipping rates at the post office and writing labels by hand. They need an automated way to generate shipping labels directly from their orders.

**Research Report**:
- Shippo provides a unified API connecting to dozens of global carriers (USPS, UPS, FedEx, DHL, etc.).
- **Ease of Use**: OHC can provide a "Buy Label" button directly on the order details page, abstracting the carrier integrations.
- **Pricing**: Pay-as-you-go model (per label fee), which is very SMB-friendly without monthly commitments.
- **Reputation**: Reliable and widely used in the e-commerce space.
- **Cloud vs Standalone**: Compatible with both modes.
- **Key Advantages**: Broad carrier coverage, negotiated discount rates available out-of-the-box.
- **Key Risks**: International customs forms can still be complex for users to fill out accurately.

**Design Doc**:
- On the "Orders" page, an unfulfilled order displays a "Create Shipping Label" button.
- The system automatically pulls the package dimensions and weight (if saved) and displays real-time carrier rates.
- The user selects a rate, purchases the label, and the system generates a printable PDF and automatically emails the tracking link to the customer.

**Implementation Prompt**: Integrate Shippo to allow users to compare real-time shipping rates, purchase labels directly from their order dashboard, and automatically update customers with tracking information.

**Priority**: P1
**Estimated Scope**: Large

---

# Global SMS Notifications for Critical Updates via Twilio

**Title**: Global SMS Notifications for Critical Updates via Twilio
**Problem Statement**: Non-technical or low-English-proficiency users, as well as their customers, often prefer SMS over email for critical updates like order confirmations or appointment reminders.

**Research Report**:
- Twilio is the industry standard for programmatic SMS delivery globally.
- **Ease of Use**: The business owner simply toggles "Enable SMS Notifications" in their settings. OHC handles the routing.
- **Pricing**: Pay-per-message model; very affordable for standard notification volumes.
- **Reputation**: Extremely high reliability and global reach.
- **Cloud vs Standalone**: Cloud mode can pool usage, while Standalone mode requires the user to plug in their own Twilio credentials.
- **Key Advantages**: Near-instant delivery, high open rates compared to email.
- **Key Risks**: Strict A2P 10DLC compliance rules in the US can make onboarding complex if not abstracted properly.

**Design Doc**:
- In "Settings > Notifications," users can toggle SMS alerts for new orders or booking reminders.
- Customers providing their phone numbers at checkout automatically receive SMS updates.
- OHC abstracts the Twilio integration, ensuring messages are sent strictly for transactional purposes to comply with carrier regulations.

**Implementation Prompt**: Implement a Twilio integration that allows business owners to seamlessly send automated, transactional SMS updates (e.g., order confirmed, appointment tomorrow) to their customers.

**Priority**: P1
**Estimated Scope**: Medium

---

# Auto-Generate Consultation Links via Zoom

**Title**: Auto-Generate Consultation Links via Zoom
**Problem Statement**: Consultants and tutors need a way to automatically generate and send video meeting links when a client books a session, avoiding manual link creation.

**Research Report**:
- Zoom is the ubiquitous video conferencing tool globally recognized by almost all consumers.
- **Ease of Use**: Users connect their Zoom account once; meeting links are generated transparently.
- **Pricing**: Free tier limits meetings to 40 minutes, which users must be aware of. Pro tier covers longer sessions.
- **Reputation**: Highly reliable, though some users may prefer browser-based alternatives like Google Meet.
- **Cloud vs Standalone**: Works well in both modes via OAuth.
- **Key Advantages**: High consumer familiarity, robust connection quality.
- **Key Risks**: The 40-minute limit on free accounts might cut off user consultations unexpectedly.

**Design Doc**:
- Users connect their Zoom account in the "Integrations" tab via a standard OAuth flow.
- When configuring a service (e.g., "1-Hour Consultation"), they select "Zoom Meeting" as the location.
- Upon booking, OHC automatically calls the Zoom API to generate a meeting link, which is instantly added to the calendar invite and confirmation email sent to both parties.

**Implementation Prompt**: Create a Zoom integration that allows users to seamlessly connect their accounts and automatically generate unique meeting links for newly booked online services.

**Priority**: P2
**Estimated Scope**: Medium

---


## Proposed Next Steps
- Review P0 integrations (Meta Graph API, Cal.com, Mercado Pago) for immediate technical feasibility.
- Begin implementation sprints for High-priority (P1) integrations to increase platform value.
- Ensure all integrations are tested with non-technical users to validate the 'User-First Lens'.