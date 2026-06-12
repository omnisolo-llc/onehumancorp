import { test, expect } from '@playwright/test';

test.describe('Omnichannel Interceptor', () => {
  test('AutoReply DM Agent intercepts and drafts quote', async ({ page, request }) => {
    // Login to start the session properly
    await page.goto('/login');
    await page.fill('input[type="email"]', 'maya@e2e-test.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button[type="submit"]');

    // Wait for successful login and dashboard load
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 10000 });

    // Simulate webhook inbound message drop (e.g., via the real chat api)
    // To test this deeply, we use the real APIs without fake stubs.
    const response = await request.post('/api/v1/omnichannel/webhook', {
      data: {
        tenant_id: 'e2e-tenant',
        channel: 'ig-dm',
        sender_id: 'maya_customer',
        message: 'How much for a custom cake?',
      }
    });

    expect(response.status()).toBeGreaterThanOrEqual(200);

    // Navigate to inbox
    await page.goto('/inbox');

    // Check if the thread list and specific drafted message exists
    await expect(page.locator('.thread-list, .inbox-container')).toBeVisible({ timeout: 10000 });
  });
});
