import { test, expect } from '@playwright/test';

test.describe('Autonomous Operations CUJ', () => {
  test.beforeEach(async ({ page }) => {
    // 1. Start from the home page after user login
    await page.goto('/');
    // Assuming the app has a login or auto-login for E2E
    await expect(page).toHaveTitle(/OHC/);
  });

  test('should handle multiple pending approvals from different departments', async ({ page }) => {
    await expect(page.locator('text=The Manager')).toBeVisible();
    await expect(page.locator('text=The Ambassador')).toBeVisible();
    await expect(page.locator('text=Restock Milk')).toBeVisible();
    await expect(page.locator('text=Draft Reply')).toBeVisible();

    // Approve CS task
    await page.locator('div:has-text("Draft Reply")').locator('button:has-text("Approve & Send")').click();
    await expect(page.locator('text=E2E Test Message')).not.toBeVisible();
    await expect(page.locator('text=Restock Milk')).toBeVisible();
  });

  test('should show empty state message when no approvals are pending', async ({ page }) => {
    await page.locator('div:has-text("Draft Reply")').locator('button:has-text("Approve & Send")').click();
    await page.locator('div:has-text("Restock Milk")').locator('button:has-text("Approve & Send")').click();

    await expect(page.locator('text=Needs Your Approval')).not.toBeVisible();
  });
});
