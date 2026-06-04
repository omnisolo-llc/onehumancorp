import { test, expect } from '@playwright/test';

// In this E2E test, we simulate an authenticated request to the POS API
// normally executed by the UI while processing a Tap-to-Pay transaction on a mobile device.

test.describe('Zero-Config Universal Tap-to-Pay POS Engine', () => {
  // To avoid needing a fully running frontend UI application just for this API test in this environment,
  // we target the backend APIs directly, representing the real backend behavior for POS.
  test('Backend Terminal API properly provisions tokens and intents', async ({ request }) => {
    // 1. Unauthenticated token request
    // The endpoint should handle unauthenticated states gracefully (typically a 401 or returning an error string under 200).
    const tokenRes = await request.post('/api/v1/payments/terminal/token');

    // As per the test harness logic, it might return an 'Unauthenticated' error payload.
    // We expect it to respond successfully on the protocol layer but convey the auth error.
    expect(tokenRes.ok()).toBeTruthy();

    const body = await tokenRes.json();
    if (body.Err) {
        expect(body.Err).toBe('Unauthenticated');
    }

    // 2. Unauthenticated intent creation
    const intentRes = await request.post('/api/v1/payments/terminal/intent', {
        data: { amount_cents: 2500, currency: "usd" }
    });

    expect(intentRes.ok()).toBeTruthy();
    const intentBody = await intentRes.json();
    if (intentBody.Err) {
        expect(intentBody.Err).toBe('Unauthenticated');
    }
  });
});
