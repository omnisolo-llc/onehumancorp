import { test, expect } from '@playwright/test';

test.describe('Work Triage Agentic Inbox', () => {
  const tenantId = 'test-tenant';

  test('Owner reviews and approves a triage item via conversational modal', async ({ page }) => {
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

    // Verify draft view is populated
    await expect(triageCard.locator('text=Draft Reply')).toBeVisible();

    // Click Review Draft Reply
    const reviewDraftBtn = triageCard.locator('[data-testid="review-draft-btn"]');
    await reviewDraftBtn.click();

    // Modal should appear
    const modal = page.locator('.triage-modal-sheet');
    await expect(modal).toBeVisible();
    await expect(modal.locator('text=In response to: Maya requested a custom cake for Friday')).toBeVisible();

    // Approve action
    const approveBtn = modal.locator('[data-testid="approve-send-btn"]');
    await approveBtn.click();

    // Should show approved status and disappear from list
    await expect(triageCard).not.toBeVisible();
    await expect(modal).not.toBeVisible();
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

  test('Triage modal allows text adjustment', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="text"]', tenantId);
    await page.click('button[type="submit"]');

    await page.goto('/dashboard');

    const triageCard = page.locator('[data-testid="triage-card-triage-test-2"]');
    await expect(triageCard).toBeVisible({ timeout: 15000 });

    const reviewDraftBtn = triageCard.locator('[data-testid="review-draft-btn"]');
    await reviewDraftBtn.click();

    const modal = page.locator('.triage-modal-sheet');
    await expect(modal).toBeVisible();

    const textarea = modal.locator('textarea');
    await expect(textarea).toBeVisible();
    await textarea.fill('Adjusted response message');
    await expect(textarea).toHaveValue('Adjusted response message');

    const cancelBtn = modal.locator('text=Cancel');
    await cancelBtn.click();
    await expect(modal).not.toBeVisible();
  });
});
