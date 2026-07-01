# Spec - Next.js API Mock Elimination

Ensure all remaining Next.js API route handlers fail closed and delegate entirely to the backend rather than returning local mock data.

## Proposed Changes

### Billing Checkout Route
- **[route.ts](file:///home/kevin/mono/src/ui/next/src/app/api/billing/create-checkout-session/route.ts)**: Remove mock fallback checkout URL on fetch failure; return `503` instead.
- **[route.test.ts](file:///home/kevin/mono/src/ui/next/src/app/api/billing/create-checkout-session/route.test.ts)**: Update test to expect `503` instead of `/checkout?tier=Starter`.

### Send Receipt Route
- **[route.ts](file:///home/kevin/mono/src/ui/next/src/app/api/v1/growth/campaign/send-receipt/route.ts)**: Remove mock receipt email body on fetch failure or invalid response. Return `502` / `503` on backend failure.
- **[route.test.ts](file:///home/kevin/mono/src/ui/next/src/app/api/v1/growth/campaign/send-receipt/route.test.ts)** (if exists): Update test to assert failing closed.

### Wrapped Route
- **[route.ts](file:///home/kevin/mono/src/ui/next/src/app/api/v1/growth/wrapped/route.ts)**: Remove mock year-in-review analytics fallback; return `502` / `503` if backend fetch fails.
- **[route.test.ts](file:///home/kevin/mono/src/ui/next/src/app/api/v1/growth/wrapped/route.test.ts)** (if exists): Update test to assert failing closed.

### Kitchen Order Translation Route
- **[route.ts](file:///home/kevin/mono/src/ui/next/src/app/api/kitchen/orders/translate/route.ts)**: Since this is purely a simulated endpoint, change it to return an error/fail closed or forward to an actual backend translation service if one is available.
- **[route.test.ts](file:///home/kevin/mono/src/ui/next/src/app/api/kitchen/orders/translate/route.test.ts)** (if exists): Update test to match.

## Verification Plan
- Run related unit tests for the modified routes and check that they all pass after mock removal.
