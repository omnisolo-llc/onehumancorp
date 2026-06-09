import { test, expect } from './fixtures';

test.describe('Omnibox Global Search', () => {
  test('should open on Cmd+K, type a query, and display results', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');

    // Wait for the app shell to be ready
    await expect(page.getByRole('navigation', { name: 'Primary' })).toBeVisible({ timeout: 15000 });

    // Press Cmd+K (Mac) or Ctrl+K (Windows/Linux)
    const modifier = process.platform === 'darwin' ? 'Meta' : 'Control';
    await page.keyboard.press(`${modifier}+KeyK`);

    // Verify the omnibox input is visible
    const omniboxInput = page.getByPlaceholder('Search customers, orders, messages... (Cmd+K)');
    await expect(omniboxInput).toBeVisible();

    // Type a query
    await omniboxInput.fill('John');

    // Wait for the search results or empty state to appear
    // We'll check for the placeholder indicating search finished or results exist
    // Let's use a dynamic locator that waits for the network response implicitly
    // Since there might be no "John" in the seeded data, we check for either an item or the "No results" message.
    await expect(page.locator('text=No results found for "John"').or(page.getByTestId('omnibox-result').first())).toBeVisible({ timeout: 10000 });
  });

  test('should dismiss when pressing Escape', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('navigation', { name: 'Primary' })).toBeVisible({ timeout: 15000 });

    const modifier = process.platform === 'darwin' ? 'Meta' : 'Control';
    await page.keyboard.press(`${modifier}+KeyK`);

    const omniboxInput = page.getByPlaceholder('Search customers, orders, messages... (Cmd+K)');
    await expect(omniboxInput).toBeVisible();

    await page.keyboard.press('Escape');
    await expect(omniboxInput).toBeHidden();
  });
});
