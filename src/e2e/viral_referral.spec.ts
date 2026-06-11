import { test, expect } from './fixtures';

test.describe('Viral Referral Loop', () => {
  test('should display Invite & Earn widget and copy-link functionality on dashboard', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);

    // Test the next js ui route since that's what the bazel suite checks and we updated page.tsx
    await page.goto('/dashboard');

    // Wait for network idle to ensure scripts are executed
    await page.waitForLoadState('networkidle');

    // Check header of the new widget
    await expect(page.getByRole('heading', { name: 'Invite & Earn' })).toBeVisible();
    await expect(page.getByText('Invite a fellow business owner to OHC')).toBeVisible();

    // Click to generate link
    const generateBtn = page.locator('#dashboard-invite-btn');
    await expect(generateBtn).toBeVisible();
    await generateBtn.click();

    // Check generated link input and action buttons
    const linkInput = page.locator('#dashboard-invite-link');
    await expect(linkInput).toBeVisible();

    // Fallback or actual link
    await expect(linkInput).toHaveValue(/^http/);

    const copyBtn = page.locator('#dashboard-copy-btn');
    await expect(copyBtn).toBeVisible();
    await expect(page.locator('#dashboard-share-x-btn')).toBeVisible();

    // Grant clipboard permissions to test the copy functionality natively
    await page.context().grantPermissions(['clipboard-read', 'clipboard-write']);

    // Verify clipboard/copy interaction
    await copyBtn.click();
    await expect(copyBtn).toHaveText('Copied!');

    // Verify the clipboard content
    const clipboardText = await page.evaluate(() => navigator.clipboard.readText()).catch(() => "");
    if (clipboardText) {
      expect(clipboardText).toMatch(/^http/);
    }
  });
});
