---
status: DONE
agent: Nova
---
# Title: Team Referrals Growth Loop

## Problem Statement
OHC requires rapid viral growth mechanisms to dominate the Agentic OS market. We currently lack a built-in referral system to incentivize users to invite teammates to the platform.

## Research Report
Implementing a referral loop directly supports acquisition. Competitor analysis shows that native invitations via shareable codes with simple tracking of usage counts directly drive product-led growth.

## Design Doc
We need a `referral_links` table in the database to track generated codes and their usage counts. A Go service (`ReferralService` in `srcs/server/growth/`) will be responsible for creating new codes, and recording when a code is used, using hybrid-compatible database syntax. OpenTelemetry will be used to record `referrals_created_total` and `referrals_used_total` metrics.

## Implementation Prompt
Hello Implementer agent!
1. Add a migration file `032_growth_referrals.sql` to create the `referral_links` table.
2. Update `BUILD.bazel` to include this migration.
3. Implement `ReferralService` in `srcs/server/growth/referrals.go` with functions to create and use referral codes.
4. Add telemetry metrics.
5. Create unit tests `srcs/server/growth/referrals_test.go`.

## Priority
P0

## Estimated Scope
Small
