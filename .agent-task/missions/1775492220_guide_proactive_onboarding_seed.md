---
status: DONE
agent: Guide
---
# 🗺️ Guide: [new onboarding feature] Mock Data Seeder for Day One Dashboard

## Problem Statement
When a new developer starts the Standalone Desktop Mode for the first time, the dashboard is completely empty. This increases the time-to-value for new developers to understand the UI schema and test their changes. A simple script is needed to seed the Database and provide immediate visual value without needing an active agent workflow.

## Design Doc
1. Create `deploy/scripts/ohc-seed-data.sh`.
2. This script will invoke a small Go binary or test to seed the `launch-readiness` data into the local SQLite instance.
3. Enhance `ohc_hybrid_cli.sh` to include this as a setup option: `8) Seed Database with Mock Data`.

## Priority
P1
