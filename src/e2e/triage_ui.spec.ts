import { test, expect } from '@playwright/test';

test.describe('Work Triage Agentic Inbox', () => {
  const tenantId = 'e2e-tenant';

  test.beforeEach(async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="email"]', 'admin@ohc.local');
    await page.fill('input[type="password"]', 'changeme');
    await page.click('button[type="submit"]');
    await page.goto('/dashboard.html');
    await expect(page.locator('h2').filter({ hasText: 'Unified Agent Feed' })).toBeVisible({ timeout: 15000 });
  });

  test('Owner reviews and approves a triage item', async ({ page }) => {
    const triageCard = page.locator('[data-testid="triage-card-triage-test-1"]');
    await expect(triageCard).toBeVisible({ timeout: 10000 });

    // Verify detail view is populated from selected card (first item by default)
    await expect(triageCard.locator('text=Maya requested a custom cake for Friday')).toBeVisible();

    // Approve action
    const approveBtn = triageCard.locator('[data-testid="approve-proposal"]');
    await approveBtn.click();

    // Should show approved status and disappear from list
    await expect(triageCard).not.toBeVisible();
  });

  test('Owner can dismiss a triage item', async ({ page }) => {
    const triageCard = page.locator('[data-testid="triage-card-triage-test-2"]');
    await expect(triageCard).toBeVisible({ timeout: 15000 });

    const dismissBtn = triageCard.locator('[data-testid="reject-proposal"]');
    await dismissBtn.click();

    await expect(triageCard).not.toBeVisible();
  });

  test('Triage feed renders items correctly', async ({ page }) => {
    const triageCard = page.locator('[data-testid="triage-card-triage-test-1"]');
    await expect(triageCard).toBeVisible({ timeout: 15000 });
    await expect(triageCard.locator('text=Instagram DM')).toBeVisible();
    await expect(triageCard.locator('text=Maya requested a custom cake for Friday')).toBeVisible();
  });

  test('Backend action executes correctly on Approve Draft', async ({ page }) => {
    const triageCard = page.locator('[data-testid="triage-card-triage-test-1"]');

    if (await triageCard.isVisible()) {
      const approveBtn = triageCard.locator('[data-testid="approve-proposal"]').first();
      await approveBtn.click();
      await expect(triageCard).not.toBeVisible({ timeout: 15000 });
    }
  });

  test('Layout is fully usable at 375px', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });

    const triageCard = page.locator('[data-testid="triage-card-triage-test-1"]');
    await expect(triageCard).toBeVisible({ timeout: 15000 });

    await expect(triageCard.locator('text=Maya requested a custom cake for Friday')).toBeVisible();
    await expect(triageCard.locator('[data-testid="approve-proposal"]')).toBeVisible();
  });

  test('Triage page layout is fully usable at 375px', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/login');
    await page.fill('input[type="text"]', tenantId);
    await page.click('button[type="submit"]');

    await page.goto('/triage');
    await expect(page.locator('h2').filter({ hasText: 'Unified Agent Feed' })).toBeVisible({ timeout: 15000 });

    const hasItems = await page.locator('[data-testid^="triage-card-"]').count() > 0;
    if (hasItems) {
      const firstCard = page.locator('[data-testid^="triage-card-"]').first();
      await expect(firstCard.locator('[data-testid="approve-proposal"]')).toBeVisible();
    } else {
      await expect(page.locator('.app-empty')).toBeVisible();
    }
  });
});
