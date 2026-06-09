import { test, expect } from '@playwright/test';

test.describe('Agent Marketplace E2E', () => {
  test('User can search and view agents in the marketplace', async ({ page }) => {
    // Navigate to the marketplace page
    await page.goto('/agent-marketplace');

    // Wait for the page to load
    await expect(page.locator('h1')).toHaveText('Agent Marketplace');

    // Check if the search input is visible
    const searchInput = page.getByPlaceholder('Search for agents...');
    await expect(searchInput).toBeVisible();

    // Verify initial agents are loaded
    await expect(page.locator('h3').first()).toBeVisible();
    await expect(page.locator('text=Senior Rust Developer')).toBeVisible();

    // Search for a specific agent
    await searchInput.fill('Senior');

    // Wait for the search results to filter
    await expect(page.locator('text=Senior Rust Developer')).toBeVisible();
    await expect(page.locator('text=SEO Optimizer')).not.toBeVisible();

    // Clear search and verify original agents are shown
    await searchInput.fill('');
    await expect(page.locator('text=Senior Rust Developer')).toBeVisible();

    // Search for an agent that doesn't exist
    await searchInput.fill('NonexistentAgent123');
    await expect(page.locator('text=No agents found matching "NonexistentAgent123"')).toBeVisible();

    // Click install agent on one of them (when search is cleared)
    await searchInput.fill('Senior Rust Developer');
    const installButton = page.getByRole('button', { name: 'Install Agent' }).first();

    // Set up a dialog handler before clicking
    page.once('dialog', async dialog => {
      expect(dialog.message()).toContain('Successfully installed Senior Rust Developer!');
      await dialog.accept();
    });

    await installButton.click();
  });
});
