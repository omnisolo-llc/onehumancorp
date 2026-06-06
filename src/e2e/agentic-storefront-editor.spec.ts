import { test, expect } from '@playwright/test';

test.describe('Agentic Storefront Editor', () => {
  test('Maya can use the Marketing Agent to edit her storefront', async ({ page }) => {
    // Navigate to the storefront builder page
    await page.goto('/storefront-builder');

    // Wait for the page to load
    await expect(page.locator('text=Welcome to OHC Smart Builder')).toBeVisible();

    // Enter bio
    await page.fill('textarea[placeholder="e.g. I run a mobile dog grooming service in Portland"]', 'Maya the home baker, I bake custom vegan cakes.');

    // Click Generate
    await page.click('button:has-text("Build My Storefront")');

    // Wait for generation to finish and preview mode to appear
    await expect(page.locator('text=Preview Mode')).toBeVisible({ timeout: 10000 });

    // Click on "Ask Agent to Edit"
    await page.click('button:has-text("Ask Agent to Edit")');

    // Verify Marketing Agent chat opens
    await expect(page.locator('text=Marketing Agent')).toBeVisible();

    // Type a request to the agent
    await page.fill('textarea[placeholder="e.g. Add a new product..."]', 'Add a new vegan chocolate cake for $45');

    // Click send (the SVG icon button)
    await page.click('button:has-text("Marketing Agent") ~ div:last-child button');

    // Wait for generation to finish and return to preview mode
    await expect(page.locator('text=Preview Mode')).toBeVisible({ timeout: 10000 });
  });
});
