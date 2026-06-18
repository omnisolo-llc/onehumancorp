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

    // The user should transition to the Unified Agent Feed directly
    await page.waitForURL('**/dashboard', { timeout: 20000 });

    // Verify that the feed is populated with an initial actionable item
    // Because we mocked this via zero_click_auto_redirect.spec.ts, we can just assert dashboard navigation here
    // as well to keep the tests robust.
    await expect(page.locator('text=Your store is ready. Review and Publish.')).toBeVisible({ timeout: 15000 });
  });
});
