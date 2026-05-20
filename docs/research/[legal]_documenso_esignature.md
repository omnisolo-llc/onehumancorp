# Title: Integration with Documenso for Automated Contract and Document E-Signatures

## Problem Statement
Small business owners (like service providers, consultants, contractors, or event organizers) frequently need clients to sign quotes, service agreements, or liability waivers before work begins. Relying on manual PDFs, physical printing, or expensive enterprise e-signature tools (like DocuSign) adds friction to the sales process, increases administrative overhead, and leads to dropped conversions. Business owners need a simple, integrated way to get legally binding signatures without leaving their primary operating platform.

## Research Report
Documenso is an open-source, next-generation alternative to DocuSign that offers seamless and secure electronic signatures. It is designed to be highly customizable, cost-effective, and transparent. Crucially, Documenso supports both Cloud (SaaS) and Standalone (self-hosted) environments, perfectly aligning with OHC's dual deployment models.

- **Ease of Use**: For clients, signing a document is a simple, mobile-friendly process that requires no account creation. For the business owner, tracking signature status is straightforward and visual.
- **Pricing**: Documenso offers a generous free tier for early-stage businesses, affordable pro plans for growing teams, and a fully self-hostable version that allows businesses to run it internally with minimal marginal costs per document.
- **Reputation**: It has a strong reputation in the open-source community as a modern, trustworthy, and performant product that respects user data and avoids the restrictive vendor lock-in of traditional legacy providers.

## Design Doc
When a small business owner finalizes a quote, agreement, or intake form within OHC, they will see a "Request E-Signature" action. Upon triggering this action, OHC will compile the document (e.g., into a PDF), securely pass it to Documenso, and generate a secure signing link.

This link will be sent to the client via SMS or email using OHC's existing unified communication channels. Within the OHC dashboard, the business owner will see the real-time status of the document (e.g., "Sent", "Viewed", "Signed"). Once the client signs the document, Documenso will notify OHC, and a copy of the fully executed document will be automatically saved and attached to the client's CRM profile and the corresponding job/order record.

## Implementation Prompt
Create an integration with Documenso to allow OHC users to request and track electronic signatures for their business documents directly from the platform.

**Acceptance Criteria:**
- Business owners can initiate a signature request directly from an existing quote, invoice, or document record.
- Clients receive a secure signing link via email/SMS and can sign on any device without friction.
- The signature status (Sent, Viewed, Signed) is accurately reflected in the OHC dashboard.
- The finalized, signed document is automatically retrieved and stored within the relevant customer and order records in OHC.
- The integration must gracefully handle both OHC Cloud (connecting to Documenso's managed SaaS API) and OHC Standalone (allowing the owner to input credentials for their self-hosted Documenso instance).

## Priority
P1

## Estimated Scope
Medium
