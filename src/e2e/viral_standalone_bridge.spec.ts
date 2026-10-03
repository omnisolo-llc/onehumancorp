import { test, expect } from './fixtures';
import type { Request } from '@playwright/test';

test.describe('Viral Standalone Bridge', () => {
  test('creates one recorded invitation and exposes matching copy and share intents', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    const invitationRequests: Request[] = [];
    page.on('request', request => {
      if (request.method() === 'POST' && /\/api\/v1\/growth\/(?:cloud-bridge\/invite|team-invites)$/.test(new URL(request.url()).pathname)) invitationRequests.push(request);
    });
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
    await expect(generateBtn).toBeEnabled();
    const responsePromise = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/growth/cloud-bridge/invite' && response.request().method() === 'POST');
    await generateBtn.click();
    const response = await responsePromise;
    expect(response.status()).toBe(200);
    const receipt = await response.json();
    expect(receipt.error).toBeUndefined();
    expect(receipt.success).not.toBe(false);
    expect(receipt.invite_link).toMatch(/^https:\/\/(cloud\.)?omnisolo\.co\/invite\/[^/?#]+$/);
    expect(receipt.invite_link).not.toMatch(/\/(fallback|default)$/);
    expect(invitationRequests).toHaveLength(1);
    expect(invitationRequests[0].postDataJSON()).toEqual({ invitee_id: 'pending' });

    // Check generated link input and action buttons
    const linkInput = page.locator('#referral-link');
    await expect(linkInput).toBeVisible();
    await expect(linkInput).toHaveValue(receipt.invite_link);

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
    expect(invitationRequests).toHaveLength(1);

    // Reload never creates another invitation or reconstructs a receipt from local metadata.
    await page.reload();
    await expect(page.locator('#generate-link-btn')).toBeDisabled();
    await expect(page.locator('#invite-link-status')).toContainText('already created');
    await expect(page.locator('#referral-link')).toHaveValue('');
    expect(invitationRequests).toHaveLength(1);
  });
});
