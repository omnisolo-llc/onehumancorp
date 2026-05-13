# Local SEO Autopilot Engine

## Problem Statement
The survival and growth of local service businesses (like plumbers, electricians, or tutors) are inextricably linked to their visibility on Google Maps and local search rankings. However, technical marketing concepts such as 'SEO', 'Backlink Acquisition', and 'Schema Markup' represent a completely foreign language to these practitioners.

## Research Report
Marketing agencies routinely charge SMBs anywhere from $500 to $2000 per month for basic 'local SEO' services. In reality, these services primarily consist of ensuring the Google Business Profile remains updated and maintaining strict NAP (Name, Address, Phone number) consistency across various online directories. OHC has a strategic opportunity to completely commoditize this entire agency service model via an automated, integrated background agent.

## Design Doc
### Architecture Vision
- **Entities**: BusinessProfile, DirectoryListing, SEOMetric, ReviewRequest.
- **UX Flow**:
  1. During the initial onboarding flow, the OHC system seamlessly guides the user to claim or create their Google Business Profile via API.
  2. Whenever the user posts a new photo to their OHC storefront, the system automatically formats and cross-posts that update to their Google Business feed.
  3. The system automatically identifies highly satisfied customers (e.g., those who left a 5-star rating internally) and prompts them via SMS to replicate their positive review on Google Maps.
- **Mobile UX**: Introduce a dedicated 'Growth' tab that abstracts away complex SEO metrics, displaying instead a highly simplified, gamified 'Visibility Score' (e.g., 85/100).
- **Agent Integration**: A dedicated Marketing Agent interfaces securely with the Google My Business API, manages directory consistency checks, and orchestrates the automated review generation campaigns.

## Implementation Prompt
**Outcome**: Construct an invisible background system that proactively manages the business's entire local search presence, pushing content updates directly to Google Business and automating the critical review request workflow.
**Critical User Journey**:
1. The user completes a service job and marks the associated invoice as 'Paid'.
2. The system waits an appropriate interval, then automatically texts the client requesting a quick review.
3. If the internal review is highly positive, the system prompts the client with a direct link to share their experience publicly on Google Maps.
**Acceptance Criteria**: The feature must integrate cleanly and reliably with the official Google Business Profile API. It must strictly automate the review request flow based on verifiable transaction completion events to ensure authenticity.

## Priority
P1

## Estimated Scope
Large
