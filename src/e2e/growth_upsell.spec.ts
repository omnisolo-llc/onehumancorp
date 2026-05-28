import { test, expect } from './fixtures';

test.describe('Growth Feature: AI Upsell Recommendations', () => {
  test('displays AI Upsell section on the dashboard and links to the dedicated page', async ({ page }) => {
    // Navigate to the dashboard
    await page.goto('/dashboard');

    // Check if the AI Upsell section is present
    await expect(page.getByRole('heading', { name: 'AI Upsell Recommendations' })).toBeVisible();

    // Click the CTA link
    await page.getByRole('link', { name: 'Generate Upsell Strategy' }).click();

    // Ensure we are navigated to the correct page
    await expect(page).toHaveURL(/\/ai-upsell/);
  });

  test('generates upsell strategy and triggers soft paywall if not pro', async ({ page }) => {
    // Navigate directly to the ai-upsell page
    await page.goto('/ai-upsell');

    // Check main title
    await expect(page.getByRole('heading', { name: 'AI Upsell Recommendations' })).toBeVisible();

    // Ensure the Generate button is disabled initially
    const generateBtn = page.getByRole('button', { name: 'Generate Upsell Items' });
    await expect(generateBtn).toBeDisabled();

    // Fill the product name
    await page.getByPlaceholder('e.g. Signature Coffee Blend').fill('Coffee Mug');

    // Ensure the Generate button is enabled after input
    await expect(generateBtn).toBeEnabled();

    // Click generate - since it's an E2E test without has_pro in local storage, this should trigger the paywall
    await generateBtn.click();

    // Wait for the Soft Paywall modal to appear
    await expect(page.getByRole('heading', { name: 'Upgrade to Pro' })).toBeVisible();

    // Close the soft paywall using the X button
    await page.getByRole('button', { name: '×' }).click();
    await expect(page.getByRole('heading', { name: 'Upgrade to Pro' })).not.toBeVisible();
  });
});
