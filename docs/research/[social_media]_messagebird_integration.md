# Integrate MessageBird for Unified Multichannel Inbox

## Problem Statement
Small business owners like Priya (Boutique) and Maya (Baker) receive customer inquiries across Instagram, WhatsApp, and Facebook. Switching between apps causes missed messages and lost sales. They need all DMs in one unified OHC inbox so their Customer Success AI can instantly reply while they sleep.

## Research Report
- **Tool Evaluated**: MessageBird (by Bird)
- **Ease of Use**: Provides a unified API for WhatsApp, Instagram, Messenger, and SMS.
- **Pricing**: Pay-per-message model. Competitive and transparent.
- **Standalone/Cloud**: Works perfectly in both modes via REST API.
- **Persona Fit**: Perfect for non-technical users who just want "all messages in one place".

## Design Doc
- **Integration Point**: Customer Success Agent and Unified Inbox UI.
- **Trigger**: Incoming webhook from MessageBird.
- **Action**: Store message in OHC unified inbox, trigger Customer Success agent for automated draft reply.
- **User View**: A simple "Connect Instagram/WhatsApp" button in OHC Settings. Messages appear in the OHC Inbox like standard texts.

## Implementation Prompt
Create a unified webhook handler for MessageBird that parses incoming messages from WhatsApp/IG/FB and stores them in the OHC tenant inbox. Implement the settings UI to allow users to connect their social channels via MessageBird OAuth. Ensure the Customer Success AI agent is triggered on new message arrival.

## Priority
P1

## Estimated Scope
Medium
