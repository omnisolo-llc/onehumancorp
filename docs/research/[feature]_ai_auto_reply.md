# Feature: AI Auto-Responder (The Always-On Sales Agent)

## Target Personas
**Carlos (Handyman, 42) & Priya (Boutique Owner, 35)**
- **Pain Point**: Frequently unavailable to answer messages because they are actively working with clients, baking, or doing manual labor. They lack a booking system, perform manual quoting, and miss leads when busy.
- **Goal**: Never miss a potential sale or booking inquiry, even when away from the phone.

## Overview
The AI Auto-Responder is an autonomous agent that intercepts incoming customer messages across multiple channels (Instagram DMs, WhatsApp, Email). It answers FAQs based on the store's policy, checks inventory or calendar availability, and can complete sales or bookings without human intervention.

## Core Capabilities
1. **Omnichannel Interception**: Connects directly to a Unified Inbox, monitoring Instagram, WhatsApp, and Email for new inquiries.
2. **Context-Aware Replies**: Answers questions using specific business context (e.g., "Yes, we have 3 red dresses in medium left," or "Carlos is available next Tuesday at 2 PM for a quote").
3. **Action Execution**: Capable of sending booking links, generating quotes, or finalizing a sale directly within the chat interface.
4. **Handoff Mechanism**: Recognizes complex or sensitive queries and smoothly hands them off to the business owner with a notification.

## User Journey
1. **Inquiry Received**: A customer sends an Instagram DM: "Do you have time to fix a leaky pipe tomorrow?"
2. **Agent Interception**: The AI Auto-Responder evaluates Carlos's calendar and standard pricing.
3. **Autonomous Reply**: The AI responds within seconds: "Hi! Carlos is booked tomorrow, but he has an opening on Wednesday at 10 AM. The initial assessment fee is $50. Would you like me to book that for you?"
4. **Confirmation**: The customer agrees, and the AI sends a booking confirmation link.
5. **Owner Notification**: Carlos receives a simple push notification: "New booking for Wednesday at 10 AM: Leaky pipe."

## Technical Architecture & Implementation
- **Unified Inbox Integration**: Built on top of the NATS hybrid event mesh to reliably process incoming messages from various channels.
- **Context Synchronization**: The Built-in Agent retrieves realtime context from the Distributed State Machine (inventory, calendar, policies).
- **Execution**: Generates natural language responses while triggering backend events (e.g., reserving calendar slots) using the Hybrid Agentic OS capabilities.
- **Resilience**: The message bus ensures async job dispatch survives network partitions, allowing the Auto-Responder to queue replies if the primary network is temporarily degraded.
