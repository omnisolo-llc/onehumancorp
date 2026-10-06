import { test, expect } from './fixtures';

test.describe('Extended Documentation & Help Features', () => {

  test('Owner/Operator Persona: Can search Help Center and see an empty state', async ({ page }) => {
    // Navigate directly to the help portal
    await page.goto('/api/v1/ui/help.html');
    await page.waitForLoadState('networkidle');

    // Enter a search query that yields no results
    const searchInput = page.getByPlaceholder('Search for help articles and videos...');
    await searchInput.fill('XYZNonExistent123');

    // Wait for the empty state to appear
    await expect(page.getByText('No results found matching "XYZNonExistent123"')).toBeVisible({ timeout: 10000 });
  });

  test('Owner/Operator Persona: Can launch interactive walkthrough from Help widget', async ({ page }) => {
    // Navigate to a page where the widget is loaded, e.g., the dashboard
    await page.goto('/api/v1/ui/dashboard.html');
    await page.waitForLoadState('networkidle');

    // Find and click the walkthrough button present on the dashboard
    const walkBtn = page.locator('#dashboard-walkthrough-btn');
    await expect(walkBtn).toBeVisible();
    await walkBtn.click();

    // Verify walkthrough bubble appears showing the first step
    await expect(page.locator('.omnisolo-walkthrough-bubble')).toBeVisible({ timeout: 10000 });
    await expect(page.locator('.omnisolo-walkthrough-bubble')).toContainText('Welcome');

    // Ensure the close button works
    const closeBtn = page.locator('.omnisolo-walkthrough-bubble').getByRole('button', { name: 'Close walkthrough', exact: true });
    await closeBtn.click();
    await expect(page.locator('.omnisolo-walkthrough-bubble')).toBeHidden();
    await walkBtn.click();
    await expect(page.locator('.omnisolo-walkthrough-bubble')).toContainText('Welcome');
    await closeBtn.click();
    await expect(page.locator('.omnisolo-walkthrough-bubble')).toBeHidden();
  });

  test('Advanced Persona: Can load Swagger UI in API Documentation page', async ({ page }) => {
    await page.goto('/api/v1/ui/api-docs.html');
    await page.waitForLoadState('networkidle');

    // Check for advanced badge
    await expect(page.getByText('Advanced:')).toBeVisible();

    // Check that Swagger UI rendered the primary container and title
    const viewer = page.frameLocator('iframe[data-ohc-api-docs-viewer]');
    await expect(viewer.locator('.swagger-ui').first()).toBeVisible({ timeout: 10000 });
    await expect(viewer.getByText('OmniSolo Advanced API Reference').first()).toBeVisible();
  });

});
