import { test, expect } from '@playwright/test';

test.describe('Omnibox Global Search', () => {
    test.beforeEach(async ({ page }) => {
        // Assume login is handled via state or mock, or navigate to an authenticated route like dashboard
        await page.goto('/dashboard');
        await page.waitForLoadState('networkidle');
    });

    test('should open Omnibox on Cmd+K or Ctrl+K and perform search', async ({ page }) => {
        // Press Cmd+K to open
        await page.keyboard.press('ControlOrMeta+K');

        // Wait for the Omnibox input to appear
        const searchInput = page.getByPlaceholder('Search customers, orders, messages, or type a command...');
        await expect(searchInput).toBeVisible();

        // Type a query
        await searchInput.fill('John');

        // It should eventually fetch results (either mock or rely on the real backend depending on the setup)
        // Here we just verify the state changes
        await expect(page.locator('text=Searching...').or(page.locator('ul.space-y-1'))).toBeVisible();

        // Close with Escape
        await page.keyboard.press('Escape');
        await expect(searchInput).toBeHidden();
    });
});
