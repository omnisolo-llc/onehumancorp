outcome: no_work
issue_title: Integrate Stripe Terminal / Tap to Pay for In-Person Payments
issue_description: The Stripe Terminal (Tap to Pay) functionality is already fully implemented. The integration module exists at `src/server/integrations/stripe/terminal.rs` and properly implements `ConnectionTokens` and `PaymentIntents`. The REST API endpoints are active in `src/server/api/terminal_api.rs`. A functional UI client for managing Tap-to-Pay payments using `@stripe/terminal-js` is fully implemented in `src/ui/next/src/app/pos/terminal/StripeTerminalClient.tsx`. There is no remaining scope to implement.
issue_priority: P1
issue_category: ""
issue_type: ""
issue_label: ""
assignees: ""
