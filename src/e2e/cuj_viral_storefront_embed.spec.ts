import { test, expect } from './fixtures';

test.describe('Viral Storefront Embed E2E User Journey', () => {
  test('Owner navigates to dashboard, opens embed modal, and views widget', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');

    // Check that Embed Storefront growth loop section is present
    await expect(page.getByRole('heading', { name: 'Embed Your Store' })).toBeVisible();

    // Click on "Get Widget" to open the modal
    await page.getByRole('button', { name: 'Get Widget' }).click();

    // Verify the modal is open
    await expect(page.getByRole('heading', { name: 'Embed Storefront' })).toBeVisible();

    // Verify the widget code contains the correct iframe URL pointing to /api/v1/growth/storefront/embed
    await expect(page.locator('textarea')).toContainText('/api/v1/growth/storefront/embed');

    // Close the modal
    await page.locator('.bg-white.w-full.max-w-md button').first().click();
  });
});
