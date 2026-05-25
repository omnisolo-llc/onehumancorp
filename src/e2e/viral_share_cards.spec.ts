import { test, expect } from './fixtures';

test.describe('Viral Social Share Cards E2E', () => {
  test('exposes share cards generator and verifies functionality', async ({ page }) => {
    await page.goto('/share-cards');
    await expect(page.getByRole('heading', { name: 'Social Share Cards 🎴' })).toBeVisible();

    // Verify inputs
    await page.getByLabel('Store Name').fill('My Awesome Test Store');
    await page.getByLabel('Tagline').fill('A great tagline here.');

    // Verify preview updates
    // The h1 inside the preview has text "My Awesome Test Store"
    await expect(page.locator('h1', { hasText: 'My Awesome Test Store' })).toBeVisible();

    // Test copy logic
    await page.getByRole('button', { name: 'Copy Link' }).click();
    await expect(page.getByText('Copied Link!')).toBeVisible();
  });
});
