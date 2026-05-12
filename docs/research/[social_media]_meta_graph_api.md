# Issue Brief: Meta Integration (Facebook & Instagram)

## Title
Implement Meta Integration (Facebook & Instagram) for Small Business Owners

## Problem Statement
Fatima the salon owner constantly misses booking requests because they are buried in her Instagram Direct Messages. Checking a separate phone app is slow and disrupts her workflow.

## Research Report
The Meta integration connects the business's Facebook Page and Instagram Professional account directly to OHC.

**Persona Impact:** This transforms Fatima's workflow. Instead of checking a separate app, all messages from her most important marketing channels appear inside the OHC unified inbox on her dashboard. She can reply during breaks.

**Advantages:** Complete access to the world's largest social networks. Customers stay in their preferred app.

**Risks:** The onboarding requires properly configured Facebook Business pages, which is confusing.

**Pricing Estimate:** The underlying connection is free for the business.

**Environment:** Works seamlessly in both Cloud (SaaS) and Standalone offline-first modes.

## Design Doc
1.  **Connection Flow:** A simple 'Connect to Facebook/Instagram' button that securely links their account.
2.  **Unified Inbox:** A centralized view within OHC where Instagram DMs and Facebook messages flow into a single stream alongside regular emails.
3.  **Notification Hub:** An alert system within OHC that notifies the user immediately when a new message arrives.

## Implementation Prompt
Create a seamless connection to Meta platforms so business owners can read and reply to their social media messages directly from the OHC dashboard. Focus purely on a click-through setup process.

## Priority
P0

## Estimated Scope
Large

### Unique Considerations
For Meta, handling the frequent token expiration is paramount. If Fatima's token expires, we cannot just fail silently. The UI must show a red banner indicating 'Instagram Connection Needs Re-Authentication' so she doesn't miss a week of DMs before noticing.
