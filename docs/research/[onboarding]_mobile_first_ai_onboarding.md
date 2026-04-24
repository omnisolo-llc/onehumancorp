<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%) !important; font-family: 'Outfit', 'Inter', sans-serif !important; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.03) !important;">

# [onboarding] Build Mobile-First, AI-Assisted Unified Onboarding Flow

## Problem Statement
Small business owners (like Maya the Baker or Carlos the Handyman) are overwhelmed by the setup complexity of existing platforms like Shopify and Wix. These platforms require significant time investment, technical knowledge, and are primarily designed for desktop use. There is a critical need for a mobile-first onboarding experience that utilizes autonomous AI agents to generate a complete business presence (products and bookings) in under 10 minutes with zero technical jargon.

## Research Report
Our competitive analysis highlights a significant gap:
- **Shopify & Wix**: Complex onboarding, desktop-centric setup, and conversational AI (chatbots) rather than autonomous setup agents. 73% of 1-star App Store reviews mention the setup being confusing for beginners.
- **GoDaddy**: Fast but shallow; offers basic AI branding but lacks robust post-launch management tools.
- **Target OHC Dominance**: OHC can capture the market by combining a seamless, mobile-first (375px) guided setup with an invisible AI Agent that automatically constructs the storefront, adds initial products/services, and configures basic settings based on a simple conversational interview.

## Design Doc
### Key Entities & Relationships
- **User (Tenant)**: Represents the business owner.
- **AI Agent (The Promoter)**: Responsible for analyzing user input and generating the initial storefront configuration.
- **Store Profile**: Contains business details, branding, and aggregated settings.
- **Catalog Item (Product/Service)**: Unified data model supporting both physical goods (e.g., cakes) and bookable services (e.g., handyman hours).

### UI Wireframes / Flow (375px Mobile-First)
1. **Welcome Screen**: Clean, glassmorphic design introducing "The Promoter" AI.
2. **Interview Flow**: Series of simple, single-question screens using native mobile keyboards (e.g., "What do you sell?", "What is your business name?").
3. **Generation Screen**: Loading state with micro-animations indicating the AI is building the store.
4. **Review & Publish**: A polished preview of the storefront with an "Approve & Go Live" button.

### Architecture Integration
- **Frontend**: Flutter application with a dedicated onboarding router.
- **Backend**: Go API endpoint to receive onboarding parameters and enqueue an AI generation job.
- **AI Coordination**: Utilize the existing AI Job Queue to trigger "The Promoter" agent to draft the storefront layout and initial inventory, storing the state in PostgreSQL.

## Implementation Prompt
**Task**: Implement a mobile-first (375px) conversational onboarding flow where the AI agent ("The Promoter") interviews the user to generate a unified storefront supporting both products and bookings.
**CUJ**: A new user signs up on their iPhone. They are guided through a short interview by the AI. Once completed, the backend agent generates a complete store profile, including sample products/services. The user reviews the generated storefront and taps "Publish" to go live.
**Acceptance Criteria**:
- Develop the mobile-first UI for the onboarding interview flow.
- Implement the backend logic to parse the interview answers and trigger the AI generation job.
- Ensure the AI successfully populates the database with a unified product/booking model.
- Add E2E tests verifying the complete flow from sign-up to storefront publication without mocking network requests.

## Priority
P0

## Estimated Scope
Large

</div>
