import { test, expect } from '@playwright/test';

test.describe('Omnichannel Work Triage CUJ - High Priority', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('Maya can dismiss high priority messages', async ({ page }) => {
    test.setTimeout(180000);

    // 1. Log in
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    const tenantId = await page.evaluate(() => localStorage.getItem('tenant_id') || 'e2e-tenant');

    const seedData = [
      {
        source: 'Instagram DM',
        priority: 'urgent',
        context: 'Message: Urgent issue with cake delivery.',
        action_type: 'Draft Reply',
        action_payload: 'I am looking into this right now.',
        customer_id: 'maya_urgent_cust'
      }
    ];

    for (const data of seedData) {
      await page.request.post(`/api/v1/triage/create?tenant_id=${encodeURIComponent(tenantId)}`, {
        data
      });
    }

    await page.goto('/triage');
    await expect(page.locator('body')).toContainText(/Work Triage/, { timeout: 15000 });

    const card = page.locator('div[data-testid^="triage-card-"]', { hasText: 'urgent' }).first();
    await expect(card).toBeVisible();

    const testId = await card.getAttribute('data-testid');
    const header = card.locator(`[data-testid="triage-card-header-${testId?.replace("triage-card-", "")}"]`);
    await header.click();

    const dismissBtn = card.locator(`button[data-testid="triage-dismiss-${testId?.replace("triage-card-", "")}"]`);
    await dismissBtn.waitFor({ state: 'visible' });
    await dismissBtn.click();

    await expect(card).not.toBeVisible({ timeout: 15000 });
  });
});
