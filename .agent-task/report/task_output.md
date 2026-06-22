issue_title: "[Research] AI Scheduling Copilot & Unified Operations Assistant"
issue_description: |
  # OHC AI Scheduling Copilot & Unified Operations Assistant

  ## Problem Statement
  Small business owners and operators (like Carlos the Field Service Owner and Leo the Music Tutor) currently lack a single, unified view of their daily schedule combined with customer context and action-oriented AI. They manage schedules in Google Calendar/Calendly, customer communication in iMessage/WhatsApp, and payments in Stripe/Square. This fragmentation leads to missed leads, double bookings, and a lack of preparation for upcoming appointments. Owners need an assistant that doesn't just show them the calendar, but actively helps them prepare, follow up, and monetize their time without forcing them to configure complex rules or switch apps constantly.

  ## Research Report

  ### Market Mapping & Competitor Discovery

  #### Top 10 General Competitors
  1. **Calendly**: Calendly.com. The standard for automated scheduling, but lacks deep CRM integration out of the box and operates purely defensively (user sets rules, Calendly enforces them).
  2. **Square Appointments**: Squareup.com/appointments. Strong POS integration, but scheduling interface is basic and lacks proactive AI help.
  3. **Acuity Scheduling (Squarespace)**: Acuityscheduling.com. Highly customizable, but complex setup and no native AI agent capabilities.
  4. **HubSpot Meetings**: Hubspot.com/products/sales/schedule-meeting. Deep CRM ties, but tailored for B2B sales teams, not SMB operators.
  5. **Doodle**: Doodle.com. Great for group scheduling, lacks SMB operations features.
  6. **SimplyBook.me**: Simplybook.me. Highly modular, but the UI is dated and lacks AI context.
  7. **Setmore**: Setmore.com. Free tier is popular, but very basic feature set.
  8. **Vagaro**: Vagaro.com. Dominates salon/fitness vertical, but highly specialized and clunky for general service providers.
  9. **Jobber**: Getjobber.com. Excellent for field services (like Carlos), but expensive and complex for simpler businesses.
  10. **Booksy**: Booksy.com. Strong consumer marketplace, but locks the provider into their ecosystem.

  #### Top 10 AI-Native Competitors
  1. **Reclaim.ai**: Reclaim.ai. AI calendar assistant for individuals, focuses on time blocking rather than customer scheduling.
  2. **Motion**: Usemotion.com. AI task and project management, complex for a simple service provider.
  3. **Clockwise**: Getclockwise.com. Team calendar optimization, not built for customer-facing bookings.
  4. **Sidekick AI**: Sidekickai.com. Scheduling assistant, but lacks deep business operations context.
  5. **Lindy.ai**: Lindy.ai. Autonomous AI executive assistant; can handle scheduling via natural language, but lacks a dedicated SMB dashboard.
  6. **Moby**: Moby.app. AI-powered booking for local businesses, interesting conversational approach.
  7. **Julie Desk**: Juliedesk.com. AI scheduling assistant acting via email, mostly B2B focused.
  8. **Clara Labs**: Claralabs.com. Human-in-the-loop AI scheduler, very expensive and enterprise-focused.
  9. **x.ai (acquired by Bizzabo)**: Historically pioneered AI email scheduling, but pivoted.
  10. **Skejul**: Skejul.com. AI predictive scheduling, more focused on logistics than SMB operations.

  ### Deep-Dive Competitor Audit: Square Appointments

  We selected Square Appointments for a deep dive because it represents the most common "all-in-one" solution for service-based SMBs today, sitting directly in the path of payments.

  *   **Capabilities**: Online booking site, calendar management, automated reminders (SMS/Email), staff management, integration with Square POS for checkout, basic customer profiles.
  *   **Success Factors**: Frictionless integration with in-person payments. If a user already uses Square for payments, Appointments is the logical next step. Free tier is generous for solo operators.
  *   **User Sentiment Audit**:
      *   *Positive*: "It just works with my card reader." "Customers find the booking page easy to use."
      *   *Negative (Pain Points)*: "I hate that I can't customize the automated texts more easily." "It doesn't help me recover people who started booking but dropped off." "The calendar view on mobile is cluttered when I have multiple staff members." "I have to switch between the Square app and my personal texts to see what the customer actually asked for before the appointment."

  ### OHC Gap & Pain Point Identification

  *   **OHC Feature Audit**: OHC currently lacks a dedicated, AI-driven scheduling and operations view designed specifically for service providers and creators. We have foundational elements (Unified Inbox concepts, Agent Feed), but not a calendar-first operational nexus.
  *   **Gap Matrix (OHC vs. Square Appointments)**:
      *   *Square*: Basic calendar, manual rules, payment integration.
      *   *OHC Gap*: We need a smart calendar that understands the *context* of the appointment, not just the time slot. We lack the interface where Carlos (Field Service) or Leo (Tutor) starts their day.
  *   **Unresolved Pain Points (Market-wide)**:
      1.  **Context Switching**: Owners check their calendar, then search messages to remember what the customer wanted, then open a payment app to see if a deposit was paid.
      2.  **Missed Revenue**: When a customer asks for availability via DM but doesn't book immediately, the lead is often lost because the owner forgets to follow up manually.
      3.  **Preparation Overhead**: Owners don't have a quick summary of the client's history before the meeting starts.

  ### Agentic Solution Design: The OHC Scheduling Copilot

  OHC will solve this by creating an "Operations Assistant" view centered around the daily schedule, supercharged by AI.

  *   **Unified Context**: The calendar event isn't just a time block; it's a living entity linked to the customer's CRM profile, past DMs, and payment status.
  *   **Agentic Preparation**: 30 minutes before a meeting, the Operations Assistant pushes a summary card to the Agent Feed: "Next: Guitar Lesson with Sarah. She struggled with chords last week. She hasn't paid her monthly invoice yet. [Draft Invoice Reminder]"
  *   **Autonomous Follow-up**: If a customer asks for availability in an Instagram DM ("Are you free Tuesday?"), the Customer Success Agent reads the connected calendar, drafts a reply with available slots ("I have 2pm or 4pm open!"), and generates a one-click booking link. If the customer doesn't book within 24 hours, the Operations Agent suggests a follow-up message to the owner.

  ## Design Doc

  *   **High-Level Architecture**:
      *   `SchedulingEngine`: Manages core availability, time zones, and booking logic.
      *   `OperationsAgent`: The LLM-powered actor that monitors the schedule, generates preparation summaries, and identifies follow-up opportunities.
      *   `CalendarIntegrationService`: Syncs with external providers (Google Calendar, Outlook) to prevent double-booking with personal events.
  *   **UI/UX Flow (Mobile First - 375px)**:
      1.  **The "Today" Tab (Home)**: A clean, vertical timeline of the day's events.
      2.  **The "Action Required" Header**: Pinned at the top of the Today tab. Shows Agent Feed cards relevant to the schedule (e.g., "3 pending booking requests", "Drafted follow-up for missed appointment").
      3.  **Appointment Detail View**: Tapping an event reveals a unified card:
          *   Client Name & Photo
          *   *AI Summary*: "3rd lesson. Focus: Jazz scales."
          *   *Status*: "Paid" / "Deposit Required"
          *   *Action Buttons*: "Message Client", "Reschedule", "Request Payment".
  *   **Visual Style**: OHC Premium Token library. Clean, Apple-style hierarchy. Minimalist calendar representation that prioritizes text readability and clear status tokens over complex grid layouts on mobile.

  ### Premium Mermaid.js Charts

  **Competitive Landscape Heatmap**
  ```mermaid
  quadrantChart
    title SMB Scheduling Platform Landscape (2025)
    x-axis "Manual Setup" --> "Agentic & Autonomous"
    y-axis "Niche/Point Solution" --> "Unified Operations"
    quadrant-1 "Ideal Future State"
    quadrant-2 "Legacy All-in-Ones"
    quadrant-3 "Legacy Point Solutions"
    quadrant-4 "Emerging Point Solutions"
    "Calendly": [0.1, 0.2]
    "Square Appointments": [0.3, 0.7]
    "Acuity Scheduling": [0.2, 0.5]
    "HubSpot Meetings": [0.4, 0.8]
    "Doodle": [0.1, 0.1]
    "SimplyBook.me": [0.2, 0.4]
    "Setmore": [0.1, 0.3]
    "Vagaro": [0.2, 0.6]
    "Jobber": [0.3, 0.7]
    "Booksy": [0.2, 0.6]
    "Reclaim.ai": [0.7, 0.2]
    "Motion": [0.6, 0.5]
    "Clockwise": [0.6, 0.4]
    "Sidekick AI": [0.7, 0.3]
    "Lindy.ai": [0.8, 0.6]
    "Moby": [0.7, 0.4]
    "Julie Desk": [0.7, 0.3]
    "Clara Labs": [0.6, 0.4]
    "x.ai": [0.6, 0.2]
    "Skejul": [0.6, 0.3]
    "OHC Future": [0.9, 0.9]
  ```

  **User Journey Comparison: Square vs OHC**
  ```mermaid
  journey
    title Journey: Preparing for a Service Appointment
    section Square Appointments
      Check calendar time: 3: Square App
      Remember what client wants: 2: iMessage/WhatsApp
      Check if invoice paid: 1: Square App
      Message client if running late: 2: iMessage/WhatsApp
    section OHC AI Copilot
      Open app & view daily summary: 5: OHC App
      See appointment with integrated client context: 5: OHC App
      One-tap action (e.g., "Request Deposit" or "Message"): 5: OHC App
  ```

  ### Comparative Tables

  **Feature Gap Analysis**
  | Feature | OHC AI Copilot (Proposed) | Square Appointments | Calendly | Reclaim.ai |
  | :--- | :--- | :--- | :--- | :--- |
  | Core Scheduling Engine | Yes | Yes | Yes | Yes |
  | POS/Payments Integration | Yes | Yes | No | No |
  | Unified CRM Context | Yes | Partial | No | No |
  | Autonomous DM Follow-up | **Yes** | No | No | No |
  | Pre-meeting AI Briefing | **Yes** | No | No | No |
  | Agent-drafted Actions | **Yes** | No | No | No |

  ## Implementation Prompt

  **Objective**: Build the foundational "Operations Assistant" dashboard (The "Today" view) for service-based owners.

  **Critical User Journey (CUJ)**:
  1.  The owner (e.g., Leo the Tutor) opens the OHC app.
  2.  The default view is the "Today" operations dashboard.
  3.  The dashboard displays a vertically scrolling list of today's upcoming appointments.
  4.  At the top of the dashboard, an AI-generated "Morning Briefing" card summarizes the day (e.g., "You have 4 appointments today. 1 client still needs to pay their deposit.").
  5.  The owner taps on an appointment and sees a detailed view that integrates the client's information, payment status, and a button to quickly message them.

  **Acceptance Criteria**:
  *   The UI must be fully responsive, starting from a 375px mobile layout.
  *   The "Today" view must clearly differentiate between past, current, and future appointments using visual styling (e.g., graying out past events).
  *   The UI must integrate seamlessly with the existing OHC design system (translucent materials, strong typography).
  *   The implementation must include Playwright E2E tests simulating an owner viewing their daily schedule and interacting with an appointment detail card.

  ## Priority & Scope
  *   **Priority**: P1
  *   **Estimated Scope**: Medium

  ## References & Sources Catalog
  1. Calendly Homepage: https://calendly.com/
  2. Calendly Features: https://calendly.com/features
  3. Calendly Pricing: https://calendly.com/pricing
  4. Square Appointments: https://squareup.com/us/en/appointments
  5. Square Appointments Pricing: https://squareup.com/us/en/appointments/pricing
  6. Acuity Scheduling: https://acuityscheduling.com/
  7. Acuity Scheduling Features: https://acuityscheduling.com/features/
  8. HubSpot Meetings: https://www.hubspot.com/products/sales/schedule-meeting
  9. HubSpot CRM Integration: https://www.hubspot.com/products/crm
  10. Doodle: https://doodle.com/en/
  11. Doodle Use Cases: https://doodle.com/en/use-cases/
  12. SimplyBook.me: https://simplybook.me/en/
  13. SimplyBook.me Custom Features: https://simplybook.me/en/custom-features
  14. Setmore: https://www.setmore.com/
  15. Setmore Free Tier: https://www.setmore.com/pricing
  16. Vagaro: https://sales.vagaro.com/
  17. Vagaro Salon Features: https://sales.vagaro.com/features/salon-software
  18. Jobber: https://getjobber.com/
  19. Jobber Mobile App: https://getjobber.com/features/mobile-app/
  20. Booksy: https://booksy.com/biz/en-us/
  21. Reclaim.ai: https://reclaim.ai/
  22. Reclaim.ai Tasks: https://reclaim.ai/features/tasks
  23. Motion: https://www.usemotion.com/
  24. Motion AI Calendar: https://www.usemotion.com/ai-calendar
  25. Clockwise: https://www.getclockwise.com/
  26. Clockwise Focus Time: https://www.getclockwise.com/product/focus-time
  27. Sidekick AI: https://www.sidekickai.com/
  28. Lindy.ai: https://www.lindy.ai/
  29. Lindy.ai Use Cases: https://www.lindy.ai/use-cases
  30. Moby: https://moby.app/
  31. Julie Desk: https://www.juliedesk.com/
  32. Clara Labs: https://claralabs.com/
  33. x.ai / Bizzabo: https://www.bizzabo.com/
  34. Skejul: https://skejul.com/
  35. Reddit r/smallbusiness - "Best scheduling app?": https://www.reddit.com/r/smallbusiness/search/?q=scheduling+app
  36. Reddit r/ecommerce - "Square appointments vs others": https://www.reddit.com/r/ecommerce/search/?q=square+appointments
  37. Trustpilot - Square Appointments: https://www.trustpilot.com/review/squareup.com
  38. Trustpilot - Calendly: https://www.trustpilot.com/review/calendly.com
  39. Trustpilot - Acuity Scheduling: https://www.trustpilot.com/review/acuityscheduling.com
  40. App Store - Square Appointments: https://apps.apple.com/us/app/square-appointments/id1044439162
  41. App Store - Calendly: https://apps.apple.com/us/app/calendly-mobile/id1457816007
  42. Play Store - Square Appointments: https://play.google.com/store/apps/details?id=com.squareup.appointments
  43. G2 - Best Appointment Scheduling Software: https://www.g2.com/categories/appointment-scheduling
  44. Capterra - Scheduling Software: https://www.capterra.com/scheduling-software/
  45. TechRadar - Best scheduling software 2025: https://www.techradar.com/best/scheduling-software
  46. Forbes - Best Scheduling Apps For Small Business: https://www.forbes.com/advisor/business/software/best-scheduling-apps/
  47. Shopify App Store - Appointment Booking: https://apps.shopify.com/categories/store-design-store-pages-appointment-booking
  48. Wix Bookings: https://www.wix.com/business/bookings
  49. Squarespace Scheduling: https://www.squarespace.com/scheduling
  50. AI in SMB Scheduling Trends 2025: https://www.google.com/search?q=AI+in+SMB+Scheduling+Trends+2025
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
