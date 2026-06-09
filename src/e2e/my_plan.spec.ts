import { test, expect } from './fixtures';

test.describe('My Plan & Cost Dashboard E2E', () => {

  test('should display current plan and upgrade button', async ({ page }) => {
    await page.goto('/plan');
    await expect(page.locator('#my-plan-name')).toBeVisible();
    await expect(page.locator('button', { hasText: 'Upgrade Plan' })).toBeVisible();
  });

  test('should show AI actions usage with progress bar', async ({ page }) => {
    await page.goto('/plan');
    await expect(page.locator('span', { hasText: 'AI Actions Used' })).toBeVisible();
    // Verify progress bar exists
    await expect(page.locator('.bg-blue-600.h-2.5')).toBeVisible();
  });

  test('should display cost dashboard total savings from compression', async ({ page }) => {
    await page.goto('/cost-dashboard');
    // In the new layout, this is "Economic Gains" or "Optimization Saved"
    await expect(page.locator('h3', { hasText: 'Economic Gains' })).toBeVisible();
    await expect(page.locator('span', { hasText: 'Optimization Saved' })).toBeVisible();
  });

  test('should show user feedback message after downloading invoice', async ({ page }) => {
    await page.goto('/plan');
    await page.click('button:has-text("Download Invoice")');
    await expect(page.locator('[role="status"]')).toContainText('Invoice download is ready');
  });

  test('should show confirmation and message after cancelling subscription', async ({ page }) => {
    await page.goto('/plan');

    // Setup dialog handler before clicking
    page.once('dialog', async dialog => {
      expect(dialog.message()).toContain('Are you sure you want to cancel');
      await dialog.accept();
    });

    await page.click('button:has-text("Cancel Subscription")');
    // The message is displayed in the role="status" div
    await expect(page.locator('[role="status"]')).toContainText('Subscription canceled successfully');
  });

});
