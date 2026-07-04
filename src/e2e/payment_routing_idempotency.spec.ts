import { test, expect } from '@playwright/test';

test.describe('Intelligent Payment Routing and Idempotency', () => {
  test('should intelligently route payment and deduplicate retries using idempotency key', async ({ page, request }) => {
    // 1. Log in and navigate to the payments ledger dashboard
    await page.goto('/login');
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button[type="submit"]');
    await page.waitForURL('/dashboard');

    // 2. Navigate to payments page
    await page.goto('/payments');

    // 3. Set payment amount that should trigger ACH (e.g. $1000)
    await page.fill('[data-testid="payment-amount-input"]', '1000');

    // 4. Intercept the network request to verify idempotency key and route
    const requestPromise = page.waitForRequest(req => req.url().includes('/api/payments/intent') && req.method() === 'POST');

    // 5. Initiate the payment
    await page.click('[data-testid="request-payment-button"]');

    // 6. Verify request contains idempotency key
    const req = await requestPromise;
    const headers = req.headers();
    expect(headers['idempotency-key']).toBeTruthy();
    expect(headers['idempotency-key']).toMatch(/^payment-\d+-/);
    const firstIdempotencyKey = headers['idempotency-key'];

    // 7. Verify UI indicates success
    await expect(page.locator('[data-testid="payment-status"]')).toHaveText('Approved');

    // 8. Wait for the idempotency key to be rotated inside React state
    // To prove idempotency works from backend perspective without relying on UI rotation,
    // we make a direct API call mimicking a network retry with the SAME idempotency key.

    // Extract cookies to authenticate direct API call
    const cookies = await page.context().cookies();
    const cookieHeader = cookies.map(c => `${c.name}=${c.value}`).join('; ');

    const retryRes = await request.post('/api/payments/intent', {
      data: {
        amount: 1000,
        currency: 'USD',
        source: 'tap_to_pay'
      },
      headers: {
        'Content-Type': 'application/json',
        'Idempotency-Key': firstIdempotencyKey,
        'Cookie': cookieHeader
      }
    });

    expect(retryRes.status()).toBe(200); // Idempotent success is 200 OK, not 201 CREATED

    const body = await retryRes.json();
    expect(body.idempotency_key).toBe(firstIdempotencyKey);
    // Assert Intelligent Routing: $1000 should route to Ach
    expect(body.optimal_payment_method).toBe('Ach');
  });
});
