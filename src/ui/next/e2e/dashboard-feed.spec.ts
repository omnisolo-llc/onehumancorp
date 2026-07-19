import { test, expect } from '@playwright/test';

test.describe('Dashboard Actionable Feed', () => {
  test('should display database-backed operations console', async ({ page }) => {
    await page.goto('/dashboard');

    await expect(page.getByRole('heading', { name: 'Business Analytics' })).toBeVisible();
    await expect(page.locator('text="Operations Map"').first()).toBeVisible();
    await expect(page.locator('text="Recent Orders"')).toBeVisible();
    await expect(page.locator('text="Inbox Activity"')).toBeVisible();
  });

  test('Assistant-first CUJ: Approve Action from Triage Feed', async ({ page, request }) => {
    const tenantId = 'e2e-tenant-' + Date.now();

    // Create an item via the simulate endpoint
    const simulateResponse = await request.post(`/api/v1/dev/simulate-agent-feed-item?tenant_id=${tenantId}`);
    expect(simulateResponse.ok()).toBeTruthy();

    await page.goto(`/dashboard?tenant_id=${tenantId}`);

    const simulatedCardText = page.locator('text=A new simulated event needs your attention.').first();
    await expect(simulatedCardText).toBeVisible({ timeout: 15000 });

    const card = simulatedCardText.locator('..').locator('..');
    const approveButton = card.locator('button', { hasText: 'Approve' }).first();
    await expect(approveButton).toBeVisible();

    await approveButton.click();
    await expect(simulatedCardText).not.toBeVisible({ timeout: 15000 });
  });
});
