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
