import { test, expect } from './fixtures';

test.describe('My Plan Page', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/plan');
  });

  test('should display Plan status and Estimated Next Bill', async ({ page }) => {
    await expect(page.locator('h1', { hasText: 'My Plan' })).toBeVisible({ timeout: 10000 });
    await expect(page.locator('#my-plan-name')).toBeVisible();
    await expect(page.locator('#my-plan-next-bill')).toBeVisible();
  });

  test('should display usage section with AI Actions and Storage', async ({ page }) => {
    await expect(page.locator('h2', { hasText: 'Your Current Usage' })).toBeVisible();
    await expect(page.locator('span', { hasText: 'AI Actions Used' })).toBeVisible();
    await expect(page.locator('span', { hasText: 'Storage Used' })).toBeVisible();
  });

  test('should display management actions', async ({ page }) => {
    await expect(page.locator('h3', { hasText: 'View Cost Details' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Change Plan' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Download Invoice' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Cancel Subscription' })).toBeVisible();
  });

  test('should navigate to cost dashboard when clicking View Cost Details', async ({ page }) => {
    const viewCostDetailsBtn = page.locator('button', { hasText: 'View Cost Details' });
    await expect(viewCostDetailsBtn).toBeVisible();
    await viewCostDetailsBtn.click();
    await expect(page.url()).toContain('/cost-dashboard');
  });

  test('should navigate to pricing when clicking Change Plan', async ({ page }) => {
    const changePlanBtn = page.locator('button', { hasText: 'Change Plan' });
    await expect(changePlanBtn).toBeVisible();
    await changePlanBtn.click();
    await expect(page.url()).toContain('/pricing');
  });

  test('should display message when clicking Download Invoice', async ({ page }) => {
    const downloadBtn = page.locator('button', { hasText: 'Download Invoice' });
    await expect(downloadBtn).toBeVisible();
    await downloadBtn.click();
    await expect(page.locator('[role="status"]')).toContainText('Invoice download is ready');
  });

  test('should display success message when clicking Cancel Subscription', async ({ page }) => {
    const cancelBtn = page.locator('button', { hasText: 'Cancel Subscription' });
    await expect(cancelBtn).toBeVisible();
    await cancelBtn.click();
    await expect(page.locator('[role="status"]')).toContainText('Subscription canceled successfully');
  });
});
