# 🔍 Scout: Setup Wizard (Plain Language Onboarding)

## Title
Setup Wizard (Plain Language Onboarding)

## Problem Statement
New users trying to launch a business are immediately alienated by technical jargon (DNS, APIs, Webhooks, CNAME) when signing up for platforms like Shopify. "Setup Complexity" is the #1 pain point for 73% of SMBs. They need an onboarding experience that feels like a simple conversation with a business advisor, completely devoid of technical terms, resulting in an instantly live storefront.

## Research Report
- **Strategy**: Conversational, jargon-free onboarding flow.
- **Target Persona**: Maya (Home Baker), Fatima (Food Cart)
- **Advantages**: Drastically reduces time-to-value and drop-off rates during signup.
- **Risks**: Oversimplifying necessary configuration steps, leading to issues later.
- **Competitor Gap**: Durable achieves fast generation but lacks business depth. Shopify has depth but massive friction. OHC aims for < 10 minutes from signup to live store using natural language.
- **Data**: 73% of users cite "Setup Complexity" as a barrier.

## Design Doc
- **High-Level Architecture**:
  - A multi-step conversational UI form.
  - User inputs basic natural language text (e.g., "I sell vegan cakes in Austin").
  - An onboarding agent parses the intent, selects a "vibe" (design template), configures basic shipping/tax defaults based on location, and populates initial placeholder products.
  - Technical configurations (domains, payment gateways) are handled invisibly in the background or abstracted into plain-language choices.
- **UI Flow**:
  - Welcome screen: "What's the name of your business?"
  - Screen 2: "What do you do? (e.g., sell products, offer services)"
  - Screen 3: "Where are your customers located?"
  - Loading animation: "Building your store..." (Agent configures backend).
  - Success screen: "Your store is live! Here is your link."

## Implementation Prompt
Implement a new conversational Setup Wizard for the Slint UI. The wizard should consist of 3-4 simple screens asking plain language questions about the business. Wire these inputs to a new backend onboarding service that automatically provisions the tenant, applies a default design theme based on the business type, and creates a basic store configuration without requiring the user to manually configure DNS or shipping zones. Ensure the UI adheres to the glassmorphism design standard.

## Priority
P0

## Estimated Scope
Medium
