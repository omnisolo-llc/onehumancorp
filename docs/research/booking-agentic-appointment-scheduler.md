# [Booking] Agentic Appointment Scheduler

**Priority:** P1 | **Estimated Scope:** Medium

## Problem Statement
Service providers (like Carlos and Leo) struggle with manual booking and quoting. Existing tools (like Wix Scheduling) are static forms, leading to endless back-and-forth emails to finalize times and quotes.

## Research Report
**Findings:**
- Service businesses make up over 40% of the SMB market, yet most platforms are built for e-commerce (physical goods).
- Shopify is purely physical/digital product focused; Wix Scheduling is functional but static.
- Users complain on r/sweatystartup about losing clients due to slow quoting.
**Evidence:** Competitor analysis of Wix and Shopify shows no native AI-negotiated booking.

## Design Doc
**Architecture:**
- Entity Types: `ServiceTemplate`, `BookingRequest`, `Quote`
- Key Relationships: `ServiceTemplate` defines rules, AI generates `Quote` for a `BookingRequest`.
- Integration Points: Calendar Sync (Google/Outlook), Stripe (Hold deposits).
- AI Integration: Agent parses customer request, checks availability, and quotes price dynamically.

## Implementation Prompt
Build a conversational booking widget where customers describe what they need (e.g., 'fix a leaky pipe next Tuesday'). The AI agent should ask clarifying questions, suggest a time, and provide an estimated quote, all without the owner's intervention.
