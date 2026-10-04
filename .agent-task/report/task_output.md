outcome: no_work
issue_title: Mobile Tap-to-Pay & Omnichannel POS Integration
issue_description: The request asks to build a Stripe Terminal POS flow for mobile. The current codebase already has Stripe Terminal POS flow fully integrated using Next.js proxy routes that normalize real backend token and card-present PaymentIntent responses for Terminal JS (Resolves #22353 and #22279). The E2E tests for POS terminal are also present in `src/ui/next/src/e2e/pos_terminal.mock-contract.ts` and `src/ui/next/src/components/pos/terminal/TapToPayOverlay.tsx` uses the `/api/v1/pos/terminal/connection-token` endpoint. No new work is required as the integration is already complete and working.
issue_priority: P1
issue_category: ""
issue_type: ""
issue_label: ""
assignees: ""
