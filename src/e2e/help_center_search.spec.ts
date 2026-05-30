import { test, expect } from './fixtures';

test.describe('Help Center Search & Articles', () => {
  test.skip('should allow searching for articles and viewing them', async ({ page }) => {
    test.setTimeout(30000); // 30 seconds

    // Navigate directly to the help page
    await page.goto('/help', { waitUntil: 'networkidle' });

    // Ensure the Help Center loaded
    await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible({ timeout: 15000 });

    // Verify search input is present
    const searchInput = page.getByPlaceholder('Search for help...');
    await expect(searchInput).toBeVisible();

    // Wait for the articles to load before searching
    await expect(page.getByRole('heading', { name: 'Getting Paid', exact: true })).toBeVisible();

    // Type a query that should match 'Getting Paid'
    await searchInput.fill('Paid');

    // "Getting Paid" should be visible
    await expect(page.getByRole('heading', { name: 'Getting Paid', exact: true })).toBeVisible();

    // "Getting Started" shouldn't match "Paid"
    await expect(page.getByRole('heading', { name: 'Getting Started', exact: true })).toBeHidden();

    // Click on the matching article
    await page.getByRole('heading', { name: 'Getting Paid', exact: true }).click();

    // Wait for the URL and the content to update
    await expect(page).toHaveURL(/.*\/help\/payments/);

    // Assert that the article's specific content is rendered
    await expect(page.getByRole('heading', { name: 'Getting Paid', exact: true })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Stripe Integration', exact: true })).toBeVisible();
    await expect(page.getByText('If you sell in person, use our mobile app to accept Tap to Pay directly on your phone.')).toBeVisible();
  });
});
