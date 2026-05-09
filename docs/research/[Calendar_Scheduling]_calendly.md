# Calendly Integration for Automated Scheduling

## Title
Sync Calendly to Automate Appointment Booking

## Problem Statement
Scheduling meetings, consultations, or services often involves endless back-and-forth emails. Small business owners lose valuable time trying to find a time that works for both parties. They need a simple way to share a link, let the client pick a time, and have it automatically appear on their calendar without any manual data entry.

## Research Report
Calendly is a leading scheduling automation platform. Research shows it uses a per-seat, monthly or annual billing model. It offers a genuinely functional free tier, with paid plans starting around $8-$12/month (Standard) and scaling up for larger teams. The free tier limits variety and automation but provides the core scheduling experience.

For small business owners, Calendly is highly recognizable and easy to use. The main risk is the potential for hidden costs as teams grow or require advanced features (like routing or complex team scheduling). However, its ubiquity makes it a must-have integration. It solves the scheduling pain point effectively. This integration would work well in both Cloud (webhook based) and Standalone (polling/webhook) environments.

## Design Doc
The business owner will connect their Calendly account via the OHC integrations page using OAuth. Once connected, OHC will automatically pull in scheduled events and display them on the internal OHC Calendar. When an OHC agent is drafting an email to a client requesting a meeting, the agent can easily insert the owner's Calendly link. If a client books a time, OHC will receive a notification and update the customer's CRM profile with the upcoming appointment details.

## Implementation Prompt
Create an integration that allows users to authorize Calendly via OAuth. Sync Calendly events into the OHC calendar view in real-time. Add a "Insert Booking Link" button in the email/message composer that automatically pastes the user's primary Calendly link. Ensure customer profiles are updated when they book a meeting via the Calendly integration.

## Priority
P1

## Estimated Scope
Small
