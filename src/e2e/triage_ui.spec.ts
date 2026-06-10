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
    await expect(page.locator('h2').filter({ hasText: 'Needs Your Attention' })).toBeVisible({ timeout: 15000 });

    const triageCard = page.locator('[data-testid="triage-card-triage-test-1"]');

    // Auto-wait for the card to appear (data should be seeded)
    await expect(triageCard).toBeVisible({ timeout: 10000 });

    // Verify detail view is populated directly on the card
    await expect(triageCard.locator('text=Maya requested a custom cake')).toBeVisible();
    await expect(triageCard.locator('text=Draft Reply')).toBeVisible();

    // Approve action directly on the card
    const approveBtn = triageCard.locator('[data-testid="approve-btn"]');
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
    await expect(page.locator('h2').filter({ hasText: 'Needs Your Attention' })).not.toBeVisible();
  });

  test('Owner can dismiss a triage item', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="text"]', tenantId);
    await page.click('button[type="submit"]');

    await page.goto('/dashboard');

    const triageCard = page.locator('[data-testid="triage-card-triage-test-2"]');

    await expect(triageCard).toBeVisible({ timeout: 15000 });

    // Dismiss action directly on the card
    const dismissBtn = triageCard.locator('[data-testid="dismiss-btn"]');
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

  test('Triage detail shows correct information', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="text"]', tenantId);
    await page.click('button[type="submit"]');

    await page.goto('/dashboard');

    const triageCard = page.locator('[data-testid="triage-card-triage-test-2"]');
    await expect(triageCard).toBeVisible({ timeout: 15000 });

    // Info is now directly on the card without clicking
    await expect(triageCard.locator('text=WhatsApp')).toBeVisible();
    await expect(triageCard.locator('text=Question about delivery times')).toBeVisible();
  });
});
