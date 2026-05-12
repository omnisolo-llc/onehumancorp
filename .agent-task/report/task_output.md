# OHC Core Architecture Research Report

## Executive Summary
This report defines the comprehensive architecture for OneHumanCorp (OHC), framing every technical decision through the lens of non-technical small business owners (Maya the Baker, Carlos the Handyman, Priya the Boutique Owner, Leo the Music Tutor, and Fatima the Food Cart Operator). The goal is to enable a zero-to-live business launch in under 10 minutes from a mobile device, with AI invisibly managing complexity.

## 1. Business Journey Architecture
The user journey from Acquisition to Referral is mapped to specific platform actions to eliminate friction.
*   **Acquisition & Onboarding:** Progressive profiling requests only the minimum required data. AI handles initial storefront generation.
*   **Activation:** The critical "Day 1" milestone—a live storefront or first booking.
*   **Retention & Revenue:** AI-driven plain-language insights trigger engagement and suggest upgrades when limits are approached.
*   **Referral:** Built-in viral loops embedded in the storefront footer and social sharing mechanics.

## 2. Data Model Architecture
The data model uses a "Shared Database, Shared Schema" approach protected by PostgreSQL RLS.
*   **Strict Tenancy:** The `tenant_id` invariant must be enforced across all queries.
*   **AI Memory Layer:** `pgvector` integrations securely handle episodic to long-term memory transition (AutoDream), enabling context-aware AI interactions.

## 3. AI Agent Department Architecture
AI operations are compartmentalized into distinct "Departments" that mirror real-world business roles.
*   **Trigger Models:** Agents act on Cron schedules, Event meshes (Teammate Mesh), or On-Demand requests.
*   **Approval Gates:** High-risk actions use a "Draft-for-Review" mechanism with 1-tap mobile approval.

## 4. Website & Storefront Builder Architecture
The "Smart Builder" replaces technical site building with "Vibe Coding."
*   **Smart Blocks:** Responsive modules tailored to business type (e.g., booking calendars for services, menus for food).
*   **Instant Publishing:** The `DRAFT` to `LIVE` transition triggers automated subdomain and SSL provisioning.

## 5. Mobile-First Architecture
All experiences must pass the "Grandmother Test" for speed and reliability, particularly on slow connections.
*   **Optimistic UI:** Local SQLite SIPDB enables offline-first drafting and instant UI feedback.
*   **Performance:** LCP targets < 1.5s via lightweight payloads.

## 6. Multi-Tenant SaaS Tier Architecture
A fair monetization strategy based on volume, not complex feature gating.
*   **Tiers:** Free, Starter, Pro, Business.
*   **Graceful Limits:** Exceeding tier thresholds triggers friendly "Business Advisory" upgrade suggestions rather than hard errors.

## Next Steps
Detailed issue briefs have been created in `docs/research/` to guide implementer agents on the specific actions required for each architectural domain.
