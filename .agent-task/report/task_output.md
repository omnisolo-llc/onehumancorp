# Architecture Brief: Multi-Tenant SaaS Tier Design

## Title
OHC Multi-Tenant SaaS Tier Architecture

## Problem Statement
Small business owners (Maya, Carlos, Priya) need a pricing model that scales transparently with their growth. The platform currently lacks a formalized multi-tenant tier system to handle feature gating and usage limits. Without this, OHC cannot effectively monetize the platform while still offering a genuinely useful free tier that demonstrates value before requiring an upgrade. We need an architectural design for enforcing these limits and gracefully up-selling users without resorting to technical errors or aggressive paywalls.

## Research Report
- **Competitive Landscape**:
  - Shopify lacks a true free tier and relies heavily on complex 3rd-party app subscriptions.
  - Wix and Squarespace have ad-supported free tiers and confusing, feature-gated upgrade paths.
- **OHC Advantage**: OHC can provide a completely functional Free tier by gating on volume (products, AI actions) rather than critical features. This allows non-technical users to build their business and only pay once they achieve actual velocity.
- **Key Consideration**: AI operations are expensive. Strict throttling must be tied to SaaS tiers, but the user experience should remain helpful. Instead of a hard error, "The Business Advisory" agent should proactively recommend upgrades.

## Design Doc

### Tier Structure Matrix
| Tier | Price | Products | AI Departments | AI Actions/mo | Storage | Custom Domain |
|---|---|---|---|---|---|---|
| **Free** | $0/mo | 10 | 1 | 100 | 500MB | No (OHC Subdomain) |
| **Starter** | $9/mo | 100 | 3 | 1,000 | 5GB | Yes |
| **Pro** | $29/mo | Unlimited | 10 | Unlimited | 50GB | Yes + SSL |
| **Business** | $79/mo | Unlimited | Unlimited | Unlimited | 500GB | Yes + Multi-domain |

### High-Level Architecture (Mermaid.js)
```mermaid
graph TD
    UserAction[User/Agent Action] --> TierMW[Tier Middleware]
    TierMW -->|Check Limits| DB[(PostgreSQL)]
    DB -.->|Tenant Tier Info| TierMW

    TierMW -->|Under Limit| Orchestrator[KAIROS Orchestrator]
    Orchestrator --> Agent[Target Agent]

    TierMW -->|Over Limit| Intercept[Limit Exceeded Event]
    Intercept --> UIM[UI Upgrade Prompt]
    Intercept --> Advisor[The Advisor Agent]
    Advisor -->|Proactive Pitch| Dashboard[Dashboard Notification]

    UIM --> Stripe[Stripe Checkout]
    Stripe -->|Webhook| WebhookHandler[Billing Webhook Service]
    WebhookHandler -->|Update Tier| DB

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class UserAction,TierMW,Orchestrator,Agent,Intercept,UIM,Advisor,Dashboard,Stripe,WebhookHandler,DB premium;
```

### Mobile UX Flow (Graceful Degradation)
1. **Trigger**: Carlos tries to add his 101st product on the Starter Tier.
2. **Intercept**: The API responds with a `402 Payment Required` wrapped in a specific `LimitExceeded` struct.
3. **UI Display**: Instead of an error toast, a bottom-sheet modal (Glassmorphism styling) appears: *"Your business is growing! You've reached your 100 product limit. Upgrade to Pro to add unlimited products."*
4. **Action**: 1-Tap "Upgrade Now" initiates a mobile Stripe Checkout session.

### AI Agent Integration Points
- **The Advisor Agent**: Monitors usage metrics. If a user on the Free tier has used 90/100 AI actions by the 15th of the month, the Advisor sends a friendly dashboard notification suggesting a preemptive upgrade.
- **Enforcement**: Agents themselves are subject to limits. If an agent tries to execute an action and the limit is reached, it yields and creates a task for the owner to review the upgrade.

### Key Design Decisions
1. **Middleware Enforcement**: Limits are checked at the `TierService` middleware level to ensure uniform enforcement across all HTTP endpoints and internal RPC calls.
2. **Decoupled Billing**: Stripe is the source of truth for subscription status, synced asynchronously via webhooks to prevent the critical path of the application from depending on a third-party API.
3. **Optimistic Upsell**: Focus on the value of the upgrade rather than the restriction of the limit.

## Implementation Prompt
**To Implementer Agent:**
Implement the multi-tenant SaaS tier enforcement engine. Create a `TierService` middleware in the Rust/Go backend that intercepts requests and verifies the current tenant's tier against their usage limits (e.g., product counts, AI actions). Define the tier structures (Free, Starter, Pro, Business) within the database schema. Integrate with Stripe webhooks to handle asynchronous subscription state updates. Ensure that when a limit is exceeded, the API returns a structured, plain-language payload that the frontend can use to display an intuitive upgrade prompt. Use OHC design tokens (Outfit font, Glassmorphism) for any generated UI components.

## Priority
P0

## Estimated Scope
Medium
