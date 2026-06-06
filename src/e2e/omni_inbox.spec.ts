import { test, expect } from './fixtures';

test.describe('Omnichannel AI Unified Inbox', () => {
  test('Agent drafts reply and user approves it', async ({ page, request }) => {
    await page.goto('/inbox');
    await expect(page.getByRole('heading', { name: 'Inbox' })).toBeVisible();

    const webhookResponse = await request.post('/api/agents/webhook', {
      data: {
        tenant_id: 'e2e-tenant',
        message: 'Do you sell vegan cakes?',
        source: 'instagram',
      }
    });
    expect(webhookResponse.ok()).toBeTruthy();

    await page.waitForTimeout(2000);
    await page.reload();

    await expect(page.getByText('instagram', { exact: false }).first()).toBeVisible();
    await expect(page.locator('text="Yes, we do vegan cakes!"').first()).toBeVisible();

    const approveBtn = page.locator('button:has-text("Approve & Send Draft")').first();
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    await expect(page.getByText('sent').first()).toBeVisible();
    await expect(approveBtn).not.toBeVisible();
  });
});
