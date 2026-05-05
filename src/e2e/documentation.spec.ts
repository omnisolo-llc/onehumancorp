import { test, expect } from '@playwright/test';

test.describe('Documentation Features', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Sign In"), button:has-text("Login")').click();
    await page.waitForURL('**/dashboard**');
  });

  test('should open help center from dashboard', async ({ page }) => {
    await page.locator('button:has-text("Help Center")').first().click();
    await expect(page.locator('text=/help center/i')).toBeVisible();
  });

  test('should open interactive walkthrough from dashboard', async ({ page }) => {
    await page.locator('button:has-text("App Tour")').first().click();
    await expect(page.locator('text=/step/i')).toBeVisible();
  });

  test('should open ai help chat from dashboard', async ({ page }) => {
    await page.locator('button:has-text("Ask AI")').first().click();
    await expect(page.locator('text=/how can I help/i')).toBeVisible();
  });

  test('should open video tutorials from dashboard', async ({ page }) => {
    await page.locator('button:has-text("Video Tutorials")').first().click();
    await expect(page.locator('text=/video/i')).toBeVisible();
  });

  test('should open api docs from dashboard', async ({ page }) => {
    await page.locator('button:has-text("API Docs")').first().click();
    await expect(page.locator('text=/API/i')).toBeVisible();
  });
});
