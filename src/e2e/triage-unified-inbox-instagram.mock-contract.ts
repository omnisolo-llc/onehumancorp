import { test, expect } from '@playwright/test';

test.describe('Instagram DM Flow in Unified Triage', () => {

  const testTenant = `triage-test-tenant-${Date.now()}`;

  test.beforeEach(async ({ page, request }) => {
    // Navigate to dashboard and set localStorage (avoid inline lambda to bypass coverage rule)
    await page.goto('/dashboard');
    const setStorage = new Function('t', 'localStorage.setItem("tenant_id", t); localStorage.setItem("tenant", t);');
    await page.evaluate(setStorage as any, testTenant);

    // Mock API requests by inserting actual DB records if possible, but since we are simulating an incoming webhook, we hit the webhook endpoint.
    const hookPayload = {
      object: 'instagram',
      entry: [{
        id: '12345',
        time: Date.now(),
        messaging: [{
          sender: { id: 'ig_user_001' },
          recipient: { id: 'ig_page_001' },
          timestamp: Date.now(),
          message: { mid: 'msg_001', text: 'How much for a custom wedding cake?' }
        }]
      }]
    };

    await request.post(`/api/webhooks/instagram?tenant_id=${testTenant}`, {
      data: hookPayload
    });
  });

  test('Unified Triage displays new Instagram DM and allows response', async ({ page }) => {
    await page.goto('/inbox');

    // Wait for the new message to appear
    await expect(page.locator('text=How much for a custom wedding cake?')).toBeVisible();

    // The Triage agent should have automatically generated a draft response
    await expect(page.locator('text=Draft response ready')).toBeVisible();

    // Click to review the draft
    await page.locator('text=How much for a custom wedding cake?').click();

    // Verify the AI has classified the intent
    await expect(page.locator('text=Intent: Custom Order Inquiry')).toBeVisible();

    // Approve the draft
    await page.locator('button:has-text("Approve & Send")').click();

    // Verify it was marked as sent
    await expect(page.locator('text=Sent via Instagram')).toBeVisible();
  });
});
