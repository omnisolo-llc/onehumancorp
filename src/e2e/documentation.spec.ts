import { test, expect } from '@playwright/test';

test.describe('Documentation Feature E2E', () => {
  test('navigates to help center and searches articles', async ({ page }) => {
    await page.goto('/dashboard');
    await page.waitForSelector('button:has-text("?")', { state: 'visible' });
    await page.click('button:has-text("?")');
    await expect(page.locator('h3', { hasText: 'Suggested Articles' })).toBeVisible();
    await page.fill('input[placeholder="Search help articles..."]', 'Agent');
    await expect(page.locator('h4', { hasText: 'Your AI Helpers' })).toBeVisible();
  });

  test('help chat widget can be opened and interacts with AI', async ({ page }) => {
    await page.goto('/dashboard');
    await page.waitForSelector('button:has-text("?")', { state: 'visible' });
    await page.click('button:has-text("?")');
    await page.click('button:has-text("Ask AI")');
    await expect(page.locator('text=Hi! I\'m your AI Support Agent.')).toBeVisible();
  });

  test('verifies tooltips appear', async ({ page }) => {
    await page.goto('/dashboard');
    await page.locator('h1', { hasText: 'Dashboard' }).hover();
    await expect(page.locator('div', { hasText: 'Check your sales, recent orders, and how your store is doing.' })).toBeVisible();
    await page.locator('a', { hasText: 'AI Departments' }).hover();
    await expect(page.locator('div', { hasText: 'See your AI team, give them tasks, or hire new helpers.' })).toBeVisible();
  });
});
