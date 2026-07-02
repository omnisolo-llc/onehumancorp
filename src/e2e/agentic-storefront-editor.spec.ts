import { test, expect } from './fixtures';

test.describe('Agentic Storefront Editor', () => {
  test('Maya can use the Marketing Agent to edit her storefront', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);

    // Navigate to the storefront builder page
    await page.goto('/storefront-builder');

    // Wait for the page to load
    await expect(page.locator('text=Welcome to OHC Smart Builder')).toBeVisible();

    // Enter bio
    await page.fill('textarea[placeholder="e.g. I manage 15 long-term apartment rentals"]', 'I manage 15 long-term apartment rentals');
    await page.click('button:has-text("Build My Storefront")');

    // Wait for generation to finish and preview mode to appear
    await expect(page.locator('text=Preview Mode')).toBeVisible({ timeout: 15000 });

    // Click on "Ask Agent to Edit"
    await page.click('button:has-text("Ask Agent to Edit")');

    // Verify Marketing Agent chat opens
    await expect(page.locator('text=Marketing Agent')).toBeVisible();

    // Type a request to the agent
    await page.fill('textarea[placeholder="e.g. Add a new product..."]', 'Add a new vegan chocolate cake for $45');

    // Click send (the SVG icon button)
    await page.locator('button:has-text("Marketing Agent") ~ div:last-child button').click();

    // Wait for generation to finish and return to preview mode
    await expect(page.locator('text=Preview Mode')).toBeVisible({ timeout: 15000 });
  });
});
