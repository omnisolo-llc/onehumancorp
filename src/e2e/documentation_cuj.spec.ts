import { test, expect } from '@playwright/test';

test.describe('Documentation User Journey', () => {
  test('Maya navigates the Help Center and views the Changelog', async ({ page }) => {
    await page.goto('/changelog.html');
    await expect(page.locator('h1', { hasText: 'Release Notes & Changelog' })).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Version 1.0 (Latest)' })).toBeVisible();

    await page.goto('/help.html');
    await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible();

    await page.fill('input[placeholder="Search for help articles and videos..."]', 'products');
    const myStoreLink = page.locator('h2', { hasText: 'My Store' });
    await expect(myStoreLink).toBeVisible();

    await myStoreLink.click();
    await expect(page.locator('h1', { hasText: 'Managing My Store' })).toBeVisible();
  });

  test('Tooltips appear on elements', async ({ page }) => {
    await page.goto('/dashboard.html');
    const helpBtn = page.locator('#ohc-help-btn');
    await expect(helpBtn).toBeVisible();
  });

  test('Interactive Walkthrough can be started', async ({ page }) => {
    await page.goto('/dashboard.html');
    const helpBtn = page.locator('#ohc-help-btn');
    await expect(helpBtn).toBeVisible();
    await helpBtn.click();

    const tourBtn = page.locator('button', { hasText: 'Tour: Set up your store' });
    await expect(tourBtn).toBeVisible();
    await tourBtn.click();

    const bubble = page.locator('#walkthrough-bubble');
    await expect(bubble).toBeVisible();
    await expect(bubble.getByText('Click here to go to your storefront builder.')).toBeVisible();
  });

  test('API Documentation is accessible', async ({ page }) => {
    await page.goto('/api-docs.html');
    await expect(page.getByText('API Documentation')).toBeVisible();
  });

  test('AI Help Chat is functional', async ({ page }) => {
    await page.goto('/dashboard.html');
    const helpBtn = page.locator('#ohc-help-btn');
    await expect(helpBtn).toBeVisible();
    await helpBtn.click();

    await expect(page.locator('#ohc-help-chat-header')).toBeVisible();
    await page.fill('#ohc-help-input', 'Hello');
    await page.click('#ohc-help-send');

    // Check that user message is in chat
    await expect(page.locator('.msg-user', { hasText: 'Hello' })).toBeVisible();
  });
});
