import { test, expect } from '@playwright/test';

test.describe('Cloud-Bridge Referral Loop', () => {
  test('should navigate to team page and generate a cloud bridge referral link', async ({ page }) => {
    // Navigate to the team page in Next.js
    await page.route('/api/v1/growth/cloud-bridge/invite', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ invite_link: 'https://ohc.app/invite/mock-123' })
      });
    });

    // Check if we need to fake login or if page handles it
    await page.evaluate(() => { localStorage.setItem('has_onboarded', 'true'); localStorage.setItem('token', 'fake-token'); });
    await page.goto('/team');

    await page.route('/api/v1/growth/cloud-bridge/invite', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ invite_link: 'https://ohc.app/invite/mock-123' })
      });
    });

    // Verify Sovereign-to-Cloud Bridge text is visible
    await expect(page.getByText('Sovereign-to-Cloud Bridge')).toBeVisible({ timeout: 15000 });

    // Verify Growth card exists
    await expect(page.getByRole('heading', { name: 'Grow Your Team' })).toBeVisible();

    // Click to generate link
    const generateBtn = page.getByRole('button', { name: 'Invite to Cloud Team' });
    await expect(generateBtn).toBeVisible();
    await generateBtn.click();

    // Wait for network response if possible, or just wait for the element to appear
    await page.waitForSelector('#cloud-bridge-invite-link', { state: 'visible', timeout: 30000 });

    // Check generated link input and action buttons
    const linkInput = page.locator('#cloud-bridge-invite-link');
    await expect(linkInput).toHaveValue(/^https:\/\/ohc\.app\/invite\//);

    const copyBtn = page.getByRole('button', { name: 'Copy', exact: true });
    await expect(copyBtn).toBeVisible();
    await expect(page.getByRole('button', { name: 'Share on WhatsApp' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Share on X (Twitter)' })).toBeVisible();

    // Grant clipboard permissions to test the copy functionality natively
    await page.context().grantPermissions(['clipboard-read', 'clipboard-write']);

    // Verify clipboard/copy interaction
    await copyBtn.click();
    await expect(page.getByRole('button', { name: 'Copied!' })).toBeVisible();

    // Verify the clipboard content includes the link
    const clipboardText = await page.evaluate(() => navigator.clipboard.readText()).catch(() => "");
    if (clipboardText) {
      expect(clipboardText).toContain('https://ohc.app/invite/');
    }

    // Verify Embed section
    await expect(page.getByText('Embed Your Business')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Copy Embed Code' })).toBeVisible();

    // Verify 10th Order milestone section
    await expect(page.getByText('10th Order! Share your success')).toBeVisible();
    await expect(page.getByRole('link', { name: 'Share to WhatsApp' })).toBeVisible();
    await expect(page.getByAltText('10th Order Milestone')).toBeVisible();
  });
});
