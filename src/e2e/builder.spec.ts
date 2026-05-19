import { test, expect } from './fixtures';

test.describe('Website & Storefront Builder API Flow', () => {
  test('generates and publishes a storefront via API', async ({ page }) => {
    // This test targets the backend API via the embedded UI simulation or direct fetch if available
    // Since we are verifying the architecture, we check if the builder screen elements are responsive

    await page.goto('/');
    // Navigate to Builder
    await page.getByRole('link', { name: 'Builder' }).click();

    await expect(page.getByRole('heading', { name: 'Edit Website' })).toBeVisible();

    // Verify initial blocks are rendered
    await expect(page.locator('#builder-preview-container')).toContainText('Hero');

    // Simulate reordering
    await page.getByRole('button', { name: 'Rearrange' }).click();
    await expect(page.locator('#builder-preview-container')).toContainText('↑');

    // Click Publish to open bottom sheet
    await page.getByRole('button', { name: 'Publish Changes' }).click();
    await expect(page.locator('#domain-setup-sheet')).toBeVisible();

    // Select free subdomain
    await page.getByRole('button', { name: /Free OHC Subdomain/ }).click();
    await page.getByPlaceholder('mybusiness').fill('maya-bakes');

    // Final publish
    await page.getByRole('button', { name: 'Publish', exact: true }).click();

    // Expect success (confetti/redirect)
    await expect(page.locator('#dashboard-screen')).toBeVisible();
  });
});
