# 🔮 Oracle Issue Brief: AI Booking Agent

## Title
Implement Autonomous Booking Agent for Service SMBs

## Problem Statement
Service-based small business owners (like Carlos the handyman or Leo the tutor) spend hours every week managing bookings via text message, Instagram DMs, and phone calls. This manual process leads to double-bookings, lost leads when they are busy working, and significant scheduling chaos. They don't want a complex calendar software; they want an assistant to handle it for them.

## Research Report
- **Top Pain Point**: "Managing bookings via text message is a nightmare."
- **Market Sizing**: The service-based solopreneur is OHC's recommended beachhead market, as e-commerce is saturated.
- **Competitor Landscape**:
  - Shopify: N/A (E-commerce focused).
  - Wix: Paid add-on, requires manual configuration by user.
  - OHC Gap: We lack a native booking engine that our AI agents can interface with.

## Design Doc
- **High-level Architecture**:
  - `BookingEvent` entity: represents a scheduled block of time.
  - `ServiceAvailability` entity: represents working hours and rules.
  - **AI Agent Integration**: The builtin LLM agent needs tools to read availability and propose/confirm `BookingEvent` records.
- **UI Flow (Mobile First - 375px)**:
  - Owner view: Simple daily agenda view. Tap to cancel/reschedule.
  - Settings: Simple toggle for "Let AI manage my schedule" and "Working hours".
  - Customer view: Chat interface where they request a time, and the AI agent negotiates and confirms.

## Implementation Prompt
Create the core booking system that allows the OHC AI agent to autonomously manage a user's calendar.
- The system must store availability and individual booking events.
- It must provide a way for the AI agent to check available slots, propose times to a customer, and confirm a booking.
- The owner's UI should be a simple daily agenda that fits perfectly on a mobile screen.
- Focus on the Critical User Journey: A customer asks for an appointment via chat, the AI checks the calendar, proposes a time, and saves the confirmed appointment to the database.

## Priority
P0

## Estimated Scope
Large
