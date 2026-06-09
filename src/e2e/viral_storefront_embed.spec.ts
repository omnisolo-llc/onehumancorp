import { test, expect } from '@playwright/test';

test.describe('Viral Storefront Embed', () => {
  test('should generate and allow copying of the viral storefront embed code', async ({ page }) => {
    // Navigate to the Team page where the GrowthReferralWidget is used
    await page.goto('/team');

    // Wait for the "Embed Your Business" section to be visible
    await expect(page.getByRole('heading', { name: 'Embed Your Business' })).toBeVisible();

    // Verify the descriptive text is present
    await expect(page.getByText('Put your storefront anywhere. Includes a built-in referral loop to reward you when other owners join through your embed.')).toBeVisible();

    // Verify the "Copy Embed Code" button is present
    const copyButton = page.getByRole('button', { name: 'Copy Embed Code' });
    await expect(copyButton).toBeVisible();

    // Click the button and accept the alert
    page.once('dialog', async (dialog) => {
      expect(dialog.message()).toBe('Embed code copied to clipboard!');
      await dialog.accept();
    });

    // Give clipboard permissions
    await page.context().grantPermissions(['clipboard-read', 'clipboard-write']);

    await copyButton.click();

    // Verify clipboard content
    const clipboardText = await page.evaluate(() => navigator.clipboard.readText());

    // Check if the clipboard content is a valid iframe with the viral link
    expect(clipboardText).toContain('<iframe');
    expect(clipboardText).toContain('src="https://ohc.app/api/v1/growth/storefront/embed');
    expect(clipboardText).toContain('⚡ Powered by OHC');
    expect(clipboardText).toContain('href="https://ohc.app/api/v1/growth/referrals/click?target=/onboarding&ref=');
  });
});
