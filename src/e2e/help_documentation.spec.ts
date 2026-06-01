import { test, expect } from './fixtures';

test.describe('Help & Documentation UI', () => {
  test('Help Center page displays correctly', async ({ page }) => {
    await page.goto('/help');
    await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible();
    await expect(page.locator('text=Getting Started')).toBeVisible();
    await expect(page.locator('input[placeholder="Search for help articles..."]')).toBeVisible();
  });

  test('Help Article pages display correctly', async ({ page }) => {
    await page.goto('/help/getting-started');
    await expect(page.getByRole('heading', { name: 'Getting Started with Your Store' })).toBeVisible();
    await expect(page.locator('text=Welcome to OneHumanCorp!')).toBeVisible();
  });

  test('Changelog page displays correctly', async ({ page }) => {
    await page.goto('/changelog');
    await expect(page.getByRole('heading', { name: 'Release Notes & Changelog' })).toBeVisible();
    await expect(page.locator('text=Version 1.0 (Latest)')).toBeVisible();
  });

  test('API Docs display correctly', async ({ page }) => {
    await page.goto('/api-docs');
    await expect(page.locator('text=Advanced:')).toBeVisible();
    await expect(page.locator('.swagger-ui')).toBeVisible();
  });

  test('Help Chat opens when clicked', async ({ page }) => {
    await page.goto('/');
    // Check if the chat button is present. Since E2E mock was removed, it should appear.
    const chatButton = page.locator('button[aria-label="Open help chat"]');
    await expect(chatButton).toBeVisible();
    await chatButton.click();

    // Verify chat UI opens
    await expect(page.locator('h3:has-text("Help Agent")')).toBeVisible();
    await expect(page.locator('text=Always here to help')).toBeVisible();
    await expect(page.locator('text=Hi! I\'m your AI Help Agent.')).toBeVisible();

    // Close chat
    const closeBtn = page.locator('button[aria-label="Close help chat"]');
    await expect(closeBtn).toBeVisible();
    await closeBtn.click();
  });
});
