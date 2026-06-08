import { test, expect } from '@playwright/test';

test.describe('Agent Marketplace UI Integration', () => {
  test('User can search and view agents from the marketplace API', async ({ page }) => {
    // Navigate to the marketplace page
    await page.goto('/agent-marketplace');

    // Check header
    await expect(page.locator('h1')).toHaveText('Agent Marketplace');

    // Check if initial agents are loaded
    await expect(page.locator('text=Data Analyst')).toBeVisible();
    await expect(page.locator('text=SEO Specialist')).toBeVisible();

    // Perform a search
    await page.fill('input[placeholder="Search for agents..."]', 'SEO');

    // Assert that Data Analyst disappears and only SEO Specialist remains
    await expect(page.locator('text=Data Analyst')).not.toBeVisible();
    await expect(page.locator('text=SEO Specialist')).toBeVisible();

    // Perform a search for non-existent
    await page.fill('input[placeholder="Search for agents..."]', 'DoesNotExist123');

    // Assert the empty state
    await expect(page.locator('text=No agents found matching "DoesNotExist123"')).toBeVisible();

    // Clear search and test interaction
    await page.fill('input[placeholder="Search for agents..."]', '');
    await expect(page.locator('text=Data Analyst')).toBeVisible();

    // Click install button on Data Analyst
    const installButton = page.locator('text=Data Analyst').locator('..').locator('..').locator('button');
    await expect(installButton).toHaveText('Install Agent');
    await installButton.click();

    // Ensure button updates state
    await expect(installButton).toHaveText('Installed');
  });
});
