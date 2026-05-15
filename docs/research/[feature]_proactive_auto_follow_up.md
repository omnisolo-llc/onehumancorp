# Feature Brief: Proactive CRM & Auto-Follow-Up System

## Title
Proactive CRM: The Retention Engine

## Problem Statement
Service providers like Carlos (Handyman) and product sellers like Maya (Baker) lose significant recurring revenue because they forget to follow up with past clients. The CRM aspect of small business is often non-existent.

## Design Doc

### High-Level Requirements
- **Event-Driven Follow-Up:** KAIROS must observe when a service is completed or a consumable product is sold, and schedule a follow-up task.
- **Agent Drafting:** The Customer Success Agent drafts a personalized follow-up message (e.g., "Hi [Name], it's been 6 months since we fixed the sink. Does it need a quick check-up?").
- **1-Tap Approval:** The draft appears in the user's Dashboard Action Feed. 1-tap approves sending the message via the customer's preferred channel (SMS/Email/DM).

### Action Items
- Define the `FollowUp` task type in the KAIROS Orchestrator.
- Implement the timeline logic (e.g., 30 days for consumables, 6 months for services).
- Create the Dashboard Action Feed card for "Approve Follow-up".
