import { test, expect } from '@playwright/test';

test.describe('Global Search / Omnibox', () => {
  test('should open via Cmd+K and search on desktop', async ({ page, isMobile }) => {
    if (isMobile) {
      test.skip();
    }

    // Navigate to dashboard
    await page.goto('/dashboard');

    // Check it's not visible initially
    await expect(page.locator('input[placeholder="Search customers, orders, messages... (Cmd+K)"]')).not.toBeVisible();

    // Trigger Cmd+K (or Ctrl+K)
    const modifier = process.platform === 'darwin' ? 'Meta' : 'Control';
    await page.keyboard.press(`${modifier}+KeyK`);

    // Omnibox should be visible
    const input = page.locator('input[placeholder="Search customers, orders, messages... (Cmd+K)"]');
    await expect(input).toBeVisible();

    // Type a search query
    await input.fill('John');

    // Wait for the debounce and fetch (mock response or real response)
    // We expect it to show some results or at least the "No results" message if none are found.
    // For now, we will wait for network request
    // Alternatively, verify the input has text
    await expect(input).toHaveValue('John');

    // Verify search endpoint was hit (optional, but good)
    const requestPromise = page.waitForRequest(request => request.url().includes('/api/v1/search') && request.method() === 'GET');
    await input.press('Space'); // Trigger a change
    await requestPromise;

    // Close using Escape
    await page.keyboard.press('Escape');
    await expect(input).not.toBeVisible();
  });

  test('should open via tap on mobile header', async ({ page, isMobile }) => {
    if (!isMobile) {
      test.skip();
    }

    // Navigate to dashboard
    await page.goto('/dashboard');

    // Check mobile search button is visible
    const searchBtn = page.locator('button[aria-label="Open search"]');
    await expect(searchBtn).toBeVisible();

    // Tap to open
    await searchBtn.click();

    // Omnibox should be visible
    const input = page.locator('input[placeholder="Search customers, orders, messages... (Cmd+K)"]');
    await expect(input).toBeVisible();

    // Fill search
    await input.fill('Mobile Search');
    await expect(input).toHaveValue('Mobile Search');
  });
});
