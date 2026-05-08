# [crm] Auto-Booking Concierge

## Problem Statement
Service-based small business owners (like Leo the tutor or Carlos the handyman) lose potential clients because they are too busy working to reply to DMs or emails promptly. Existing solutions require the business owner to manually configure complex scheduling software (like Calendly) and link it to their website. They need an invisible assistant that handles booking inquiries and manages their calendar automatically.

## Research Report
"Booking System Integration" is a major pain point, highlighted by 75% of service-based SMBs in our research. Furthermore, "No Automated Follow-Ups" causes lost revenue for 62% of respondents. Legacy e-commerce platforms primarily focus on physical goods, leaving a massive gap for an AI-native scheduling solution.
*Sources: Competitor gap analysis (Shopify's lack of native service booking) and Reddit r/smallbusiness discussions.*

## Design Doc
- **High-Level Architecture**:
  - Integration with communication channels (Email, Instagram DMs).
  - A persistent background AI Agent (using the KAIROS Distributed State Machine) monitors incoming messages.
  - The Agent cross-references the tenant's availability and services (linked to `CatalogItem` of type `service`).
  - The Agent autonomously replies to the customer, negotiates a time, and creates an `Order` or `Booking` record.
- **Mobile UX Flow (375px first)**:
  - The business owner simply sees a "New Booking Confirmed" push notification on their phone.
  - The UI provides a simple chat interface to view the conversation the Agent had with the client, with an option to manually intervene.
- **AI Agent Integration Points**: Deep LLM integration to understand intent, extract dates/times, and maintain conversational context, ensuring the AI tone matches the business's brand.

## Implementation Prompt
Implement an Auto-Booking Concierge feature. The Critical User Journey (CUJ) is: A customer sends a message asking for an appointment. The OHC background agent receives the message, checks the business's availability, replies to the customer to confirm a time, and automatically schedules the service. The business owner receives a push notification of the confirmed booking without having to lift a finger. Ensure the feature supports multi-tenant isolation and leverages the existing agent memory and orchestration systems. Do not prescribe specific database schemas or API contracts; focus on fulfilling the end-to-end user experience.

## Priority
P1

## Estimated Scope
Medium