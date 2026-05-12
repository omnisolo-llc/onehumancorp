# Deep Dive: Integration Data Model Mapping Strategy

## Executive Summary
This document serves as an appendix to the Q3 Tool Integration Research report. It details the specific strategies required to normalize the disparate data models of our target integrations (Meta, Google Calendar, Stripe, Mailchimp, etc.) into the canonical One Human Corp (OHC) schema. This normalization is critical to ensure the core OHC codebase remains agnostic to the specific third-party tools the user has chosen to connect.

## 1. The Core OHC Canonical Models

Before we can map external data, we must define the target structures. The OHC platform relies on the following canonical structs:

### `OhcContact`
Represents a human being or a business entity interacting with the OHC tenant.
- `id`: UUID (Internal)
- `first_name`: String
- `last_name`: String
- `email`: String (Unique per tenant)
- `phone_number`: String (E.164 formatted)
- `marketing_opt_in`: Boolean
- `timezone`: String (IANA format)

### `OhcEvent`
Represents a scheduled block of time on the calendar.
- `id`: UUID (Internal)
- `title`: String
- `start_time`: DateTime (UTC)
- `end_time`: DateTime (UTC)
- `participant_ids`: List[UUID] (References `OhcContact`)
- `status`: Enum (SCHEDULED, CANCELLED, COMPLETED)
- `meeting_url`: Optional[String]

### `OhcMessage`
Represents a discrete piece of communication.
- `id`: UUID (Internal)
- `sender_id`: UUID (References `OhcContact` or internal User)
- `content`: String
- `timestamp`: DateTime (UTC)
- `channel`: Enum (SMS, EMAIL, INSTAGRAM_DM, FACEBOOK_MESSAGE, WHATSAPP)
- `external_thread_id`: String (Used for correlating replies)

## 2. Mapping Strategies by Category

### Social Media (Meta / WhatsApp / ManyChat)

**The Challenge:** Meta returns deeply nested JSON structures for messages, often containing disparate fields for text, images, and quick replies. WhatsApp templates require specific variable mappings.

**Mapping to `OhcMessage`:**
1.  **Ingestion:** When the Meta webhook hits the `/webhooks/meta` endpoint, the raw payload is pushed to Redis.
2.  **Transformation:** The `MetaWebhookWorker` parses the payload.
3.  **Contact Resolution:** It extracts the `sender.id` from the Meta payload. It queries the `oauth_identities` table to find an `OhcContact` linked to that specific Meta ID. If none exists, it creates a new "stub" `OhcContact` with the available public profile data (usually just a first name).
4.  **Message Normalization:** It maps the Meta `message.text` to `OhcMessage.content`. It maps the Meta `timestamp` (often Unix epoch milliseconds) to a UTC DateTime. It sets `channel` to `INSTAGRAM_DM`.
5.  **Persistence:** The `OhcMessage` is saved to the SQLite/Postgres database.

### Calendar Sync (Google Calendar / Outlook)

**The Challenge:** Google Calendar uses a complex recurrence model (RRULE). Outlook uses a slightly different recurrence model. Both allow for single-instance exceptions (e.g., "Cancel just this Tuesday's meeting").

**Mapping to `OhcEvent`:**
1.  **Polling/Push:** OHC receives a notification that a user's calendar has changed.
2.  **Expansion:** Instead of attempting to normalize the complex RRULE strings across different providers, the OHC sync worker uses the provider's API to request the "expanded" instances of the events for the next 90 days.
3.  **Upsert Logic:** The worker iterates through the expanded events.
    -   If an event's `external_id` (the Google Event ID) already exists in OHC, it updates the `start_time`, `end_time`, and `title`.
    -   If it does not exist, it creates a new `OhcEvent`.
    -   Crucially, if an event exists in OHC (linked to Google) but is *missing* from the Google API response, the worker must mark the `OhcEvent` as `CANCELLED` (soft delete) rather than hard deleting it, preserving historical data.

### Email Marketing (Mailchimp / Loops)

**The Challenge:** Mailchimp uses the concept of "Audiences" and "Tags". Loops uses "Contacts" and "Custom Events".

**Mapping from `OhcContact`:**
1.  **Trigger:** An `OhcContact` is updated (e.g., they check the "Subscribe to newsletter" box during checkout).
2.  **Adapter Pattern:** The system routes this event to the active Email Marketing adapter (e.g., `MailchimpAdapter`).
3.  **Transformation:** The adapter translates the `OhcContact` into the provider's format. For Mailchimp, it maps `email` to `email_address`, and `first_name`/`last_name` to the specific `merge_fields` configured in the user's Mailchimp account.
4.  **Tagging:** The adapter maps OHC internal tags (e.g., "VIP Customer") to Mailchimp Tags, ensuring the business owner can easily segment their audience on the third-party platform.

### Payments (Mercado Pago / Razorpay)

**The Challenge:** Webhook formats vary wildly. Stripe sends `payment_intent.succeeded`. Mercado Pago sends a notification that an `action` occurred, requiring OHC to make a secondary API call to fetch the actual payment details.

**Mapping to Internal Invoices:**
1.  **Idempotency Key:** When OHC initiates a checkout session, it passes the internal OHC `Invoice ID` as the `reference_id` or `metadata` field to the payment provider.
2.  **Webhook Ingestion:** The webhook arrives. For Mercado Pago, the worker makes the secondary `GET /v1/payments/{id}` call to fetch the status.
3.  **State Machine:** The worker extracts the `reference_id` (our Invoice ID) and the `status` (e.g., `approved`, `rejected`).
4.  **Normalization:** It translates the provider's status into the internal OHC invoice state (`PAID`, `FAILED`). It then safely updates the database, utilizing a distributed lock to prevent race conditions if duplicate webhooks arrive simultaneously.

## 3. Handling API Deprecations and Versioning

The DNL provides a massive advantage when third-party APIs change.

If Meta deprecates v17.0 of the Graph API and mandates migration to v18.0, the core OHC business logic remains entirely untouched. The engineering team only needs to update the specific `MetaWebhookWorker` adapter.

To manage this safely, the codebase should employ versioned adapters:
`src/integrations/meta/v17/webhook_parser.go`
`src/integrations/meta/v18/webhook_parser.go`

During the migration window, feature flags can be used to route a small percentage of incoming webhooks to the v18 parser, verifying its correctness against the canonical `OhcMessage` struct before rolling it out to all tenants.

## 4. Conclusion
The Data Normalization Layer is not merely a structural convenience; it is the architectural firewall that protects the One Human Corp codebase from the chaos of the external SaaS ecosystem. By strictly enforcing these mapping strategies, we guarantee that the platform remains stable, testable, and scalable regardless of which third-party tools the user decides to connect.
