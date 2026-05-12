# 📱 [RESEARCHER] Mobile-First Booking System

## Title
Mobile-First Booking System for Service Businesses

## Problem Statement
Service providers like Carlos (handyman) and Leo (music tutor) rely on manual scheduling via texts and calls. They miss leads when busy and struggle to manage appointments on the go. Existing solutions are either too complex (Mindbody) or not integrated into their main website (Calendly). Framed from a business owner lens, missing a booking means missing revenue.

## Research Report
- Competitor Landscape:
  - Shopify/Wix: Primarily e-commerce focused. Booking apps are paid add-ons with poor mobile UX.
  - Squarespace Acuity: Good, but standalone and disconnected from a holistic business OS.
  - OHC Advantage: Native, mobile-first booking integrated directly with CRM and payments.
- User Pain Points:
  - Reddit r/smallbusiness: "I spend 2 hours a day just texting clients back and forth for scheduling."
  - Trustpilot: "The booking app on Wix is clunky on mobile."

## Design Doc
- High-Level Architecture:
  - `Booking` entity linked to `Service`, `Customer`, and `Calendar`.
  - Integration with Google/Apple Calendar.
- UI Flow (Mobile-first 375px):
  - Screen 1: "Set Availability" (Simple weekly toggle).
  - Screen 2: "Service Details" (Name, duration, price).
  - Screen 3: "Share Booking Link" (Copy link or add to site).
- AI Agent Integration Points:
  - Agent auto-suggests buffer times and sends automated reminders.

## Implementation Prompt
Develop a mobile-native booking interface where users can define services, set availability, and generate a booking link. The customer-facing flow must be under 3 steps. The system must automatically send SMS/email confirmations and reminders. The entire setup for the business owner should take under 2 minutes.

## Priority
P1

## Estimated Scope
Large
