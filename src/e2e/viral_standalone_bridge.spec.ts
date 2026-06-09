import { test, expect } from '@playwright/test';

test.describe('Viral Standalone Bridge', () => {
  test('should navigate to dashboard and generate a referral link', async ({ page }) => {
    // Navigate to the success.html page being served by tauri
    await page.goto('/success.html');

    // Verify we are on success page
    await expect(page.getByRole('heading', { name: "You're all set!" })).toBeVisible();

    // Click Go to Dashboard
    await page.getByRole('button', { name: 'Go to Dashboard' }).click();

    // Wait for navigation
    // We should be on dashboard.html
await expect(page).toHaveURL(/.*dashboard(\.html)?/);

    // Verify standalone mode badge
    await expect(page.getByText('Standalone Mode (Zero Data Leakage)')).toBeVisible();

    // Verify Growth card
    await expect(page.getByRole('heading', { name: 'Grow Your Team' })).toBeVisible();

    // Click to generate link
    const generateBtn = page.getByRole('button', { name: 'Get My Invite Link' });
    await expect(generateBtn).toBeVisible();
    await generateBtn.click();

    // Check generated link input and action buttons
    const linkInput = page.locator('#referral-link');
    await expect(linkInput).toBeVisible();
    await expect(linkInput).toHaveValue(/^https:\/\/cloud\.ohc\.network\/invite\//);

    const copyBtn = page.getByRole('button', { name: 'Copy', exact: true });
    await expect(copyBtn).toBeVisible();
    await expect(page.getByRole('button', { name: 'Share on WhatsApp' })).toBeVisible();

    // Verify clipboard/copy interaction (just visually here as clipboard API needs permissions in some contexts)
    await copyBtn.click();
    await expect(page.getByRole('button', { name: 'Copied!' })).toBeVisible();

    // Verify WhatsApp Share opens new tab with the correct URL
    const whatsappBtn = page.getByRole('button', { name: 'Share on WhatsApp' });
    const [popup] = await Promise.all([
      page.waitForEvent('popup'),
      whatsappBtn.click()
    ]);

    // Check URL contains wa.me and the encoded viral loop text
    const popupUrl = popup.url();
    // wa.me gets expanded to api.whatsapp.com by the browser often
    expect(popupUrl).toMatch(/wa\.me|api\.whatsapp\.com/);
    expect(popupUrl).toContain('Powered+by+OHC');
    expect(popupUrl).toContain(encodeURIComponent('https://cloud.ohc.network/invite/'));
  });
});
