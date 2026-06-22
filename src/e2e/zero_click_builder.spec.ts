import { test, expect } from './fixtures';

test.describe('Zero Click Builder Viral Growth Loop', () => {
  test('should allow an owner to generate a store from a single prompt and see viral share option', async ({ page, request, loginAs, adminUser }) => {
    // Navigate to the new growth feature
    await loginAs(page, adminUser);

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
    await expect(page.getByText('Your business is live!')).toBeVisible({ timeout: 20000 });

    // Verify the generated preview iframe is visible
    const previewIframe = page.locator('iframe[title="Live Storefront Preview"]');
    await expect(previewIframe).toBeVisible();

    // Verify the launch button is present
    const launchBtn = page.getByRole('button', { name: /Launch My Store/i });
    await expect(launchBtn).toBeVisible();

    // Share button
    const shareBtn = page.getByRole('button', { name: /Share on X/i });
    await expect(shareBtn).toBeVisible();

    // Click the launch button to verify redirect
    await launchBtn.click();
    await expect(page).toHaveURL(/\/dashboard/);
  });
});
