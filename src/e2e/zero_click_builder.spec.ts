import { test, expect } from './fixtures';

test.describe('Zero Click Builder Viral Growth Loop', () => {
  test('should allow an owner to generate a store from a single prompt and see viral share option', async ({ page, request, loginAs, adminUser }) => {
    await loginAs(page, adminUser);

    await page.goto('/zero-click-builder.html');

    // Verify mobile-first layout
    await page.setViewportSize({ width: 375, height: 812 });

    // Verify title
    await expect(page.locator('h1', { hasText: 'Zero-Click Business Generator' })).toBeVisible({ timeout: 15000 });

    // The generate button should be disabled initially
    const generateBtn = page.getByRole('button', { name: 'Generate Store' });
    await expect(generateBtn).toBeDisabled();

    // Fill in the prompt
    await page.fill('#prompt', 'I am a home baker in Austin selling custom vegan cakes and cupcakes.');

    // The button should now be enabled
    await expect(generateBtn).toBeEnabled();

    // Submit the form
    await generateBtn.click();

    // Wait for the loading state to complete and the result to appear
    await expect(page.locator('h2', { hasText: 'Your business is live!' })).toBeVisible({ timeout: 15000 });

    // Verify "Powered by OHC" branding is present (viral loop)
    await expect(page.locator('#dashboard-footer-viral-link')).toBeVisible();

    // Launch Store (acts as "Approve & Go Live")
    await page.getByRole('button', { name: '🚀 Launch My Store' }).click();

    // Check navigation to dashboard
    await expect(page).toHaveURL(/.*dashboard.html/, { timeout: 15000 });
  });
});
