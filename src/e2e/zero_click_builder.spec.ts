import { test, expect } from './fixtures';

test.describe('Zero Click Builder Viral Growth Loop', () => {
  test('should allow an owner to generate a store from a single prompt and see viral share option', async ({ page, request, loginAs, adminUser }) => {
    // Navigate to the new growth feature
    await loginAs(page, adminUser);

    // We stub the network route here so the E2E test doesn't actually hit the LLM provider
    await page.route('**/api/v1/growth/zero-click-builder/generate', async route => {
      const json = {
        name: 'Seattle Roasters',
        url: 'https://seattle-roasters.ohc.app',
        products_count: 8,
      };
      await route.fulfill({ json });
    });

    await page.goto('/zero-click-builder');

    // Verify mobile-first layout
    await page.setViewportSize({ width: 375, height: 812 });

    // Verify title
    await expect(page.locator('h1', { hasText: 'Zero-Click Business Generator' })).toBeVisible({ timeout: 15000 });

    // Verify "Powered by OHC" branding is present (viral loop)
    await expect(page.getByText('⚡ Powered by OHC')).toBeVisible();

    // The generate button should be disabled initially
    const generateBtn = page.getByRole('button', { name: /Generate My Business/i });
    await expect(generateBtn).toBeDisabled();

    // Fill in the prompt
    await page.fill('textarea[id="prompt"]', 'I am a local coffee roaster in Seattle needing a storefront.');

    // The button should now be enabled
    await expect(generateBtn).toBeEnabled();

    // Submit the form
    await generateBtn.click();

    // Wait for the loading state to complete and the result to appear
    await expect(page.getByText('Your business is live!')).toBeVisible({ timeout: 10000 });

    // Verify the generated content is displayed
    await expect(page.getByText('Store URL')).toBeVisible();
    await expect(page.getByText('Seattle Roasters')).toBeVisible();

    // Verify the viral share buttons are present
    const shareBtn = page.getByRole('button', { name: /Share to Twitter/i });
    await expect(shareBtn).toBeVisible();

    const goToDashboardBtn = page.getByRole('button', { name: /Go to Dashboard/i });
    await expect(goToDashboardBtn).toBeVisible();
  });
});
