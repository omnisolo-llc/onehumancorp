import { test, expect } from '@playwright/test';

test.describe('Unified Agent Feed Offline Mode', () => {
  test('optimistically handles actions when offline and syncs when back online', async ({ page, context }) => {
    // Navigate to dashboard
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible({ timeout: 20000 });

    // Ensure we are on the dashboard
    await page.goto('/dashboard');

    // Make sure we have the feed
    await expect(page.locator('text=Proposals').first()).toBeVisible();

    // Trigger an agent action via the backend webhook to generate a proposal
    const apiBase = process.env.OHC_API_URL || process.env.BACKEND_URL || process.env.BASE_URL || '';
    const tenantId = 'e2e-tenant';
    const webhookPayload = {
      tenant_id: tenantId,
      message: 'I need to book a plumbing repair for tomorrow.',
      source: 'instagram'
    };
    const response = await page.request.post(`${apiBase}/api/agents/webhook`, {
      data: webhookPayload,
    });
    expect(response.ok()).toBeTruthy();

    // Wait for the proposal to show up
    await page.reload();
    await expect(page.locator('text=Action Needed').first()).toBeVisible({ timeout: 25000 });

    const approveButton = page.locator('button', { hasText: 'Approve' }).first();
    await expect(approveButton).toBeVisible();

    // Now let's simulate going offline
    await context.setOffline(true);

    // Click "Approve" while offline
    await approveButton.click();

    // 1. Ensure the card optimistically disappears from the feed
    await expect(approveButton).toBeHidden();

    // 2. Ensure "Pending Sync" badge appears
    await expect(page.locator('text=/Pending Sync \\(1\\)/')).toBeVisible();

    // Now let's come back online
    await context.setOffline(false);

    // Fire the online event manually for Playwright in case it doesn't automatically propagate
    await page.evaluate(() => window.dispatchEvent(new Event('online')));

    // 3. Ensure the "Pending Sync" badge disappears (sync successful)
    await expect(page.locator('text=/Pending Sync \\(1\\)/')).toBeHidden({ timeout: 10000 });
  });
});
