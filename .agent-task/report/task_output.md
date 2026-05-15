# [SMS & Notifications] Integrate Twilio for Automated Customer SMS Notifications

**Title**: [SMS & Notifications] Integrate Twilio for Automated Customer SMS Notifications

**Problem Statement**: Small business owners often struggle to communicate efficiently with customers, especially those who don't check email regularly or prefer immediate updates. Customers rely heavily on SMS for appointment reminders, order readiness, and shipping updates. Without automated SMS capabilities, businesses face higher no-show rates, missed communications, and increased manual work, directly impacting revenue and customer satisfaction. This tool allows the business owner to effortlessly reach customers where they are.

**Research Report**: I evaluated Twilio, MessageBird, and Plivo. Twilio is the recommended choice due to its unmatched global carrier coverage, high deliverability, and reliability. For small business owners, dropped messages or delayed notifications can damage their reputation, making reliability the top priority. Twilio supports both Cloud (where OHC manages the centralized multi-tenant account) and Standalone modes (where business owners can input their own Twilio API credentials). The pricing is affordable, roughly $0.0079 per message in the US, making it viable even for small businesses. The main risk involves ensuring compliance with local opt-out (STOP) regulations, which Twilio provides tools to handle. For the non-technical OHC user, the complexity is entirely hidden behind a simple "Enable SMS Notifications" toggle.

**Design Doc**: Within the OHC dashboard, a new "SMS Notifications" section will be added. The business owner will see simple toggles to enable SMS for specific events like "Appointment Reminder" and "Order Shipped". In Cloud mode, OHC uses its central Twilio account to send the messages, attributing usage to the specific tenant. In Standalone mode, the owner is prompted to enter their Twilio Account SID and Auth Token. When a relevant domain event (e.g., `OrderShipped`) is triggered, the system checks the business's SMS preferences, formats a short text message, and calls the SMS sending service. Opt-outs and delivery failures will be surfaced simply as "Message not delivered" or "Customer unsubscribed" in the UI.

**Implementation Prompt**: Add a new "SMS Settings" page to the owner's dashboard following the Grandmother Test UX standard—use clear, plain language with simple toggles for notification types. Do not use technical jargon. For Standalone users, provide simple input fields for Twilio credentials. Hook into existing business events (like orders and appointments) to check the owner's preferences and send an SMS to the customer's phone number if enabled. Implement daily sending limits to prevent unexpected costs or accidental spam, and ensure opt-outs are respected automatically.

**Priority**: P1

**Estimated Scope**: Medium
