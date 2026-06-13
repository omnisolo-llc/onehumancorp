import { test, expect } from '@playwright/test';

test.describe('Work Triage Agentic Inbox', () => {
  const tenantId = 'e2e-tenant';

  test.beforeEach(async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="email"]', 'admin@ohc.local');
    await page.fill('input[type="password"]', 'changeme');
    await page.click('button[type="submit"]');
    await page.goto('/dashboard');
    await expect(page.locator('h2').filter({ hasText: 'Welcome back' }).first()).toBeVisible({ timeout: 15000 });
  });

  test('Owner reviews and approves a triage item', async ({ page }) => {
    const triageCard = page.locator('[data-testid="triage-card-app-test-ab12-34f7-e43e-7264a9c4021d"]');
    await expect(triageCard).toBeVisible({ timeout: 10000 });

    // Verify detail view is populated from selected card (first item by default)
    await expect(page.locator('text=Mark requested to reschedule his 4 PM lesson to 5 PM today. You have a conflict. Suggest tomorrow at 4 PM?')).toBeVisible();
    await expect(page.locator('text=Mark requested to reschedule his 4 PM lesson to 5 PM today. You have a conflict. Suggest tomorrow at 4 PM?')).toBeVisible();

    // Approve action
    const approveBtn = page.locator('[data-testid="approve-btn"]');
    await approveBtn.click();

    // Should show approved status and disappear from list
    await expect(triageCard).not.toBeVisible();
  });

  test('Owner can dismiss a triage item', async ({ page }) => {
    const triageCard = page.locator('[data-testid="triage-card-app-test-cd34-34f7-e43e-7264a9c4021d"]');
    await expect(triageCard).toBeVisible({ timeout: 15000 });

    // Select the card first to ensure detail view updates
    await triageCard.click();

    const dismissBtn = page.locator('[data-testid="dismiss-btn"]');
    await dismissBtn.click();

    await expect(triageCard).not.toBeVisible();
  });

  test('Triage feed renders items correctly', async ({ page }) => {
    const triageCard = page.locator('[data-testid="triage-card-app-test-ab12-34f7-e43e-7264a9c4021d"]');
    await expect(triageCard).toBeVisible({ timeout: 15000 });
    await expect(triageCard.locator('text=Operations')).toBeVisible();
    await expect(triageCard.locator('text=Mark requested to reschedule his 4 PM lesson to 5 PM today. You have a conflict. Suggest tomorrow at 4 PM?')).toBeVisible();
  });

  test('Triage detail shows correct information on click', async ({ page }) => {
    const triageCard = page.locator('[data-testid="triage-card-app-test-cd34-34f7-e43e-7264a9c4021d"]');
    await expect(triageCard).toBeVisible({ timeout: 15000 });
    await triageCard.click();

    await expect(page.locator('text=Agent tentatively booked a roof repair estimate for Sarah on Tuesday 2 PM. Pending $50 deposit. No action needed.')).toBeVisible();
    await expect(page.locator('text=Agent tentatively booked a roof repair estimate for Sarah on Tuesday 2 PM. Pending $50 deposit. No action needed.')).toBeVisible();
  });

  test('Layout is fully usable at 375px', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });

    // Have to reload since viewport change might act weird mid-flight in some setups, but we just check visibility
    const triageCard = page.locator('[data-testid="triage-card-app-test-ab12-34f7-e43e-7264a9c4021d"]');
    await expect(triageCard).toBeVisible({ timeout: 15000 });

    // Ensure detail view can be scrolled into view or is stacked correctly
    await triageCard.click();
    await expect(page.locator('text=Mark requested to reschedule his 4 PM lesson to 5 PM today. You have a conflict. Suggest tomorrow at 4 PM?')).toBeVisible();
    await expect(page.locator('[data-testid="approve-btn"]')).toBeVisible();
  });
});
