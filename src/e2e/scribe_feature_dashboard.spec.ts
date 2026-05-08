import { test, expect } from '@playwright/test';

test.describe('Scribe Feature Dashboard Navigation', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate from the home page after user login with no pre-authenticated shortcuts
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Sign In"), button:has-text("Login")').click();
    await page.waitForURL('**/dashboard**');
  });

  test('should navigate to Documentation Overview and open Help Center', async ({ page }) => {
    await page.locator('button:has-text("Menu")').first().click();
    await page.locator('button:has-text("Documentation Overview")').first().click();
    await expect(page.locator('text=/Documentation Features \\(Scribe\\)/i').first()).toBeVisible();
    await page.locator('button:has-text("Open Help Center")').first().click();
    await expect(page.locator('text=/How can we help\\?/i').first()).toBeVisible();
  });

  test('should navigate to Documentation Overview and open Interactive Walkthrough', async ({ page }) => {
    await page.locator('button:has-text("Menu")').first().click();
    await page.locator('button:has-text("Documentation Overview")').first().click();
    await page.locator('button:has-text("Start Walkthrough")').first().click();
    await expect(page.locator('text=/Set up your store/i').first()).toBeVisible();
  });

  test('should navigate to Documentation Overview and open AI Help Chat', async ({ page }) => {
    await page.locator('button:has-text("Menu")').first().click();
    await page.locator('button:has-text("Documentation Overview")').first().click();
    await page.locator('button:has-text("Open AI Chat")').first().click();
    await expect(page.locator('text=/Type your question here/i').first()).toBeVisible();
  });

  test('should navigate to Documentation Overview and open Video Tutorials', async ({ page }) => {
    await page.locator('button:has-text("Menu")').first().click();
    await page.locator('button:has-text("Documentation Overview")').first().click();
    await page.locator('button:has-text("Watch Videos")').first().click();
    await expect(page.locator('text=/Watch & Learn/i').first()).toBeVisible();
  });

  test('should navigate to Documentation Overview and open API Documentation', async ({ page }) => {
    await page.locator('button:has-text("Menu")').first().click();
    await page.locator('button:has-text("Documentation Overview")').first().click();
    await page.locator('button:has-text("View API Docs")').first().click();
    await expect(page.locator('text=/Connect Custom Software/i').first()).toBeVisible();
  });

  test('should navigate to Documentation Overview and open Release Notes', async ({ page }) => {
    await page.locator('button:has-text("Menu")').first().click();
    await page.locator('button:has-text("Documentation Overview")').first().click();
    await page.locator('button:has-text("View Release Notes")').first().click();
    await expect(page.locator('text=/What\'s New in OHC/i').first()).toBeVisible();
  });
});
