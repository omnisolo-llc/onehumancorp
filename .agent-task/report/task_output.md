# Research Report: OHC Business Journey Architecture

## Overview
This report documents the findings and architectural decisions for the complete end-to-end business journey on the OneHumanCorp (OHC) platform. It maps out how non-technical small business owners transition from initial discovery to active revenue generation, fully supported by OHC's AI agents.

## Problem Statement
Small business owners coming to OHC range from zero technical knowledge to semi-technical. Regardless of their background, the platform must guide them from idea to a live, functioning business in under 10 minutes. Currently, the architecture lacks a formalized, end-to-end design of this business journey that maps how different personas progress through Acquisition, Onboarding, Activation, Retention, Revenue, and Referral. Without this, there is a risk of introducing friction points that lead to abandonment, particularly for users managing their business exclusively from mobile devices.

## Persona Analysis & Journey
We analyzed the journey for five distinct personas:
1. **Maya (The Home Baker):** Focuses on product catalog creation, deposit-based orders, and Instagram integration. Her journey highlights the transition from social media acquisition to a generated storefront via the Marketing Agent.
2. **Carlos (The Freelance Handyman):** Needs service listings, booking calendars, and automated quotes. His activation relies on the Sales Agent drafting quote templates and the Operations Agent managing bookings.
3. **Priya (The Boutique Owner):** Requires hybrid online/in-store capability. Her critical path includes inventory syncing, variant configuration, and Stripe Terminal setup for POS.
4. **Leo (The Music Tutor):** Deals with subscriptions and bookings. His onboarding involves connecting Google Calendar, setting up recurring billing, and creating a link-in-bio page.
5. **Fatima (The Food Cart Operator):** Requires multi-lingual support, robust low-data mobile performance, and instantaneous order toggles ("Sold Out"). Her focus is purely operational and transactional.

## Competitive Gap Analysis
- **Shopify:** Complex onboarding (30-60 mins), requires significant manual setup, not mobile-first for management.
- **Wix/Squarespace:** Website-centric, lacks native booking/service integrations out-of-the-box, limited AI agency.
- **GoDaddy:** Basic implementation with limited functionality for complex flows like deposits or specific booking types.
- **OHC Advantage:** Zero-jargon, 10-minute setup, invisible AI orchestration, mobile-first management.

## Architectural Principles Identified
1. **Mobile-First Engagement:** All onboarding flows must be fully functional on a 375px screen.
2. **Progressive Disclosure:** Ask only the absolute minimum required to go live (Name, Type of Business, First Product). Defer complex settings (tax, advanced shipping) until necessary.
3. **AI-Driven Bootstrapping:** The "Promoter" agent generates the initial storefront design based on the business type and minimal inputs.

## Proposed Next Steps
- Implement the foundations in the Flutter application following the implementation prompt detailed in the issue brief.
- Ensure telemetry tracking is built into the funnel for Acquisition and Activation tracking.
- Create Playwright E2E tests for the new onboarding wizard.

*(Refer to `docs/research/[architecture]_business_journey.md` for full implementation details and Mermaid sequence diagrams).*