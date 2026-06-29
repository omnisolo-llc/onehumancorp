import { test, expect } from '@playwright/test';

test.describe('Unified Agent Inbox - Instagram DM CUJ', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should receive an Instagram DM webhook, triage it, show an Action Card, edit it, and approve it', async ({ page }) => {
    test.setTimeout(120000); // Allow time for worker processing

    // 1. Setup Tenant and Auth
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    const tenantId = await page.evaluate(() => localStorage.getItem('tenant_id') || 'e2e-tenant');
    const senderId = 'jane_baker_' + Date.now();

    // 2. Inject Webhook
    // Send a webhook payload mimicking an Instagram DM to /api/v1/omnichannel/webhook
    const webhookRes = await page.request.post('/api/v1/omnichannel/webhook', {
      headers: {
        'x-tenant-id': tenantId
      },
      data: {
        tenant_id: tenantId,
        source: 'Instagram DM',
        senderId: senderId,
        message: 'Do you have vegan vanilla cupcakes for this Saturday?',
      }
    });

    expect(webhookRes.ok()).toBeTruthy();

    // 3. Navigate to Unified Agent Feed / Dashboard
    await page.goto('/dashboard');

    // Switch to active proposals tab if available
    const proposalsTab = page.locator('button', { hasText: /Proposals/i }).first();
    if (await proposalsTab.isVisible()) {
      await proposalsTab.click();
    }

    // 4. Wait for Action Card to appear (Backend worker must process it)
    // The feed should eventually show the Instagram DM card.
    const dmCard = page.locator('div[data-testid="instagram-dm-card"]');
    await expect(dmCard).toBeVisible({ timeout: 60000 });

    // 5. Verify Content
    await expect(dmCard).toContainText(`Instagram DM from @${senderId}`);
    await expect(dmCard).toContainText('Do you have vegan vanilla cupcakes');

    // 6. Test Edit Flow
    const editBtn = dmCard.locator('button[data-testid="edit-instagram-dm"]');
    await expect(editBtn).toBeVisible();
    await editBtn.click();

    const editTextarea = dmCard.locator('textarea[data-testid="edit-instagram-dm-textarea"]');
    await expect(editTextarea).toBeVisible();

    // Modify the draft content
    await editTextarea.fill('Yes we do! That will be $45. Here is the payment link.');

    // 7. Approve
    const approveBtn = dmCard.locator('button[data-testid="approve-instagram-dm"]');
    await expect(approveBtn).toContainText('Save & Send');
    await approveBtn.click();

    // 8. Verify Optimistic removal / completion
    await expect(dmCard).not.toBeVisible({ timeout: 15000 });
  });
});
