import { test, expect } from './fixtures';

test.describe('Extended Documentation & Help Features', () => {

  test('Owner/Operator Persona: Can search Help Center and see an empty state', async ({ page }) => {
    // Navigate directly to the help portal
    await page.goto('/api/ui/help.html');
    await page.waitForLoadState('networkidle');

    // Enter a search query that yields no results
    const searchInput = page.getByPlaceholder('Search for help articles and videos...');
    await searchInput.fill('XYZNonExistent123');

    // Wait for the empty state to appear
    await expect(page.getByText('No results found matching "XYZNonExistent123"')).toBeVisible({ timeout: 10000 });
  });

  test('Owner/Operator Persona: Can launch interactive walkthrough from Help widget', async ({ page }) => {
    // Navigate to a page where the widget is loaded, e.g., the dashboard
    await page.goto('/api/ui/dashboard.html');
    await page.waitForLoadState('networkidle');

    // Find and click the walkthrough button present on the dashboard
    const walkBtn = page.locator('#dashboard-walkthrough-btn');
    await expect(walkBtn).toBeVisible();
    await walkBtn.click();

    // Verify walkthrough bubble appears showing the first step
    await expect(page.locator('.ohc-walkthrough-bubble')).toBeVisible({ timeout: 10000 });
    await expect(page.locator('.ohc-walkthrough-bubble')).toContainText('Welcome');

    // Ensure the close button works
    const closeBtn = page.locator('.ohc-walkthrough-close');
    await closeBtn.click();
    await expect(page.locator('.ohc-walkthrough-bubble')).not.toBeVisible();
  });

  test('Advanced Persona: Can load Swagger UI in API Documentation page', async ({ page }) => {
    await page.goto('/api/ui/api-docs.html');
    await page.waitForLoadState('networkidle');

    // Check for advanced badge
    await expect(page.getByText('Advanced:')).toBeVisible();

    // Check that Swagger UI rendered the primary container and title
    await expect(page.locator('.swagger-ui')).toBeVisible({ timeout: 10000 });
    await expect(page.getByText('OHC Advanced API Reference')).toBeVisible();
  });

});
