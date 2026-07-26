import { test, expect } from './fixtures';

test.describe("Workspace Search Validation", () => {
  test('Search Workspace functionality works and verifies expected elements', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);

    // Wait for the dashboard to load
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    // Mock API to return a predictable set of results for search query
    await page.route('/api/v1/search?q=john', async (route) => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          success: true,
          results: [
            {
              id: 'c1',
              entity_type: 'customer',
              title: 'John Doe',
              subtitle: 'john@example.com',
              route: '/customers/c1'
            },
            {
              id: 'm1',
              entity_type: 'message',
              title: 'Message via email',
              subtitle: 'Hello John, ...',
              route: '/inbox/m1'
            }
          ]
        })
      });
    });

    // We do not have a hardcoded DOM layout for the search bar, but standard dashboard typically has a search input.
    // As per exhaustive UI verification rules, we interact with the real frontend search.
    const searchInput = page.locator('input[type="search"], input[placeholder*="Search"]').first();
    await expect(searchInput).toBeVisible();
    await searchInput.fill('john');
    await searchInput.press('Enter');

    // Verify search results UI
    const searchResultsContainer = page.locator('.search-results, [data-testid="search-results"]');
    // If there is no specific container, we can verify the text directly.
    await expect(page.locator('text=John Doe').first()).toBeVisible({ timeout: 15000 });
    await expect(page.locator('text=john@example.com').first()).toBeVisible();

    // Test the clicking functionality
    const customerResult = page.locator('text=John Doe').first();
    await customerResult.click();

    // The URL should change to the route specified
    await expect(page).toHaveURL(/.*\/customers\/c1/);
  });
});
