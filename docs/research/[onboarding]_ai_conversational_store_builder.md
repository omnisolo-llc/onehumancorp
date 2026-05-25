# Issue Brief: AI Conversational Store Builder

## Title
[Onboarding] AI Conversational Store Builder

## Problem Statement
Small business owners, like Maya (a baker) and Carlos (a handyman), are overwhelmed by the setup complexities of legacy platforms like Shopify and Wix. They face "Blank Page Paralysis" and struggle with technical jargon like DNS, SKUs, and complex shipping zones. They want to start selling immediately, but traditional onboarding processes require them to learn new skills, acting as complex toolkits rather than invisible assistants. This high friction actively discourages solopreneurs from launching their online presence.

## Research Report
- **Competitor Landscape**:
  - **Shopify**: High setup complexity. Over 70% of 1-star reviews cite confusing menus and configuration. Uses AI primarily as a "Copilot" (e.g., Sidekick) rather than an autonomous builder.
  - **Wix**: Medium setup complexity. Offers AI website generation (Wix ADI), but the interface can still be cluttered and overwhelming.
  - **Squarespace**: Medium setup complexity. Good for static portfolios but lacks true business management depth for beginners.
  - **Durable & 10Web**: AI-native platforms claiming rapid setup ("under 30 seconds"), but often lack the deep operational layers needed for actual business management.
- **User Needs**: Users demand a "Time-to-Live" of under 10 minutes. They want an automated system that infers the heavy lifting from minimal input. They need an "employee" (an autonomous agent) rather than a "toolkit" (a manual builder).
- **OHC Opportunity**: OHC can dominate by shifting the paradigm from "manual configuration" to "conversational generation." By leveraging AI as an invisible "Department," OHC can proactively generate the entire business engine (storefront, payments, CRM, bookings) through a simple conversational interface, bypassing complex dashboards entirely.

## Design Doc
### High-Level Architecture
- **Trigger**: The user initiates onboarding via a conversational interface (e.g., chat or voice input).
- **Agent Action (The Onboarder)**:
  - Engages the user in a natural conversation to gather minimal essential details (e.g., "I'm Carlos, a handyman in Seattle. I charge $75/hr and need a booking system.").
  - Extrapolates business metadata and structure.
  - Communicates with other Agent Departments (e.g., Marketing Agent to write copy and select templates, Operations Agent to configure booking/inventory, Finance Agent to set up default pricing and tax settings).
- **UI Flow**: The user interacts with a simple chat interface. As the conversation progresses, a live, mobile-first preview of the storefront dynamically updates.
- **Final Approval**: The user is presented with a complete, fully functional business engine and asked for a "1-Tap Approval" to go live.

### Mobile UX Flow (375px First)
1. **Welcome Screen**: A friendly chat interface greets the user: "Hi! Let's get your business online. Tell me what you do in a few words."
2. **Conversational Input**: The user responds via text or voice.
3. **Dynamic Generation & Loading State**: The UI splits: the top half shows the chat, the bottom half shows a skeleton loading state of the site being built in real-time ("Our team is drafting your site, setting up your booking system...").
4. **Clarification Prompts (Optional)**: The Onboarder agent asks 1-2 targeted questions if crucial information is missing (e.g., "Do you want to accept online payments?").
5. **Review & Launch**: The user views the fully generated mobile storefront. A prominent "Approve & Launch" button allows instant publication.

### AI Agent Integration Points
- **The Onboarder (Conversational Agent)**: Manages user interaction and intent extraction.
- **The Marketing Agent**: Generates SEO-optimized copy, selects layouts, and sources placeholder imagery.
- **The Operations Agent**: Configures backend modules (e.g., Cal.com integration for bookings, Stripe for payments) based on inferred business type.

## Implementation Prompt
Implement the "AI Conversational Store Builder" for the primary onboarding flow. Replace traditional multi-step forms with a chat-based interface powered by "The Onboarder" agent. The system should parse natural language input to autonomously configure a complete OHC business instance—including frontend design, backend operational logic (bookings, inventory), and payment gateways. The flow must dynamically render a live preview and conclude with a single "Approve & Launch" action, achieving a time-to-live of under 10 minutes.

## Priority
P0

## Estimated Scope
Large