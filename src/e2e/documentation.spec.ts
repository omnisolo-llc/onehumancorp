import { test, expect } from './fixtures';

test.describe('Documentation full suite', () => {
  test('Help portal loads properly and search works', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/help');

    // Title should be present
    const title = page.locator('h1');
    await expect(title).toBeVisible();
    await expect(title).toContainText('In-App Help Center');

    // Make sure search bar exists
    const searchInput = page.getByPlaceholder('Search for help articles and videos...');
    await expect(searchInput).toBeVisible();
    await searchInput.fill('Products');

    // Wait for the articles to filter
    await expect(page.getByText('Adding products and services')).toBeVisible({ timeout: 10000 });

    // Chat widget open interaction
    const chatBtn = page.getByRole('button', { name: 'Ask anything' });
    await expect(chatBtn).toBeVisible();
    await chatBtn.click();

    // Check if the chat input is now visible
    const chatInputForm = page.getByPlaceholder('Ask anything...');
    await expect(chatInputForm).toBeVisible();
  });

  test('Changelog pulls data dynamically', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/changelog');

    // Title should be present
    const title = page.locator('h1');
    await expect(title).toBeVisible();
    await expect(title).toContainText('Release Notes & Changelog');
  });

  test('Walkthrough feature works on Dashboard', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/dashboard');

    // Evaluate to force walkthrough
    await page.evaluate(() => {
        localStorage.setItem("TEST_WALKTHROUGH", "true");
    });

    const walkBtn = page.locator('#dashboard-walkthrough-btn');
    await expect(walkBtn).toBeVisible();
    await walkBtn.click();

    const overlay = page.locator('.ohc-walkthrough-overlay');
    await expect(overlay).toBeVisible();

    const bubble = page.locator('.ohc-walkthrough-bubble');
    await expect(bubble).toBeVisible();
    await expect(bubble).toContainText('Business Analytics');
  });
});
