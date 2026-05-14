# OneHumanCorp Onboarding Architecture

This document describes the design and implementation of the "Day One" onboarding experience for the OHC Hybrid Agentic OS.

## Overview

The onboarding experience is designed to be frictionless, mobile-first, and highly intuitive. It guides new users from an initial landing state to a fully configured business with live products, AI-driven descriptions, and automated agent integration in under 10 minutes.

## Core Pillars

1. **Simplicity (Grandmother Test)**: All technical jargon is replaced with plain language.
2. **Instant Gratification**: AI auto-generates content (descriptions, site structure) immediately.
3. **Resilience**: State is persisted at every step to allow cross-device resumption.
4. **Visual Excellence**: Glassmorphic UI with high touch targets (≥44px) and fluid animations.

## State Machine

The onboarding flow follows a strict 9-step linear progression:

1. **Hero Entry**: Value proposition and entry point.
2. **Business Type Discovery**: Selection of industry/niche.
3. **Identity**: Business naming.
4. **Offerings**: Selection of selling categories (Physical, Services, Subscriptions).
5. **Inventory Kickstart**: Adding the first product with AI description generation and photo cropping.
6. **Brand Expression**: Template selection with live business-name preview.
7. **Global Presence**: Domain/Subdomain selection.
8. **Account Finalization**: Core user credentials.
9. **Verification**: Email verification step.
10. **Success & Checklist**: CONFETTI transition and actionable next-steps.

## Technical Implementation

### Frontend

- **Location**: Embedded in `src/server/lib.rs` (UI Handler).
- **Technology**: Vanilla JS for zero-latency, CSS transitions, and Flexbox/Grid for mobile-first responsiveness.
- **Persistence**: Periodic `POST` calls to `/api/onboarding/state` with the full `wizardData` JSON blob.

### Backend

- **API Layer**: `src/server/api/onboarding/mod.rs`.
- **Service Layer**: `src/server/services/onboarding/onboarding_agent.rs`.
- **Database**: `onboarding_state` table (Postgres/SQLite parity).

### AI Generation

The `/api/onboarding/generate-description` endpoint provides high-context product descriptions based on simple names, enabling users to go live with professional-grade content instantly.

## Testing Strategy

- **Unit Tests**: Rust tests for `OnboardingAgent` logic in `src/server/services/onboarding/onboarding_agent.rs`.
- **E2E Tests**: Comprehensive Playwright suites in `src/e2e/onboarding_complete.spec.ts`.

## Future Improvements

- Integrated Meta/Instagram authentication within the wizard.
- Real-time LLM-driven site customization during the "Generating" state.
- Voice-activated onboarding for low-literacy food service personas.
