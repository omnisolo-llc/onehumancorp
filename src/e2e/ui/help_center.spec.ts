import { test, expect } from '../fixtures';

test.describe('Help Center & Documentation Features', () => {
  // Test owner persona: Maya - Home Baker using the app to find help
  test('Owner can navigate Help Center, use search, and play a video tutorial', async ({ page }) => {

    // 1. Owner opens Help Center from navigation or direct URL
    await page.goto('/api/ui/help.html');

    // 2. Help Center Page is loaded
    await expect(page.locator('h1:has-text("Help Center")')).toBeVisible();

    // 3. Search for a specific topic
    const searchInput = page.locator('input[placeholder="Search for help articles and videos..."]');
    await searchInput.fill('store in 5 minutes');
    await page.waitForTimeout(500); // Wait for debounce or search update

    // 4. Verify search results contain video tutorial
    await expect(page.locator('text=How to set up your store in 5 minutes')).toBeVisible();

    // 5. Open video tutorial modal
    await page.locator('text=How to set up your store in 5 minutes').click();

    // 6. Verify video modal opens and can be closed
    const closeButton = page.locator('button[aria-label="Close video"]');
    await expect(closeButton.first()).toBeVisible();
    await closeButton.first().evaluate((b) => (b as HTMLElement).click());
    await expect(closeButton.first()).not.toBeVisible();
  });

  test('Owner can access API docs and see Advanced user tooltips', async ({ page }) => {

    // 1. Go to Help Center
    await page.goto('/api/ui/help.html');

    // 2. Click the API Documentation link in Advanced section
    const apiLink = page.locator('a:has-text("API Documentation")');
    await expect(apiLink).toBeVisible();

    // 3. Navigate to API Docs
    await apiLink.evaluate((b) => (b as HTMLElement).click());
    await expect(page).toHaveURL(/\/api-docs\.html/);

    // 4. Hover to see tooltip
    const tooltipTarget = page.locator('#api-docs-tooltip');
    await expect(tooltipTarget).toBeVisible();
    await tooltipTarget.hover({ force: true });
    await expect(page.locator('text=Direct API access is only for custom integrations.')).toBeVisible();

    // 5. Verify API docs loaded (Swagger UI)
    await expect(page.locator('text=Advanced:')).toBeVisible();
    await expect(page.locator('.swagger-ui')).toBeVisible();
  });



});
