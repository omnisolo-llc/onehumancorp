# Title: Invisible AI-Driven Conversational Onboarding

## Problem Statement
The process of setting up an online business is daunting for non-technical users like **Fatima (food cart)** and **Maya (baker)**. When trying platforms like Shopify or Wix, they are presented with overwhelming dashboards, complex settings panels, and requests to configure DNS records, payment gateways, and tax settings before they can even see their storefront. The cognitive load is too high, leading to massive drop-off rates. They want to sell their products, not become software administrators. They don't know what a "slug" is, they just want people to buy their cupcakes.

## Research Report
*   **Competitor Gap:**
    *   **Shopify:** Setup requires navigating a complex admin panel. It assumes the user understands e-commerce terminology.
    *   **Wix/Squarespace:** Uses an AI setup flow (e.g., Wix ADI) but it still ultimately drops the user into a traditional, complex editor.
    *   **GoDaddy:** Setup is fast but results in a highly generic, low-quality site.
*   **User Pain Points (Validated from App Store/Reddit Reviews):**
    *   "I just want to add my 5 products and start selling, why do I have to set up all these tax profiles first?"
    *   "The dashboard is too confusing, I don't know where to start."
    *   "I gave up trying to figure out how to link my domain."
*   **The OHC Opportunity:** Instead of a traditional dashboard, onboarding should be a simple conversation with an AI agent. "Hi Fatima, what do you sell?" -> "I sell empanadas." -> "Great! How much do they cost?" -> "$3 each." -> "Perfect. I've created your store and added your first product. Here is your link."

## Design Doc
*   **Core Entity Types:**
    *   `OnboardingSession`: Tracks the user's progress through the conversational flow.
    *   `Tenant`: The business being created.
    *   `Product`: The initial items being added to the catalog.
*   **Key Relationships:**
    *   `OnboardingSession` is linked to a user/tenant.
*   **Mobile UX Flow (375px First):**
    1.  User downloads the app and signs up.
    2.  Instead of a dashboard, they see a clean chat interface.
    3.  The OHC AI greets them: "Welcome to OHC! What's the name of your business?"
    4.  The user types the answer.
    5.  The AI asks a few simple, sequential questions (What do you sell? How much is it? Do you deliver or do pickup?).
    6.  Behind the scenes, the AI configures the necessary entities (Tenant, Product, basic settings).
    7.  The AI presents a "Magic Link" to their live storefront.
*   **AI Agent Integration Points:**
    *   The onboarding conversational flow is entirely driven by an LLM agent configured to extract specific entities (Business Name, Product Name, Price).
    *   The agent uses the extracted entities to trigger internal "mutations" (e.g., `CreateTenant`, `CreateProduct`) without exposing the complexity to the user.

## Implementation Prompt
**User-Facing Outcome:** A user can create a fully functional, ready-to-sell online store entirely by answering 3-4 simple questions in a chat interface, taking less than 2 minutes. They never see a complex settings dashboard during setup.

**Critical User Journey:**
1.  User signs up for OHC.
2.  User enters the conversational onboarding flow.
3.  User answers: "My business is Maya's Bakery."
4.  User answers: "I sell Chocolate Cake for $30."
5.  The AI processes this, creates the tenant, sets up the catalog, and replies: "You're all set! Your store is live at mayasbakery.ohc.store."

**Acceptance Criteria:**
*   A chat-based UI exists for initial onboarding instead of a traditional form.
*   The AI agent correctly extracts business name and product details from natural language input.
*   The system successfully creates a new `Tenant` and at least one `Product` based on the AI's extraction.
*   The user is provided with a working link to their new storefront at the end of the conversation.

## Priority
P0

## Estimated Scope
Medium