import { test, expect } from './fixtures';

test.describe('Zero Click Builder Viral Growth Loop', () => {
  test('should allow an owner to generate a store from a single prompt and see viral share option', async ({ page, request, loginAs, adminUser }) => {
    // Navigate to the new growth feature
    await loginAs(page, adminUser);


    await page.goto('/onboarding/zero-click');
    await page.locator('input[placeholder*="baker"]').click();


    // Verify mobile-first layout
    await page.setViewportSize({ width: 375, height: 812 });

    // Verify title
    await expect(page.locator('h1', { hasText: 'Tell us about your business' })).toBeVisible({ timeout: 15000 });

    // Verify "Powered by OHC" branding is present (viral loop)
    await expect(page.locator('#dashboard-footer-viral-link')).toBeVisible();

    // The generate button should be disabled initially
    const generateBtn = page.getByTestId('generate-storefront-btn');
    await expect(generateBtn).toBeDisabled();

    // Fill in the prompt
    await page.fill('#instant-bio', 'I am a local coffee roaster in Seattle needing a storefront.');

    // The button should now be enabled
    await expect(generateBtn).toBeEnabled();

    // Submit the form
    await generateBtn.click();

    // Wait for the loading state to complete and the result to appear
    await expect(page.locator('h2', { hasText: 'Your business is live!' })).toBeVisible({ timeout: 15000 });

    // Launch Store (acts as "Approve & Go Live")
    await page.getByText('🚀 Launch My Store').click();

    // Check navigation to dashboard
    await expect(page).toHaveURL(/.*dashboard/, { timeout: 15000 });
  });
});