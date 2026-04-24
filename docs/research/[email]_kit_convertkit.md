# Scout: Email Marketing (Kit / ConvertKit)

## Title
Creator-Focused Email Automation 📧 (Kit API Integration)

## Problem Statement
Small business owners like Leo (the music tutor) and Maya (the baker) are essentially creators. They need to build a "tribe" of loyal customers through storytelling and regular updates. Standard email tools are often too clinical or too complex. Kit (formerly ConvertKit) is designed specifically for creators, focusing on ease of use, high deliverability, and powerful automation sequences that nurture leads into long-term customers.

## Research Report
- **Goal**: Evaluate Kit (ConvertKit) API v3/v4 as the primary email engine for OHC's Marketing department.
- **Features evaluated**:
  - **Broadcasts & Sequences**: Simple APIs to send one-off newsletters or multi-day automated journeys.
  - **Subscriber Management**: Robust tagging and segmentation logic.
  - **Forms & Landing Pages**: Easily integrate OHC lead capture forms with Kit.
  - **Creator Network**: Potential for OHC users to cross-promote with other creators.
- **Benefits for OHC users (Non-technical)**:
  - Clean, writing-focused interface that doesn't overwhelm.
  - Automation templates (e.g., "Welcome Sequence", "Post-Purchase Follow-up") that work out of the box.
  - High deliverability ensures their emails actually land in the Inbox, not Spam.
- **Integration Risks**:
  - Kit API v4 is in beta; v3 is the current stable standard.
  - Managing email list hygiene (unsubscribes) must be perfectly synced between OHC and Kit.
- **Pricing**: Generous free tier (up to 1,000 subscribers); paid plans for automation and multiple users.
- **Cloud vs Standalone**: Native support for Cloud mode. For Standalone, users can provide their own Kit API key to enable email functionality from their local machine.

### Persona Pain Point Summary
| Persona | Pain Point | Solution via Kit Integration |
|---------|------------|------------------------------|
| **Leo (Tutor)** | Struggles to keep his students engaged between lessons. | Automated "Weekly Practice Tip" sequence that keeps him top-of-mind. |
| **Maya (Baker)**| Wants to announce her holiday specials to past customers. | "Broadcast" email sent to everyone tagged with "Past Customer" in her OHC list. |

## Design Doc
- **Component**: `CreatorEmailService`
- **Responsibilities**:
  - Synchronize OHC Customer tags and segments to Kit.
  - Provide a simple "Email Designer" in the OHC app that maps to Kit's template system.
  - Listen for Webhooks (e.g., `subscriber.unsubscribe`) to update the OHC customer record.
- **User Experience**:
  - A "Marketing" tab in the OHC app.
  - AI "Promoter" drafts an email; user reviews and clicks "Send via Kit".

## Implementation Prompt
"Integrate the Kit (ConvertKit) API into OHC's Marketing department. Create a Go package in `src/server/integrations/kit/` that supports creating subscribers, adding tags, and triggering broadcasts. Implement a background worker that keeps the OHC customer list in sync with the Kit subscriber list. Acceptance criteria: A user can tag a customer in OHC and see that customer appear in their Kit account with the same tag."

## Priority
P1

## Estimated Scope
Small
