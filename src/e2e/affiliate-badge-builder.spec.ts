import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';


test.describe('Affiliate Badge Builder', () => {
  test('should generate affiliate badge HTML based on inputs', async ({ page }) => {
    const adminPageObj = await adminPage(page);
    // Navigate to the Dashboard, which contains the Growth section
    await adminPageObj.goto('/ui/dashboard.html');

    // Navigate to the Affiliate Badge Builder via the link in the Growth section
    const link = adminPageObj.locator('#affiliate-badge-link');
    await expect(link).toBeVisible();
    await link.click();

    // Verify page loads
    await expect(adminPageObj).toHaveURL(/.*affiliate-badge-builder\.html/);
    await expect(page.getByRole('heading', { name: 'Affiliate Badge Builder 💸' })).toBeVisible();

    // Verify default preview text
    const previewText = adminPageObj.locator('#badgeTextPreview');
    await expect(previewText).toHaveText('Powered by OHC');

    // Change Badge Text
    const textInput = adminPageObj.locator('#badgeText');
    await textInput.fill('Built with OHC');
    await expect(previewText).toHaveText('Built with OHC');

    // Change Theme to Dark
    const themeSelect = adminPageObj.locator('#badgeTheme');
    await themeSelect.selectOption('dark');

    // Verify the preview element has the 'dark' class
    const badgeElement = adminPageObj.locator('#badgeElement');
    await expect(badgeElement).toHaveClass(/dark/);

    // Verify the embed code contains the updated text, theme inline styles, and the referral URL
    const embedCode = adminPageObj.locator('#embedCode');
    const embedValue = await embedCode.inputValue();

    expect(embedValue).toContain('Built with OHC');
    expect(embedValue).toContain('background-color: #111827'); // Dark theme background
    expect(embedValue).toContain('api/v1/growth/referrals/click?target=/onboarding&ref=e2e-tenant&source=affiliate_badge');

    // Copy HTML Code button
    const copyBtn = adminPageObj.locator('#copyBtn');
    await expect(copyBtn).toBeVisible();

    // Set up a listener for clipboard (since Playwright context clipboard can be tricky, we check button text change)
    await copyBtn.click();
    await expect(copyBtn).toHaveText('Copied!');
  });
});
