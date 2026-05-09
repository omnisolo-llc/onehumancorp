# Email Marketing: Customer Campaigns

## Problem Statement
Business owners want to send promotions or updates to their existing customer list, but tools like Mailchimp are too expensive, complex, and disconnected from where their customer data lives.

## Research Report
**Selected Tools:** Resend (Cloud) & Listmonk (Standalone)
We evaluated options for both operating modes of OHC. Resend offers a fantastic developer experience for the Cloud, while Listmonk is ideal for self-hosted Standalone deployments.
- **Ease of use for non-technical users:** The complexity must be abstracted. OHC will provide a simple text editor and handle the heavy lifting of lists and delivery.
- **Pricing:** Resend offers generous tiers; Listmonk is open source (free).
- **Reputation:** Both are highly respected in the developer community for reliability.

## Design Doc
**Integration with OHC:**
- **Trigger:** Owner creates a "Campaign" in OHC and clicks "Send".
- **Action:** OHC compiles the target customer list, renders the email template, and queues it via the Resend API (Cloud) or local Listmonk instance (Standalone).
- **User Interface:** A simple composer interface (like writing a regular email) with audience selection (e.g., "All past customers", "Recent buyers").
- **Environment:** Cloud (Resend); Standalone (Listmonk).

## Implementation Prompt
**User-Facing Outcome:** The owner can draft a promotional message and send it to their entire customer list with one click, right from OHC.
**Acceptance Criteria:**
- Simple email draft interface (no complex drag-and-drop builders initially).
- Automatic handling of unsubscribe links to ensure compliance.
- Basic reporting (how many were delivered/opened).

## Priority
P2

## Estimated Scope
Medium
