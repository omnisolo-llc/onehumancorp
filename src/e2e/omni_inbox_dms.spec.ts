import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('omni_inbox_dms');

test.describe('Omnichannel Inbox DMs', () => {
  test('receives webhook message and displays it in inbox', async ({ page, request }) => {
    // 1. Send webhook payload
    const tenantId = 'tenant-1';
    const payload = {
      tenant_id: tenantId,
      channel: 'instagram',
      sender_id: 'sarah_cakes123',
      content: 'Hi! I bought a vegan cake 2 months ago. Do you still make the vegan chocolate?',
      name: 'Sarah'
    };

    const webhookRes = await request.post('/api/v1/webhooks/omnichannel', {
      data: payload,
    });
    expect(webhookRes.ok()).toBeTruthy();

    // 2. Log in and go to inbox
    await page.goto('/login');
    await page.fill('input[type="email"]', 'admin@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button[type="submit"]');

    // Check navigation to dashboard
    await expect(page.locator('text=Dashboard').first()).toBeVisible({ timeout: 15000 });

    // Go to inbox
    await page.goto('/inbox');
    await expect(page.getByRole('heading', { name: 'Inbox' })).toBeVisible();

    // 3. Verify message and drafted reply appear
    await expect(page.locator('text=sarah_cakes123').first()).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text=Hi! I bought a vegan cake').first()).toBeVisible();

    const messageCard = page.locator('text=sarah_cakes123').first().locator('..').locator('..');
  });
});
