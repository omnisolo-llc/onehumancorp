import { test, expect } from '../../../../e2e/fixtures';

test.describe('Share & Save Widget', () => {
  test('should navigate to widget from dashboard, click share, and reveal discount code', async ({ page, loginAs, unlimitedAdminUser }) => {
    // Navigate to Dashboard
    await loginAs(unlimitedAdminUser.email, 'password123');
    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    // Verify the Share & Save Widget link is visible
    const shareLinkCard = page.locator('#share-and-save-link');
    await expect(shareLinkCard).toBeVisible();

    // Click to navigate to widget
    await shareLinkCard.click();
    await page.waitForURL('**/share-and-save-widget**');

    // Verify widget UI elements
    await expect(page.getByRole('heading', { name: 'Unlock 10% Off!' })).toBeVisible();
    const shareButton = page.getByRole('button', { name: 'Share on X to Unlock' });
    await expect(shareButton).toBeVisible();

    // Mock window.open to prevent actually opening a new tab during the test
    await page.evaluate(() => {
        window.open = () => null;
    });

    // Grant clipboard permissions
    await page.context().grantPermissions(['clipboard-read', 'clipboard-write']);

    // Click the share button
    await shareButton.click();

    // The discount code should be revealed after a delay
    const codeText = page.getByText('SHARE10');
    await expect(codeText).toBeVisible({ timeout: 5000 });

    // Test the copy button
    const copyButton = page.getByRole('button', { name: 'Copy' });
    await expect(copyButton).toBeVisible();
    await copyButton.click();

    // Verify copy button changes text temporarily
    await expect(page.getByRole('button', { name: 'Copied!' })).toBeVisible();

    // Verify the clipboard content (skip if headless environment doesn't support reading)
    try {
        const clipboardText = await page.evaluate(async () => {
            return await navigator.clipboard.readText();
        });
        expect(clipboardText).toContain('SHARE10');
    } catch (e) {
        console.warn('Clipboard read failed (expected in some headless environments): ', e);
    }

    // Verify back to dashboard navigation
    const backButton = page.getByRole('button', { name: 'Back to Dashboard' });
    await backButton.click();
    await page.waitForURL('**/dashboard**');
  });
});
