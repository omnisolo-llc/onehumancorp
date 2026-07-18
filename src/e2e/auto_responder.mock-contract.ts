import { test, expect } from './fixtures';

test.describe('Auto-Responder Integration', () => {
  test('Webhook message enqueue and UI verification', async ({ page, request }) => {
    // 1. Simulate a webhook by sending a request to the Meta webhook handler
    const res = await request.post('/api/v1/webhooks/meta', {
      headers: {
        'Content-Type': 'application/json',
      },
      data: {
        "object": "instagram",
        "entry": [
          {
            "messaging": [
              {
                "sender": {"id": "1234"},
                "recipient": {"id": "e2e-tenant"},
                "message": {"text": "Do you make vegan cakes?"}
              }
            ]
          }
        ]
      }
    });

    expect(res.status()).toBe(200);

    // 2. Wait a moment for background processing
    await page.waitForTimeout(4000);

    // 3. Verify the metric on the dashboard
    await page.goto('/dashboard');
    // Check if metric-auto-replied increments
    await expect(page.locator('#metric-auto-replied')).toBeVisible({ timeout: 10000 });
    const countText = await page.locator('#metric-auto-replied').textContent();
    const count = parseInt(countText || '0', 10);
    expect(count).toBeGreaterThan(0);

    // 4. Verify AI Handled badge in Triage view
    await page.goto('/triage');
    await expect(page.locator('text=AI Handled')).toBeVisible({ timeout: 10000 });
  });
});
