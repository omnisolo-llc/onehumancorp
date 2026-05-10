# Researcher Report: AI Agent Department and SaaS Tier Architecture

## Overview
As the Principal Product Architect, I have defined and refined the architectural blueprints for OneHumanCorp's core AI departments and its multi-tenant SaaS tiering system. These designs ensure that the platform remains "Grandmother-Tested" while providing sophisticated, invisible automation for SMB owners like Maya, Carlos, and Fatima.

## Key Findings & Strategic Gaps
- **Financial Fog**: Small business owners lack a bridge between raw transactions and actionable financial health. "The Accountant" department bridges this by providing plain-language bookkeeping and proactive cash-flow advice.
- **Conversion Friction**: Every minute of lag in replying to an inquiry is a lost sale. "The Salesperson" automates the lead-to-order flow via context-aware quoting and persistent (yet friendly) follow-ups.
- **Legal Anxiety**: Compliance is often seen as a "black art." "The Protector" makes legal safety a 1-tap experience by generating contextual policies (e.g., allergen disclaimers for food, liability waivers for services).
- **Tier Transparency**: OHC's competitive advantage lies in a "Genuine Free Tier" where limits are based on volume (products/actions) rather than feature gating.

## Delivered Architecture Briefs

### 1. The Accountant (Finance & Payments)
- **File**: `docs/research/[finance]_the_accountant_architecture.md`
- **Focus**: Invisible bookkeeping, automatic reconciliation, and plain-language weekly reports.
- **Priority**: P1

### 2. The Salesperson (Sales & Acquisition)
- **File**: `docs/research/[sales]_the_salesperson_architecture.md`
- **Focus**: Autonomous quote generation from inquiries and lead follow-up loops.
- **Priority**: P1

### 3. The Protector (Legal & Compliance)
- **File**: `docs/research/[legal]_the_protector_architecture.md`
- **Focus**: Contextual policy generation (GDPR, Liability, Allergen) and safety score monitoring.
- **Priority**: P2

### 4. Multi-Tenant SaaS Tiers (Refinement)
- **File**: `docs/research/[architecture]_multi_tenant_saas_tiers.md`
- **Focus**: Formalized tier limits and gRPC-based tier enforcement middleware.
- **Priority**: P0

## Verification Results
- **Documentation Audit**: All new files adhere to the mandatory structure (Title, Problem Statement, Research Report, Design Doc with Mermaid diagrams, Implementation Prompt).
- **System Stability**: Verified via `npx @bazel/bazelisk test //src/server:server_test` (PASSED). Documentation changes have zero impact on existing runtime logic but provide the necessary blueprints for implementer agents.

## Next Steps
- Implement "The Accountant" ledger schema and reconciliation logic.
- Build "The Salesperson" inquiry parsing and auto-quote engine.
- Integrate "The Protector" risk scrutinizer into the product/service creation workflow.
