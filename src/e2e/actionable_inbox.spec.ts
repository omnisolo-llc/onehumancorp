import { test, expect } from './fixtures';
import { e2eDbQuery } from './db_utils';

test.describe('Actionable Inbox UX flow for owners on mobile', () => {
  test.use({ viewport: { width: 375, height: 667 } });

  test('Receives a message, triage generates a draft, and owner can approve via dashboard feed', async ({ page }) => {
    const tenantId = 'e2e-tenant';
    const message = 'Can I get a custom vegan cake for this weekend?';

    const webhookResponse = await page.request.post('/api/v1/omnichannel/webhook', {
      data: {
        tenant_id: tenantId,
        channel: 'instagram_dm',
        sender_id: 'maya_bakes',
        message: message,
      }
    });

    expect(webhookResponse.ok()).toBeTruthy();

    await page.waitForTimeout(2000);

    await page.goto('/dashboard.html');

    // We should see an Instagram DM actionable item.
    await expect(page.locator("text=Instagram DM").first()).toBeVisible({ timeout: 15000 });

    // Check that we see the customer message
    await expect(page.getByText('Can I get a custom vegan cake for this weekend?').first()).toBeVisible();

    // Check for the Approve & Send button
    const approveBtn = page.getByTestId('approve-instagram-dm').first();
    await expect(approveBtn).toBeVisible();

    await approveBtn.click();

    // The message should disappear. Wait for it to disappear.
    await expect(page.getByText('Can I get a custom vegan cake for this weekend?').first()).not.toBeVisible();
  });
});
