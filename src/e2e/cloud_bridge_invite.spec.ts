import { test, expect } from './fixtures';

test.describe('Sovereign-to-Cloud Bridge Invite', () => {
  test('generates cloud bridge invite link from team page', async ({ page }) => {
    // Navigate to Team page
    await page.goto('/team');

    // Check Growth Referral Widget is visible
    await expect(page.getByRole('heading', { name: 'Grow Your Team' })).toBeVisible();
    await expect(page.getByText('Sovereign-to-Cloud Bridge')).toBeVisible();

    // Click Invite to Cloud Team button
    const generateBtn = page.getByRole('button', { name: 'Invite to Cloud Team' });
    await generateBtn.click();

    // Verify link is generated
    const input = page.locator('#cloud-bridge-invite-link');
    await expect(input).toBeVisible({ timeout: 10000 });

    const value = await input.inputValue();
    expect(value).toContain('https://ohc.app/invite/inv-');

    // Verify copy button works (UI feedback)
    const copyBtn = page.getByRole('button', { name: 'Copy' });
    await copyBtn.click();
    await expect(page.getByRole('button', { name: 'Copied!' })).toBeVisible();
  });
});
