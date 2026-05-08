**Title**: Integrate Cal.com for OHC

## Problem Statement
I spend too much time going back and forth with clients trying to find a time to meet or schedule a service. Double bookings happen often.

## Research Report
**Tool Evaluated:** Cal.com

**Findings:** Cal.com is an open-source scheduling tool that offers a robust API (v2) and supports self-hosting. It handles timezone resolution, calendar syncing (Google, Outlook), and integrates with video conferencing. It has a generous free tier and clear developer docs. It's highly customizable and brandable.

**Pricing:** Free for individuals; Team plans start at $12/user/month.

**Cloud vs Standalone Mode:** Excellent for both. Can use their hosted API for Cloud, and run a self-hosted Cal.com instance alongside OHC Standalone.

## Design Doc
OHC will generate a unique Cal.com booking link for the business owner. Customers can click 'Book Appointment' on the OHC site, select a time, and it syncs to the owner's connected calendar.

## Implementation Prompt
Create a seamless Cal.com integration where business owners can connect their Google/Outlook calendars, define their working hours in OHC, and get a shareable booking link. The customer booking experience must be embedded in the OHC platform.

## Priority
P1

## Estimated Scope
Medium
