import { test, expect } from '@playwright/test';

test.describe('Help Center CUJ', () => {
  test('Verify help center and search', async ({ page }) => {
    // Navigate to help center
    await page.goto('http://localhost:3000/help');

    // Expect heading
    await expect(page.locator('h1')).toContainText('Help Center');

    // Check articles are loaded
    await expect(page.getByText('Getting Started')).toBeVisible();
    await expect(page.getByText('My Store')).toBeVisible();
    await expect(page.getByText('Getting Paid')).toBeVisible();

    // Perform a search
    await page.getByPlaceholder('Search for help articles...').fill('Add products');

    // Expect Getting Started to disappear, but My Store to stay
    await expect(page.getByText('Getting Started')).toBeHidden();
    await expect(page.getByText('My Store')).toBeVisible();

    // Perform a search that yields no results
    await page.getByPlaceholder('Search for help articles...').fill('Nonexistent article 123');
    await expect(page.getByText('No articles found matching "Nonexistent article 123"')).toBeVisible();

    // Clear search and navigate to an article
    await page.getByPlaceholder('Search for help articles...').fill('');
    await page.getByText('Getting Started').click();

    // Ensure we are on the article page
    await expect(page).toHaveURL(/\/help\/getting-started/);
    await expect(page.locator('h1')).toContainText('Getting Started with Your Store');
    await expect(page.getByText('Step 1: Tell us about your business')).toBeVisible();

    // Navigate back to the help center
    await page.getByText('Back to Help Center').click();
    await expect(page).toHaveURL(/\/help/);
  });

  test('Verify unknown article handles 404 gracefully', async ({ page }) => {
    await page.goto('http://localhost:3000/help/some-random-unknown-article');

    await expect(page.locator('h1')).toContainText('Article Not Found');
    await expect(page.getByText("We couldn't find the article you're looking for.")).toBeVisible();
  });
});
