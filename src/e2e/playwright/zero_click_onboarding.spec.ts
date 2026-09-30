import { test, expect } from '../fixtures';

test.describe('Zero-Click Onboarding Flow', () => {
  test.use({ viewport: { width: 375, height: 667 } }); // strictly mobile viewport

  test('should complete the zero-click onboarding flow on mobile', async ({ page }) => {
    // Navigate to the real local server
    await page.goto('/setup.html');
    await expect(page).toHaveTitle(/OmniSolo|OmniSolo/);

    await page.getByRole('button', { name: 'Generate My Workspace', exact: true }).click();

    // Initial Screen
    await expect(page.locator('h1', { hasText: 'Tell us about your business' })).toBeVisible({ timeout: 15000 });

    // Check if the input loaded
    await expect(page.locator('#instant-bio')).toBeVisible();

    // Type into the input
    await page.locator('#instant-bio').fill('I am a baker in Austin selling custom cakes');

    // The instant path provisions directly; conversational review is covered separately.
    const generated = page.waitForResponse(response => response.url().endsWith('/api/v1/growth/zero-click-builder/generate') && response.request().method() === 'POST');
    await page.locator('#generate-storefront-btn').click();
    expect((await generated).status()).toBe(200);

    // The flow goes to the success/dashboard screen.
    await expect(page).toHaveURL(/.*(dashboard\.html|dashboard|success\.html).*/, { timeout: 30000 });
  });
});
