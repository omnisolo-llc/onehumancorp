import { test, expect } from '@playwright/test';

test.describe('Help Center & Documentation', () => {
  test('should display and search help articles', async ({ page }) => {
    await page.goto('/help');
    await expect(page.locator('h1').first()).toContainText('Help Center');

    // Check if sections like 'Getting Started' load
    await expect(page.locator('text=Getting Started').first()).toBeVisible();

    // Fill in the search query and wait for filtering
    const searchInput = page.locator('input[placeholder="Search for help articles and videos..."]');
    await searchInput.fill('Adding Products');

    // Wait for the debounced search to filter out "Getting Started"
    await expect(page.locator('text=Getting Started')).toBeHidden();

    // Ensure the searched article appears
    const articleLink = page.locator('a:has-text("Adding Products")');
    await expect(articleLink).toBeVisible();

    // Click to navigate to the article details
    await articleLink.click();
    await expect(page).toHaveURL(/\/help\/my-store/);

    // Verify article detail page
    await expect(page.locator('h1')).toContainText('Managing My Store');
  });

  test('should display video tutorials', async ({ page }) => {
    await page.goto('/help');

    // Look for video tutorials
    await expect(page.locator('h2:has-text("Video Tutorials")')).toBeVisible();
    await expect(page.locator('text="How to set up your first store easily"')).toBeVisible();
  });

  test('should link to API Documentation', async ({ page }) => {
    await page.goto('/help');

    // Check advanced section
    const apiLink = page.locator('a[href="/api-docs"]');
    await expect(apiLink).toBeVisible();

    // Navigate to API docs
    await apiLink.click();
    await expect(page).toHaveURL(/\/api-docs/);

    // Wait for Swagger UI to load
    await expect(page.locator('.swagger-ui')).toBeVisible();
    await expect(page.locator('text=OHC Advanced API Reference')).toBeVisible();
  });

  test('should navigate to Changelog', async ({ page }) => {
    await page.goto('/changelog');
    await expect(page.locator('h1')).toContainText('Release Notes & Changelog');

    // Check that at least one version shows up
    await expect(page.locator('h2')).first().toBeVisible();
  });

  test('should display the AI help chat button', async ({ page }) => {
    await page.goto('/help');

    // Expect the floating Ask Anything button
    const chatBtn = page.locator('button[aria-label="Open help chat"]');
    await expect(chatBtn).toBeVisible();

    // Open chat
    await chatBtn.click();

    // Check interface is open
    await expect(page.locator('#ai-chat-interface')).toBeVisible();
    await expect(page.locator('text=Ask AI Help')).toBeVisible();

    // Close chat
    const closeBtn = page.locator('button[aria-label="Close help chat"]');
    await closeBtn.click();
    await expect(page.locator('#ai-chat-interface')).toBeHidden();
  });
});
