**Title**: Integrate MailerLite for OHC

## Problem Statement
I want to send newsletters and promotional offers to my customer list, but I don't know how to design them or manage my subscribers without paying for a complex enterprise tool.

## Research Report
**Tool Evaluated:** MailerLite

**Findings:** MailerLite is exceptionally user-friendly for small businesses. It offers a great drag-and-drop editor, automation workflows, and high deliverability. The free tier supports up to 1,000 subscribers and 12,000 emails/month. The API is RESTful and straightforward.

**Pricing:** Free up to 1K subscribers; Paid starts at $9/mo.

**Cloud vs Standalone Mode:** Works well for both, relying on standard API calls.

## Design Doc
When a new customer buys or signs up in OHC, their email is automatically added to a MailerLite group. Business owners can trigger pre-built MailerLite campaigns directly from the OHC dashboard.

## Implementation Prompt
Integrate MailerLite so that any customer captured in OHC is synced to a MailerLite subscriber list. Provide a simple interface in OHC for the owner to view their subscriber count and recent campaign performance.

## Priority
P2

## Estimated Scope
Medium
