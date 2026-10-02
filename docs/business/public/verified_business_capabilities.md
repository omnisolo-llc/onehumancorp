# OmniSolo Business Capabilities and Operating Guide

This document explains how an owner accomplishes verified business work with the current OmniSolo product, in plain language, based on the active capability map and audit remediation ledger.

## Setup and Onboarding
An owner starts by providing their business details to establish a tenant identity. The workspace relies on tenant scoping to ensure isolation across data and actions.

## Connected Accounts
OmniSolo supports connecting essential business tools. Currently, supported connections include Stripe for payments and OpenAI for API-based LLM access. You retain control of your external accounts and can revoke access anytime.

## Standing Authority & AI Delegation
Your in-product AI team handles routine work based on standing authority. For example, it can draft proposals or invoices based on your inputs. The system enforces strict boundaries: agents cannot spend money, modify external provider data, or approve out-of-policy exceptions without your explicit authorization. Any request outside this granted authority halts the action and flags a requirement for your review.

## Evidence of Work
Every action performed by your AI team produces durable evidence. Inquiries lead to persisted quotes; approved work results in a drafted invoice or delivery artifact. The system prevents duplicate usage accounting and ensures one event is counted once. Unresolved exceptions or unknown provider states are clearly surfaced instead of being treated as completed tasks.

## Cost and Billing
OmniSolo operates under a managed cost model where infrastructure, tooling, and model inference are separated. For inference, you may use managed API access or provider-permitted customer API keys (BYOK). The platform tracks usage reliably and associates it with your tenant and payer profile. The system ensures budgets are atomically reserved before work begins. Hard caps guarantee that costs never exceed the funds authorized.

## Exceptions and Recovery
When things don't go as planned—such as a declined payment, an exhausted budget, or a revoked credential—the system safely pauses. The AI team preserves the state and receipts so you can review the error, address the dependency, and restart the workflow safely. Duplicate events are filtered to prevent duplicate charging.

## Research Scope and Evidence Baseline
* **Source Dates**: Baseline established from the September 18, 2026 usage audit and native migration review.
* **Study Populations**: Currently focusing on single-owner service businesses as candidates for verification.
* **Uncertainties**: Actual per-workflow serving costs, willingness to pay, and provider subscription usage limits are yet to be validated with real-world workloads.
* **Metric Definitions**: Compute includes CPU, memory, and API waits. Business milestones are tracked from inquiry to cash delivery.
* **Scope**: This document covers only natively built and tested capabilities within the current `make test` acceptance gates. Legacy or unverified placeholders (like generated checkout links) are explicitly excluded from being represented as active features.

## Superpowers Audit Trail
* **Loaded Skills**: `using-superpowers` (rev `5bf4e78011075bcfc0dc295f0724994cd123ee71`), `brainstorming`, `writing-plans`, `executing-plans`.
* **Outcome**: A definitive source of truth document generated matching Automator standards.
