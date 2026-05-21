# Comprehensive Architecture Research Report

This report synthesizes the end-to-end architectural research conducted for OneHumanCorp (OHC), focusing on core business journeys, data modeling, multi-tenancy, AI orchestration, and mobile-first storefront building. The ultimate goal is to enable any non-technical user to launch and grow a business in under 10 minutes.

## 1. Business Journey Architecture
### Key Findings
The journey maps (Acquisition, Onboarding, Activation, Retention, Revenue, Referral) highlighted significant friction points for non-technical personas:
- **Cognitive Overload:** Complex setup forms lead to high abandonment.
- **Technical Jargon:** Terms like "SSL" or "API" intimidate users like Fatima (food cart) or Maya (baker).
### Recommended Architecture
- **Progressive Profiling:** Defer non-critical setup. Only ask for the absolute minimum to get a live, "draft" state storefront.
- **AI-Guided Onboarding:** The "Marketing Agent" acts as the primary wizard, extrapolating storefront metadata from a simple bio input.

## 2. Multi-Tenant Data Model
### Key Findings
Robust isolation is required, but AI agents need cross-table context to be useful.
### Recommended Architecture
- **RLS-First PostgreSQL:** `tenant_id` is mandatory on all tables. All queries must run within a strictly scoped PostgreSQL Row-Level Security session (`SET app.current_tenant = ...`).
- **Semantic Memory:** Use `pgvector` for AI memory (the `autodream_memories` table), allowing agents to perform semantic searches constrained by the `tenant_id` invariant.

## 3. AI Agent Department & Orchestration
### Key Findings
Agents must act autonomously but require human oversight for critical actions to maintain trust.
### Recommended Architecture
- **KAIROS Event Mesh:** A unified, event-driven mesh handles department coordination (e.g., Ops finishes order -> triggers Success agent to send email).
- **1-Tap Approval Workflow:** High-risk actions (external emails, refunds, social posts) are drafted into a pending state. The business owner receives a mobile push notification and approves the action with a single tap in the dashboard.

## 4. Mobile-First Storefront Builder
### Key Findings
Users need a site that is "born live." Traditional block builders are too complex for a 375px screen.
### Recommended Architecture
- **Smart Blocks:** Pre-configured vertical blocks (Hero, Menu, Booking) that automatically adapt to the business type.
- **Vibe Coding:** AI selects palettes and typography based on business metadata.
- **Performance:** Strict targets of LCP < 1.5s and FID < 100ms, using progressive JPEGs and a minimal initial payload.

## 5. Multi-Tenant SaaS Tiers
### Key Findings
Monetization requires a transparent tier system that focuses on volume, not feature-gating, to allow users to experience value before upgrading.
### Recommended Architecture
- **TierService Middleware:** Enforces usage limits at the API layer based on the user's tier (Free, Starter, Pro, Business).
- **Graceful Degradation:** Reaching a limit triggers a plain-language upgrade prompt rather than a hard technical error.
- **Proactive Upgrades:** The Business Advisory agent intelligently suggests upgrades when a user approaches a limit.

## Next Steps
All identified gaps have been documented as actionable Issue Briefs in the `docs/research/` directory. The engineering swarm should begin implementation starting with the P0 priority issues (Mobile-First Review, Data Model Evolution, and Smart Builder).
