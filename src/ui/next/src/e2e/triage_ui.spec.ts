import { test, expect } from '@playwright/test';

test.describe('Work Triage Agentic Inbox on Dashboard', () => {
  const tenantId = 'e2e-tenant';

  test.beforeEach(async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="email"]', 'admin@ohc.local');
    await page.fill('input[type="password"]', 'changeme');
    await page.click('button[type="submit"]');
    await page.goto('/dashboard');
    // Ensure the Proposals tab is visible, which holds the Work Triage Feed
    await expect(page.locator('button').filter({ hasText: 'Proposals' }).first()).toBeVisible({ timeout: 15000 });
  });

  test('Work Triage Feed renders correctly', async ({ page }) => {
    const triageFeed = page.locator('[data-testid="work-triage-feed"]');
    await expect(triageFeed).toBeVisible({ timeout: 15000 });
  });

  test('Owner reviews and approves a triage item', async ({ page }) => {
    const triageCard = page.locator('[data-testid="triage-card-triage-test-1"]');
    await expect(triageCard).toBeVisible({ timeout: 15000 });

    await expect(triageCard.locator('text=Maya requested a custom cake for Friday')).toBeVisible();

    const approveBtn = page.locator('[data-testid="triage-approve-triage-test-1"]');
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // Optimistic UI update should remove the item
    await expect(triageCard).not.toBeVisible();
  });

  test('Owner can dismiss a triage item', async ({ page }) => {
    const triageCard = page.locator('[data-testid="triage-card-triage-test-2"]');
    await expect(triageCard).toBeVisible({ timeout: 15000 });

    const dismissBtn = page.locator('[data-testid="triage-dismiss-triage-test-2"]');
    await dismissBtn.click();

    await expect(triageCard).not.toBeVisible();
  });

  test('Layout is fully usable at 375px', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });

    const triageCard = page.locator('[data-testid="triage-card-triage-test-1"]');
    await expect(triageCard).toBeVisible({ timeout: 15000 });
    await expect(page.locator('[data-testid="triage-approve-triage-test-1"]')).toBeVisible();

    const approveBtn = page.locator('[data-testid="triage-approve-triage-test-1"]');
    const boundingBox = await approveBtn.boundingBox();
    expect(boundingBox!.height).toBeGreaterThanOrEqual(44);
  });

  test('Proactive Context Agent item handles approval', async ({ page }) => {
    const triageCard = page.locator('[data-testid="triage-card-triage-test-db"]');
    await expect(triageCard).toBeVisible({ timeout: 15000 });

    await expect(triageCard.locator('text=Decision Assistant')).toBeVisible();

    const approveBtn = page.locator('[data-testid="triage-approve-triage-test-db"]');
    await approveBtn.click();

    await expect(triageCard).not.toBeVisible();
  });
});
