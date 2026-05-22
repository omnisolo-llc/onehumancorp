# Issue Brief: Autonomous AI Setup Agent

## Title
Autonomous AI Setup Agent for Zero-Config SMB Onboarding

## Problem Statement
Small business owners—especially those without technical backgrounds (like Maya the baker, Carlos the handyman, Priya the boutique owner, Leo the music tutor, and Fatima the food cart owner)—are overwhelmed by the complexity of setting up an online presence. Existing platforms (Shopify, Wix) are essentially just a box of tools that require users to learn web design, configure domains, set up complex booking/inventory syncs, and write copy. This results in abandonment and failure to launch. Non-technical users need an invisible agent that does the heavy lifting, allowing them to launch a fully functional business from their phone in under 10 minutes by simply answering a few questions and making decisions, not building a site.

## Research Report
*   **Competitor Analysis**: Tools like Shopify and Wix offer "AI" that mostly generates text or layouts, but the burden of configuring the actual business logic (inventory, booking, payments, domains) still falls entirely on the user. The onboarding time to a *live* store is typically measured in hours or days.
*   **AI-Native Competitors**: Emerging platforms like Durable and Mixo can generate a basic website in seconds, but they lack the deep back-office capabilities (booking, POS, unified inbox, multi-location inventory) required to run a real business.
*   **User Pain Points (from reviews and Reddit)**: Users consistently complain about "complex setup," "hidden costs for basic apps," and "needing to hire a developer just to get started."
*   **OHC Advantage**: OHC is positioned to combine the speed of AI-native builders with the depth of a full SMB platform. We don't just generate a website; we configure the entire business operating system.
*   **Strategic Focus**: By implementing an autonomous agent that handles setup invisibly, OHC can capture the vast segment of the market that is currently intimidated by traditional builders.

## Design Doc
*   **High-Level Architecture**:
    *   **Conversational Interface**: A mobile-first, chat-based onboarding flow where an AI agent asks targeted questions about the business.
    *   **Agentic Orchestrator**: An intelligent engine that translates the user's conversational inputs into system configurations (e.g., creating products, setting up booking calendars, configuring tax rules, generating initial marketing copy).
    *   **Template & Component Library**: A set of modular, AI-assemblable UI components for the storefront.
    *   **Integration Points**: Seamless connection to the Unified Booking Engine, Universal Capacity & Inventory Ledger, and the Omnichannel AI Inbox.
*   **UI Wireframes/Screen Flow (Mobile First - 375px)**:
    1.  **Welcome Screen**: Simple, inviting. "Hi, I'm OHC. Tell me about your business in a few words."
    2.  **Conversational Flow**: 3-5 screens of chat-style interaction. "What services do you offer?" "Do you have photos, or should I generate some?"
    3.  **Loading/Generation Screen**: "Building your business..." with animated progress indicators (building site, configuring booking, setting up payments).
    4.  **Review & Launch Screen**: A preview of the live storefront and a summary of the configured back-office. A single "Launch" button.
*   **AI Agent Integration**: The core of this feature is the orchestrator agent that interprets natural language and autonomously configures the underlying OHC modules via internal APIs.

## Implementation Prompt
**User-Facing Outcome:**
A non-technical user (e.g., a baker or handyman) can open the OHC app, describe their business in plain language, and within 10 minutes have a live, fully functional storefront with booking, inventory, and payments configured, ready to accept customers.

**Critical User Journey:**
1.  User signs up and enters the "Setup Agent" flow.
2.  User provides basic details (name, industry, offerings) via a chat interface.
3.  The AI Agent generates a complete storefront, including relevant copy and images.
4.  The AI Agent configures the back-office (e.g., sets up a booking calendar for a handyman, or product listings for a baker).
5.  User reviews the generated business, makes any necessary tweaks via a simple editor, and clicks "Launch".

**Acceptance Criteria:**
*   The setup process must be primarily conversational, requiring no drag-and-drop web design skills.
*   The agent must successfully configure at least one core back-office feature (e.g., booking, inventory, or digital product delivery) based on the user's industry.
*   The end result must be a publicly accessible storefront.
*   The entire flow must be fully functional and optimized for mobile devices (375px width).

## Priority
P0

## Estimated Scope
Large
