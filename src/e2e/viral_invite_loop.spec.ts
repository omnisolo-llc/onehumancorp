import { test, expect } from './fixtures';

test.describe('Viral Invite Loop on Team Page', () => {
  test('should display GrowthReferralWidget and generate a link', async ({ page, context, loginAs, unlimitedAdminUser }, testInfo) => {
    expect(testInfo.project.use.baseURL).toBeTruthy();
    expect(['localhost', '127.0.0.1', '[::1]']).toContain(new URL(testInfo.project.use.baseURL!).hostname);
    await context.grantPermissions(['clipboard-read', 'clipboard-write'], { origin: new URL(testInfo.project.use.baseURL!).origin });
    // Login
    await loginAs(page, unlimitedAdminUser);

    await page.goto('/team');

    // Wait for the UI to be ready
    await page.waitForLoadState('networkidle');

    // Check if the growth component is visible
    await expect(page.getByRole('heading', { name: 'Grow Your Team' })).toBeVisible();
    await expect(page.getByText('Create an invitation link for your verified account. Creating a link does not send it or confirm that anyone has joined.')).toBeVisible();

    // Click the invite button
    const pending = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/growth/cloud-bridge/invite' && response.request().method() === 'POST');
    await page.getByRole('button', { name: 'Unlock Cloud Collaboration' }).click();
    const response = await pending;
    expect(response.status()).toBe(200);
    const receipt = await response.json();
    expect(receipt.error).toBeUndefined();
    expect(receipt.success).not.toBe(false);

    // Wait for the link to be generated (input appears)
    const linkInput = page.locator('#cloud-bridge-invite-link');
    await expect(linkInput).toBeVisible();
    await expect(linkInput).toHaveValue(/^https:\/\/(cloud\.)?omnisolo\.co\/invite\/.+/);

    await expect(linkInput).toHaveValue(receipt.invite_link);

    // Test the native clipboard operation with its test-context permissions.
    await page.bringToFront();
    // Test copy button interaction
    await page.getByRole('button', { name: 'Copy' }).first().click();
    await expect(page.getByRole('button', { name: 'Copied!' })).toBeVisible();
    expect(await page.evaluate(() => navigator.clipboard.readText())).toBe(receipt.invite_link);

    // Verify share on WhatsApp is available
    await expect(page.getByRole('button', { name: 'Share on WhatsApp' })).toBeVisible();

    // Verify share on X (Twitter) is available
    await expect(page.getByRole('button', { name: 'Share on X (Twitter)' })).toBeVisible();
  });
});
