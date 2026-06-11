import { test, expect } from '@playwright/test';

test.describe('Work Triage Agentic Inbox', () => {
  const tenantId = 'test-tenant';

  test('Owner reviews and approves a triage item', async ({ page }) => {
    // Log in with tenant
    await page.goto('/login');
    await page.fill('input[type="text"]', tenantId);
    await page.click('button[type="submit"]');

    // Go to Triage
    await page.goto('/dashboard');

    // Wait for the triage queue to load
    await expect(page.locator('h2').filter({ hasText: 'Unified Agent Feed' })).toBeVisible({ timeout: 15000 });

    const triageCard = page.locator('[data-testid="triage-card-triage-test-1"]');

    // Auto-wait for the card to appear (data should be seeded)
    await expect(triageCard).toBeVisible({ timeout: 10000 });

    // Verify detail view is populated from selected card
    await expect(page.locator('text=Maya requested a custom cake')).toBeVisible();
    await expect(page.locator('text=Draft Reply')).toBeVisible();

    // Approve action
    const approveBtn = page.locator('[data-testid="approve-btn"]');
    await approveBtn.click();

    // Should show approved status and disappear from list
    await expect(triageCard).not.toBeVisible();
  });

  test('Owner sees empty state when there are no items', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="text"]', 'empty-tenant-triage-test');
    await page.click('button[type="submit"]');

    await page.goto('/dashboard');

    // It should not render the needs attention section if there are no items
    // Since it's loading initially, we wait for a known dashboard element instead
    await expect(page.locator('text=Business Analytics')).toBeVisible({ timeout: 15000 });

    // Wait a bit to ensure it finished loading, then assert not visible
    await page.waitForTimeout(2000);
    await expect(page.locator('h2').filter({ hasText: 'Unified Agent Feed' })).not.toBeVisible();
  });

  test('Owner can dismiss a triage item', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="text"]', tenantId);
    await page.click('button[type="submit"]');

    await page.goto('/dashboard');

    const triageCard = page.locator('[data-testid="triage-card-triage-test-2"]');

    await expect(triageCard).toBeVisible({ timeout: 15000 });

    await triageCard.click();
    const dismissBtn = page.locator('[data-testid="dismiss-btn"]');
    await dismissBtn.click();

    await expect(triageCard).not.toBeVisible();
  });

  test('Triage feed renders items correctly', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="text"]', tenantId);
    await page.click('button[type="submit"]');

    await page.goto('/dashboard');

    const triageCard = page.locator('[data-testid="triage-card-triage-test-1"]');
    await expect(triageCard).toBeVisible({ timeout: 15000 });
    await expect(triageCard.locator('text=Instagram DM')).toBeVisible();
    await expect(triageCard.locator('text=Urgent')).toBeVisible();
  });

  test('Triage detail shows correct information on click', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="text"]', tenantId);
    await page.click('button[type="submit"]');

    await page.goto('/dashboard');

    const triageCard = page.locator('[data-testid="triage-card-triage-test-2"]');
    await expect(triageCard).toBeVisible({ timeout: 15000 });

    await triageCard.click();
    await expect(page.locator('text=WhatsApp')).toBeVisible();
    await expect(page.locator('text=Question about delivery times')).toBeVisible();
  });

  test('Backend action executes correctly on Approve Draft', async ({ page }) => {
    // This test verifies that the backend side effect (inserting into omni_inbox_messages) works.
    await page.goto('/login');
    await page.fill('input[type="text"]', 'e2e-tenant');
    await page.click('button[type="submit"]');

    await page.goto('/dashboard');
    const triageCard = page.locator('[data-testid="triage-card-triage-test-1"]');

    // Since previous test might have consumed it, let's just make sure we click approve if visible
    if (await triageCard.isVisible()) {
      const approveBtn = page.locator('[data-testid="approve-btn"]').first();
      await approveBtn.click();
      await expect(triageCard).not.toBeVisible({ timeout: 15000 });
    }

    // Wait for the action to be processed.
    await page.waitForTimeout(2000);

    // Actually, getting to the inbox messages API requires auth. We can verify it via UI if there's an inbox page.
    await page.goto('/inbox');
    await expect(page.locator('text=Hi Maya! I can definitely help with the custom cake. It will be $50.')).toBeVisible({ timeout: 15000 });
  });

  test('Layout is fully usable at 375px', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/login');
    await page.fill('input[type="text"]', tenantId);
    await page.click('button[type="submit"]');

    await page.goto('/dashboard');
    const triageCard = page.locator('[data-testid="triage-card-triage-test-1"]');
    await expect(triageCard).toBeVisible({ timeout: 15000 });

    // Ensure detail view can be scrolled into view or is stacked correctly
    await triageCard.click();
    await expect(page.locator('text=Draft Reply')).toBeVisible();
    await expect(page.locator('[data-testid="approve-btn"]')).toBeVisible();
  });
});
