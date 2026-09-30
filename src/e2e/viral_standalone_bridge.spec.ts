import { test, expect } from '@playwright/test';

test.describe('Viral Standalone Bridge', () => {
  test('should navigate to dashboard and generate a referral link', async ({ page }) => {
    // Navigate to the success.html page being served by tauri
    await page.goto('/success.html');

    // Verify we are on success page
    await expect(page.getByRole('heading', { name: /You're (all set!|Live!)/i })).toBeVisible();

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
    await expect(linkInput).toHaveValue(/^https:\/\/(cloud\.)?omnisolo(\.network|\.co)\/invite\//);

    const copyBtn = page.getByRole('button', { name: 'Copy', exact: true });
    await expect(copyBtn).toBeVisible();
    await expect(page.getByRole('button', { name: 'Share on WhatsApp' })).toBeVisible();

    // Grant clipboard permissions to test the copy functionality natively
    await page.context().grantPermissions(['clipboard-read', 'clipboard-write']);

    // Verify clipboard/copy interaction
    await copyBtn.click();
    await expect(page.getByRole('button', { name: 'Copied!' })).toBeVisible();

    const inviteLink = await linkInput.inputValue();
    const expectedShareText = `Join my team on OmniSolo OneHumanCorp! Here is your invite link:\n\n${inviteLink}\n\n⚡ OmniSolo`;
    const clipboardText = await page.evaluate(() => navigator.clipboard.readText());
    expect(clipboardText).toBe(expectedShareText);

    // Verify WhatsApp Share opens new tab with the correct URL
    const whatsappBtn = page.getByRole('button', { name: 'Share on WhatsApp' });
    const shareRequest = page.context().waitForEvent('request', request =>
      new URL(request.url()).hostname === 'wa.me');
    const [popup, request] = await Promise.all([
      page.waitForEvent('popup'),
      shareRequest,
      whatsappBtn.click()
    ]);

    // Validate the app's original share intent before WhatsApp rewrites its URL.
    const shareUrl = new URL(request.url());
    expect(shareUrl.protocol).toBe('https:');
    expect(shareUrl.searchParams.get('text')).toBe(expectedShareText);
    await popup.close();
  });
});
