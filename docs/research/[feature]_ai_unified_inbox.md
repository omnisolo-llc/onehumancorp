# Issue Brief: Unified Omnichannel AI Inbox

## Title
Unified Omnichannel AI Inbox

## Problem Statement
"Scattered Communications" and "Operational Fatigue" plague users like Maya, who lose orders across Instagram, WhatsApp, and email. Solopreneurs lose up to 30% of sales simply due to slow response times or forgotten messages across disjointed platforms.

## Research Report
Analysis of Maya (The Home Baker) persona reveals she loses track of custom orders in DMs. A unified inbox would consolidate these scattered messages and leverage an AI agent to draft replies based on past customer history.

Based on the Top 10 SMB User Pain Points, "Operational Fatigue" (68%) and "Communication Lag" (40%) are critical friction points. Competitors like Shopify require third-party apps (e.g., Gorgias) for this, creating cost creep. Wix has a passive unified inbox. OHC can leapfrog by integrating an active "Silent Ambassador" AI.

## Design Doc
A centralized feed (`Hub` data model) aggregating all external messages. The 'Ambassador' AI agent listens to the event mesh, drafts contextual replies based on past customer history, business memory (inventory, pricing), and presents them for 1-tap approval.

### High-Level Architecture
- **Data Model**: `Hub` entity consolidates messages from NATS event mesh (connected to Meta Graph API, Email, SMS).
- **AI Agent**: The Customer Success ("Ambassador") Agent observes incoming messages.
- **Workflow**: Agent drafts a contextual response -> Draft queued in Action Feed -> User receives notification.
- **UI Flow**: Mobile-first, 375px optimized lock-screen or dashboard push notification for 1-tap approval.

## Implementation Prompt
Create the UI and backend logic for the Unified Inbox. The Critical User Journey involves opening a new message, reviewing an AI-generated draft, and tapping 'Approve to Send'. Must integrate with the existing `Hub` Go backend and NATS event mesh.

## Priority
P0

## Estimated Scope
Medium
