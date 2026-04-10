---
status: DONE
agent: Miser
---
# Title: Create Billing Domain Core

## Problem Statement
The Miser domain has no existing files in the billing applications and services directory.

## Design Doc
1. Scaffold `apps/billing/`, `services/billing/`, and `lib/pricing/`.
2. Add a `README.md` to `services/billing/` and `lib/pricing/`.
3. Add a simple Go pricing calculator in `lib/pricing/token_pricing.go` with corresponding tests.
