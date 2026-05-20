# Calendar & Scheduling Integration

## Problem Statement
Service-based businesses (tutors, handymen, consultants) lose customers due to the friction of back-and-forth scheduling. They need a simple, public booking link that seamlessly syncs with their personal calendars to prevent double bookings.

## Evaluated Tools
We evaluated the following scheduling infrastructures:
1. **Calendly**: A very popular and user-friendly scheduling tool. However, its embed options can sometimes feel disjointed from the host platform, and it is a closed ecosystem.
2. **Cal.com**: An open-source scheduling infrastructure. It handles timezone math, calendar conflict resolution, and booking pages natively.

## Key Recommendation
- **Cal.com** is strongly recommended for its open-source nature and 'Atoms' UI components. This allows OHC to deeply integrate scheduling into the platform. Crucially, its self-hosted option makes it extremely valuable for OHC's Standalone mode, ensuring users have access to scheduling without relying on cloud infrastructure.
