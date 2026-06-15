import { test, expect } from '@playwright/test';

test.describe('Work Triage Agentic Inbox', () => {
  const tenantId = 'e2e-tenant';

  test.beforeEach(async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="email"]', 'admin@ohc.local');
    await page.fill('input[type="password"]', 'changeme');
    await page.click('button[type="submit"]');
    await page.goto('/dashboard');
    await expect(page.locator('h2').filter({ hasText: 'Unified Agent Feed' })).toBeVisible({ timeout: 15000 });
  });

  test('Owner reviews and approves a triage item', async ({ page }) => {
    const triageCard = page.locator('[data-testid="triage-card-triage-test-1"]');
    await expect(triageCard).toBeVisible({ timeout: 10000 });

    // Verify detail view is populated from selected card (first item by default)
    await expect(page.locator('text=Maya requested a custom cake for Friday')).toBeVisible();
    await expect(page.locator('text=Hi Maya! I can definitely help with the custom cake. It will be $50.')).toBeVisible();

    // Approve action
    const approveBtn = page.locator('[data-testid="approve-btn"]');
    await approveBtn.click();

    // Should show approved status and disappear from list
    await expect(triageCard).not.toBeVisible();
  });

  test('Owner can dismiss a triage item', async ({ page }) => {
    const triageCard = page.locator('[data-testid="triage-card-triage-test-2"]');
    await expect(triageCard).toBeVisible({ timeout: 15000 });

    // Select the card first to ensure detail view updates
    await triageCard.click();

    const dismissBtn = page.locator('[data-testid="dismiss-btn"]');
    await dismissBtn.click();

    await expect(triageCard).not.toBeVisible();
  });

  test('Triage feed renders items correctly', async ({ page }) => {
    const triageCard = page.locator('[data-testid="triage-card-triage-test-1"]');
    await expect(triageCard).toBeVisible({ timeout: 15000 });
    await expect(triageCard.locator('text=Instagram DM')).toBeVisible();
    await expect(triageCard.locator('text=Maya requested a custom cake for Friday')).toBeVisible();
  });

  test('Triage detail shows correct information on click', async ({ page }) => {
    const triageCard = page.locator('[data-testid="triage-card-triage-test-2"]');
    await expect(triageCard).toBeVisible({ timeout: 15000 });
    await triageCard.click();

    await expect(page.locator('text=Question about delivery times')).toBeVisible();
    await expect(page.locator('text=We deliver between 9 AM and 5 PM on weekdays.')).toBeVisible();
  });

  test('Layout is fully usable at 375px', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });

    // Have to reload since viewport change might act weird mid-flight in some setups, but we just check visibility
    const triageCard = page.locator('[data-testid="triage-card-triage-test-1"]');
    await expect(triageCard).toBeVisible({ timeout: 15000 });

    // Ensure detail view can be scrolled into view or is stacked correctly
    await triageCard.click();
    await expect(page.locator('text=Maya requested a custom cake for Friday')).toBeVisible();
    await expect(page.locator('[data-testid="approve-btn"]')).toBeVisible();
  });
});
