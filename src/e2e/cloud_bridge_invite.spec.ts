import { test, expect } from './fixtures';

test.describe('Sovereign-to-Cloud Bridge Invite', () => {
  test('generates cloud bridge invite link from team page', async ({ page, context, loginAs, unlimitedAdminUser }, testInfo) => {
    expect(testInfo.project.use.baseURL).toBeTruthy();
    expect(['localhost', '127.0.0.1', '[::1]']).toContain(new URL(testInfo.project.use.baseURL!).hostname);
    await context.grantPermissions(['clipboard-read', 'clipboard-write'], { origin: new URL(testInfo.project.use.baseURL!).origin });
    // Navigate to Team page
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/team');

    // Check Growth Referral Widget is visible
    await expect(page.getByRole('heading', { name: 'Grow Your Team' })).toBeVisible();
    await expect(page.getByText('Sovereign-to-Cloud Bridge')).toBeVisible();

    // Click Invite to Cloud Team button
    const generateBtn = page.getByRole('button', { name: /Invite to Cloud Team|Unlock Cloud Collaboration/ });
    const pending = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/growth/cloud-bridge/invite' && response.request().method() === 'POST');
    await generateBtn.click();
    const response = await pending;
    expect(response.status()).toBe(200);
    const receipt = await response.json();
    expect(receipt.error).toBeUndefined();
    expect(receipt.success).not.toBe(false);

    // Verify link is generated
    const input = page.locator('#cloud-bridge-invite-link');
    await expect(input).toBeVisible({ timeout: 10000 });

    const value = await input.inputValue();
    expect(value).toContain('https://omnisolo.co/invite/inv-');
    expect(value).toBe(receipt.invite_link);

    // Verify copy button works (UI feedback)
    const copyBtn = page.getByRole('button', { name: 'Copy', exact: true });
    await page.bringToFront();
    await copyBtn.click();
    await expect(page.getByRole('button', { name: 'Copied!' })).toBeVisible();
    expect(await page.evaluate(() => navigator.clipboard.readText())).toBe(receipt.invite_link);
  });
});
