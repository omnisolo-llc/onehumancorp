import { test, expect } from '@playwright/test';

test.describe('Work Triage Agentic Inbox', () => {
  const tenantId = 'e2e-tenant';

  test.beforeEach(async ({ page }) => {
    // Log in with tenant
    await page.goto('/login');
    await page.fill('input[type="text"]', tenantId);
    await page.click('button[type="submit"]');

    // Go to Triage via dashboard
    await page.goto('/dashboard');

    // Wait for the triage queue to load
    await expect(page.locator('h2').filter({ hasText: 'Needs Your Attention' })).toBeVisible();
  });

  test('Owner reviews and approves a triage item', async ({ page }) => {
    const triageCard = page.locator('[data-testid="triage-card-triage-test-1"]');
    await expect(triageCard).toBeVisible();
    await expect(triageCard.locator('text=Maya requested a custom cake')).toBeVisible();

    const approveBtn = triageCard.locator('[data-testid="approve-btn"]');
    await approveBtn.click();
    await expect(triageCard).not.toBeVisible();
  });

  test('Owner dismisses a triage item', async ({ page }) => {
    const triageCard = page.locator('[data-testid="triage-card-triage-test-2"]');
    await expect(triageCard).toBeVisible();
    await expect(triageCard.locator('text=Question about delivery times')).toBeVisible();

    const dismissBtn = triageCard.locator('button', { hasText: 'Dismiss' });
    await dismissBtn.click();
    await expect(triageCard).not.toBeVisible();
  });

  test('Triage correctly displays priority badges', async ({ page }) => {
    const triageCardUrgent = page.locator('[data-testid="triage-card-triage-test-4"]');
    await expect(triageCardUrgent.locator('.triage-priority.urgent')).toBeVisible();
    await expect(triageCardUrgent.locator('.triage-priority')).toHaveText('Urgent');

    const triageCardMedium = page.locator('[data-testid="triage-card-triage-test-5"]');
    await expect(triageCardMedium.locator('.triage-priority.medium')).toBeVisible();
  });

  test('Triage correctly displays action payloads', async ({ page }) => {
    const triageCard = page.locator('[data-testid="triage-card-triage-test-3"]');
    await expect(triageCard.locator('.triage-action-title')).toHaveText('Draft Reply');
    await expect(triageCard.locator('text=Thank you for the feedback!')).toBeVisible();
  });

  test('Triage controls are responsive on 375px viewport', async ({ page }) => {
    const triageCard = page.locator('[data-testid="triage-card-triage-test-5"]');

    // Change viewport to 375px
    await page.setViewportSize({ width: 375, height: 667 });

    const controls = triageCard.locator('.triage-controls');
    await expect(controls).toBeVisible();

    // Verify CSS computed style flex-direction is column
    const flexDirection = await controls.evaluate((el) => {
      return window.getComputedStyle(el).flexDirection;
    });

    expect(flexDirection).toBe('column');
  });
});
