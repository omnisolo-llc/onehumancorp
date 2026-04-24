# Scout: Calendar & Scheduling (Calendly)

## Title
Advanced Multi-User Scheduling & Handoff 🗓️ (Calendly API Integration)

## Problem Statement
Service-based businesses with growing teams, or those needing specialized meeting types (like Priya's boutique consultations or Carlos's emergency repairs), require more than a simple calendar. They need round-robin scheduling, automated reminders, and the ability to embed the booking experience directly into their native mobile apps and websites without jumping to external pages.

## Research Report
- **Goal**: Evaluate the new Calendly Scheduling API as a high-tier scheduling option for OHC's Operations Department.
- **Features evaluated**:
  - **Scheduling API**: Build scheduling directly into OHC without redirects or iframes.
  - **Round Robin**: Automatically distribute bookings among team members.
  - **Workflow Automation**: Automated SMS/Email sequences before and after meetings.
  - **Routing Forms**: Qualify leads before they can book a slot.
- **Benefits for OHC users (Non-technical)**:
  - Professional, white-labeled booking experience that stays within the OHC ecosystem.
  - Reduces "no-shows" via automated reminders.
  - Scales with the business as they hire more staff.
- **Integration Risks**:
  - Calendly API is robust but requires careful handling of OAuth and webhook signatures.
  - Higher-tier features (like Round Robin) may require a paid Calendly subscription for the user.
- **Pricing**: Free tier available; "Professional" and "Teams" plans required for advanced features.
- **Cloud vs Standalone**: Native support for Cloud mode. For Standalone, Calendly's API can be called from the local backend, with webhooks routed through the Hybrid MCP tunnel.

### Persona Pain Point Summary
| Persona | Pain Point | Solution via Calendly Integration |
|---------|------------|-----------------------------------|
| **Priya (Boutique)** | Wants to offer "VIP Personal Shopping" sessions but only when her lead stylist is available. | Round-robin scheduling between herself and her stylist, ensuring the customer always gets a slot. |
| **Carlos (Handyman)**| Forgets to send the "I'm on my way" text. | Calendly Workflows can automate the "Day of" reminder via SMS. |

## Design Doc
- **Component**: `AdvancedSchedulingService`
- **Responsibilities**:
  - Manage Calendly OAuth for OHC tenants.
  - Provide a Flutter wrapper for the Calendly "headless" scheduling flow.
  - Synchronize Calendly events with OHC's internal task list and Finance department for billing.
- **User Experience**:
  - Users select "Advanced Scheduling" in the OHC app.
  - They configure "Meeting Types" (e.g., 30m consultation, 1h repair).
  - A clean, OHC-branded calendar widget appears on their storefront.

## Implementation Prompt
"Integrate the Calendly Scheduling API into OHC. Create a service in `src/server/integrations/calendly/` that handles user authentication and event synchronization. Build a Flutter widget in `src/app/lib/widgets/scheduling/` that utilizes the Calendly API to provide a native booking experience. Ensure that when a booking is confirmed, it triggers a 'Meeting Created' event in the OHC Teammate Mesh."

## Priority
P1

## Estimated Scope
Medium
