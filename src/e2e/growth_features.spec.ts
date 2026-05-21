import { test, expect } from '@playwright/test';

test.describe('OHC Growth Features', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');
  });

  test('should display weekly insights and allow dismissal', async ({ page }) => {
    const insightCard = page.locator('text=Weekly Insights').locator('..').locator('..');
    await expect(insightCard).toBeVisible();
    await expect(page.locator('text=Sales are slow this week')).toBeVisible();

    // Click action button
    await page.click('text=Yes, Do It');

    // Should be dismissed from UI
    await expect(page.locator('text=Sales are slow this week')).not.toBeVisible();
  });

  test('should display order drafts and allow approval', async ({ page }) => {
    await expect(page.locator('text=New WhatsApp Order Draft')).toBeVisible();
    await expect(page.locator('text=Approve $40.00')).toBeVisible();

    await page.click('text=Approve $40.00');
    await expect(page.locator('text=New WhatsApp Order Draft')).not.toBeVisible();
  });

  test('should navigate to unified catalog', async ({ page }) => {
    await page.click('text=Manage Catalog');
    await expect(page).toHaveURL(/\/catalog/);
    await expect(page.locator('h1:has-text("Catalog")')).toBeVisible();
  });

  test('should allow adding a new physical product to catalog', async ({ page }) => {
    await page.goto('/catalog');
    await page.click('text=Add New');

    await page.fill('placeholder="Product Name"', 'Sourdough Bread');
    await page.fill('placeholder="0.00"', '12.50');
    await page.fill('textarea', 'Freshly baked every morning.');

    await page.click('button:has-text("Save Item")');

    await expect(page.locator('text=Sourdough Bread')).toBeVisible();
    await expect(page.locator('text=$12.50')).toBeVisible();
  });

  test('should allow adding a new service to catalog', async ({ page }) => {
    await page.goto('/catalog');
    await page.click('text=Add New');

    await page.click('text=Service');
    await page.fill('placeholder="Service Name"', 'Baking Lesson');
    await page.fill('placeholder="0.00"', '50.00');

    await page.click('button:has-text("Save Item")');

    await expect(page.locator('text=Baking Lesson')).toBeVisible();
    await expect(page.locator('text=Service')).toBeVisible();
  });
});
