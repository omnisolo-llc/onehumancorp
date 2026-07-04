import { test, expect } from './fixtures';

test.describe('Viral Post Generator', () => {
  test('should allow owner to generate post and handle paywall', async ({ page, context }) => {
    // 1. Navigate to dashboard
    await page.goto('/dashboard');

    // 2. Find and click the Promoter Agent / Viral Post Generator link in GrowBusinessCard
    const promoterLink = page.locator('a[href="/viral-post-generator"]');
    await expect(promoterLink).toBeVisible();
    await promoterLink.click();

    // Verify page content
    await expect(page.getByRole('heading', { name: 'Promoter Agent Post Generator' })).toBeVisible();

    // Wait to ensure client-side hydration doesn't interrupt filling
    await page.waitForTimeout(500);

    // 3. Fill in details
    const productNameInput = page.getByPlaceholder('e.g. Signature Coffee Blend');
    await productNameInput.fill('Ultimate Developer Coffee');

    const keyBenefitInput = page.getByPlaceholder('e.g. a bold start to your morning');
    await keyBenefitInput.fill('maximum focus and energy');

    // Generate the post
    const generateBtn = page.getByRole('button', { name: 'Generate Post' });
    await generateBtn.click();

    // Verify output
    await expect(page.getByText(/Ultimate Developer Coffee/)).toBeVisible();
    await expect(page.getByText(/maximum focus and energy/)).toBeVisible();
    await expect(page.getByText(/Powered by OHC/)).toBeVisible();

    // 4. Try to remove branding
    const removeBrandingCheckbox = page.getByRole('checkbox', { name: /Remove "Powered by OHC" branding/i });
    await removeBrandingCheckbox.click();

    // Paywall appears
    await expect(page.getByRole('heading', { name: 'Upgrade to Pro' })).toBeVisible();

    // Close the paywall
    await page.getByRole('button', { name: 'Close paywall' }).click();

    // Ensure paywall is gone
    await expect(page.getByRole('heading', { name: 'Upgrade to Pro' })).toBeHidden();

    // 5. Copy the post
    const copyButton = page.getByRole('button', { name: 'Copy to Clipboard' });
    await expect(copyButton).toBeVisible();

    await copyButton.click();
    await expect(page.getByText('Copied!')).toBeVisible();
  });
});
