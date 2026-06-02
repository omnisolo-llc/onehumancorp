# Research Report: Autonomous AI Cashflow Forecasting Engine

## Problem Statement
Small businesses struggle primarily with cashflow management. Existing tools like QuickBooks or Xero are passive and require high financial literacy. OHC has an opportunity to leverage its unified ledger (Operations, Marketing, Finance) to proactively predict cash gaps and push actionable solutions in plain language to the user.

## Design Doc
1. **Predictive Cashflow Model**: A service in `src/server/services/finance/forecast.rs` that takes historical ledger data (revenue, expenses) and predicts cashflow for the next 30 days.
2. **AI Integration**: "The Accountant" Agent synthesizes these numbers into an actionable plain language alert (e.g., "You might have a $500 shortfall next week").
3. **UI Dashboard Card**: A Financial Health card on the main dashboard showing the forecast and providing a 1-tap solution (like sending invoice reminders or taking an advance).
4. **1-Tap Solution API**: An endpoint to trigger a quick resolution (e.g., `POST /api/finance/resolve-gap`).
