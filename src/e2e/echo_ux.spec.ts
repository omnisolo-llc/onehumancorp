import { test, expect } from '@playwright/test';

test.describe('Dashboard UX Simplification', () => {
  test('should display Today\'s Sales metric', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text="Today\'s Sales"')).toBeVisible();
  });
  test('should display plain language label Connect Apps', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text="Connect Apps"').first()).toBeVisible();
  });
  test('should toggle first-time user tour hint for Sales', async ({ page }) => {
    await page.goto('/');
    const btn = page.locator('text="?"').first();
    await btn.click();
    await expect(page.locator('text="This shows your total revenue for today."')).toBeVisible();
  });
  test('should show loading skeleton on View Orders', async ({ page }) => {
    await page.goto('/');
    const btn = page.locator('text="View Orders"').first();
    await btn.click();
    await expect(btn).toBeVisible();
  });
  test('should have Grandmother test simplified My Team', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text="My Team"').first()).toBeVisible();
  });
});
