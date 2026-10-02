# Onboarding without a model provider

AI-assisted intake, chat completion and zero-click generation must not invent a
business catalog when no provider is configured. Their authenticated endpoints
return HTTP 503 with `error: onboarding_ai_unconfigured` and a message directing
the owner to review and enter business details manually. No preparation, catalog,
customer, price, deposit term or launch is created by this unavailable response.

The existing authenticated `/api/v1/onboarding/start` accepts explicit reviewed
business and product fields independently of model availability. It preserves the
existing owner/tenant checks, durable preparation receipt and separate launch step.
The maintained wizard's manual continuation is paired client work: it must preserve
the user's inputs, require any missing fields and never substitute mock products or
prices. Arbitrary HTTP 503 responses do not authorize advancing to review.

Verification uses the existing source-bound onboarding durability harness: complete
production intake/chat methods and API source, real signed authentication and
PostgreSQL transactions, with configured-provider invocation as a panic-only test
boundary. It covers missing-provider responses and no mutation as well as the real
manual preparation path. It does not certify an actual model/provider, browser
execution, public site publication, or the full repository acceptance suite.

## Maintained legacy setup pages

All five maintained setup HTML copies offer an explicit manual-review action only
for an unambiguous HTTP503 `onboarding_ai_unconfigured` response. Other failures
stay in their existing error/unknown-outcome flow. The action retains the original
chat or instant prompt alongside the owner's existing form fields; it does not
prepare or launch anything. Changing accounts retires the prompt and controls.

The manual offer step requires the owner to enter a price. Blank input never
becomes a zero-price product; an explicitly entered zero remains supported. The
existing `firstProductPrice` draft key preserves that input through owner-scoped
local/remote saves and reload. Finish validates the offer and price before the
existing receipt-checked preparation/launch flow. Browser journey fixtures enter
their own synthetic test price rather than depending on an invented default.

User and actual assistant chat messages are saved immediately to the current
owner's local draft. These snapshots remain pending until the existing draft
write protocol acknowledges them. A stalled or unavailable provider cannot erase
the user's message on reload, and storage failure keeps its existing visible
warning. These checks use actual HTML and deterministic response boundaries;
they do not certify a live model, hosted browser, or public publication.

A pending chat reply or an unconfigured instant-intake continuation also checks
that the owner's persisted draft fields still match the captured request view.
If another view changed those fields, setup holds without saving its older form
or adopting the newer fields. Metadata-only acknowledgement changes do not discard
an otherwise-current response. This prevents the new immediate history snapshot
and manual navigation from overwriting an intervening local edit.
